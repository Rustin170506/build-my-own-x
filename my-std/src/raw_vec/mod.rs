use std::{
    alloc::{handle_alloc_error, Allocator, Global, Layout, LayoutError},
    cmp,
    collections::{TryReserveError, TryReserveErrorKind},
    hint,
    mem::{self, SizedTypeProperties},
    ptr::NonNull,
};

enum AllocInit {
    /// The contents of the new memory are uninitialized.
    Uninitialized,
    /// The new memory is guaranteed to be zeroed.
    Zeroed,
}

struct Cap(usize);

impl Cap {
    const ZERO: Self = Self(0);
}

/// A low-level utility for more ergonomically allocating, reallocating, and deallocating
/// a buffer of memory on the heap without having to worry about all the corner cases involved.
pub struct RawVec<T, A: Allocator = Global> {
    ptr: NonNull<T>,
    cap: Cap,
    alloc: A,
}

impl<T> RawVec<T, Global> {
    pub const NEW: Self = Self::new();

    /// Creates the biggest possible `RawVec` (on the system heap)
    /// without allocating. If `T` has positive size, then this makes a
    /// `RawVec` with capacity `0`. If `T` is zero-sized, then it makes a
    /// `RawVec` with capacity `usize::MAX`. Useful for implementing
    /// delayed allocation.
    #[must_use]
    pub const fn new() -> Self {
        Self::new_in(Global)
    }

    /// Creates a `RawVec` (on the system heap) with exactly the capacity and
    /// alignment requirements for a `[T; cap]`. This is equivalent to
    /// calling `RawVec::new` when `capacity` is `0` or `T` is zero-sized.
    #[must_use]
    #[inline]
    pub fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_in(cap, Global)
    }

    #[must_use]
    #[inline]
    pub fn with_capacity_zeroed(capacity: usize) -> Self {
        Self::with_capacity_zeroed_in(capacity, Global)
    }
}

impl<T> Default for RawVec<T, Global> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, A: Allocator> RawVec<T, A> {
    // Tiny Vecs are dumb. Skip to:
    // - 8 if the element size is 1, because any heap allocators is likely
    //   to round up a request of less than 8 bytes to at least 8 bytes.
    // - 4 if elements are moderate-sized (<= 1 KiB).
    // - 1 otherwise, to avoid wasting too much space for very short Vecs.
    pub(crate) const MIN_NON_ZERO_CAP: usize = if mem::size_of::<T>() == 1 {
        8
    } else if mem::size_of::<T>() <= 1024 {
        4
    } else {
        1
    };

    /// Like `new`, but parameterized over the choice of allocator for
    /// the returned `RawVec`.
    pub const fn new_in(alloc: A) -> Self {
        // `cap: 0` means ""unallocated". zero-sized types are ignored.
        Self {
            ptr: NonNull::dangling(),
            cap: Cap::ZERO,
            alloc,
        }
    }

    /// Like `with_capacity`, but parameterized over the choice of allocator for
    /// the returned `RawVec`.
    #[inline]
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self::allocate_in(capacity, AllocInit::Uninitialized, alloc)
    }

    /// Like `with_capacity_zeroed`, but parameterized over the choice of allocator for
    /// the returned `RawVec`.
    #[inline]
    pub fn with_capacity_zeroed_in(capacity: usize, alloc: A) -> Self {
        Self::allocate_in(capacity, AllocInit::Zeroed, alloc)
    }

    fn allocate_in(capacity: usize, init: AllocInit, alloc: A) -> Self {
        // Don't allocate here because `Drop` will not deallocate when `capacity` is 0.
        if T::IS_ZST || capacity == 0 {
            Self::new_in(alloc)
        } else {
            // We avoid `unwrap_or_else` here because it bloats the amount of
            // LLVM IR generated.
            let layout = match Layout::array::<T>(capacity) {
                Ok(layout) => layout,
                Err(_) => capacity_overflow(),
            };
            match alloc_guard(layout.size()) {
                Ok(_) => {}
                Err(_) => capacity_overflow(),
            }
            let result = match init {
                AllocInit::Uninitialized => alloc.allocate(layout),
                AllocInit::Zeroed => alloc.allocate_zeroed(layout),
            };
            let ptr = match result {
                Ok(ptr) => ptr,
                Err(_) => handle_alloc_error(layout),
            };
            // Allocators currently return a `NonNull<[u8]>` whose length
            // matches the size requested. If that ever changes, the capacity
            // here should change to `ptr.len() / mem::size_of::<T>()`.
            Self {
                ptr: unsafe { NonNull::new_unchecked(ptr.cast().as_ptr()) },
                cap: Cap(capacity),
                alloc,
            }
        }
    }

    /// Gets a raw pointer to the start of the allocation.
    /// Note that this is `NonNull::dangling()` if `capacity == 0` or `T` is a ZST.
    /// #[inline]
    pub fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// Gets the capacity of the allocation.
    ///
    /// This will always return `usize::MAX` if `T` is a ZST.
    #[inline(always)]
    pub fn capacity(&self) -> usize {
        if T::IS_ZST {
            usize::MAX
        } else {
            self.cap.0
        }
    }

    fn current_memory(&self) -> Option<(NonNull<u8>, Layout)> {
        if T::IS_ZST || self.cap.0 == 0 {
            None
        } else {
            const { assert!(mem::size_of::<T>() % mem::align_of::<T>() == 0) };
            unsafe {
                let align = mem::align_of::<T>();
                let size = mem::size_of::<T>().unchecked_mul(self.cap.0);
                let layout = Layout::from_size_align_unchecked(size, align);
                Some((self.ptr.cast(), layout))
            }
        }
    }

    /// Returns a shared reference to the allocator backing this `RawVec`.
    pub fn allocator(&self) -> &A {
        &self.alloc
    }

    fn needs_to_grow(&self, len: usize, additional: usize) -> bool {
        additional > self.capacity().wrapping_sub(len)
    }

    pub fn reserve(&mut self, len: usize, additional: usize) {
        if self.needs_to_grow(len, additional) {
            handle_reserve(self.grow_amortized(len, additional))
        }
    }

    /// A specialized version of `reserve()` used only by the hot and
    /// oft-instantiated `Vec::push()`, which does its own capacity check.
    pub fn reserve_for_push(&mut self, len: usize) {
        handle_reserve(self.grow_amortized(len, 1))
    }
}

impl<T, A: Allocator> RawVec<T, A> {
    unsafe fn set_ptr_and_cap(&mut self, ptr: NonNull<[u8]>, cap: usize) {
        // Allocators currently return a `NonNull<[u8]>` whose length matches
        // the size requested. If that ever changes, the capacity there should
        // change to `ptr.len() / mem::size_of::<T>()`.
        self.ptr = NonNull::new_unchecked(ptr.cast().as_ptr());
        self.cap = Cap(cap);
    }

    fn grow_amortized(&mut self, len: usize, additional: usize) -> Result<(), TryReserveError> {
        debug_assert!(additional > 0);
        if T::IS_ZST {
            // Since we return a capacity of `usize::MAX` when `elem_size` is 0,
            // getting to there necessarily means the `RawVec` is overfull.
            return Err(TryReserveErrorKind::CapacityOverflow.into());
        }

        // Nothing we can really do about these checks, sadly.
        let required_cap = len
            .checked_add(additional)
            .ok_or(TryReserveErrorKind::CapacityOverflow)?;

        // This guarantees exponential growth.
        // The doubling cannot overflow because `cao <= isize::MAX` and the type of `cap` is `usize`.
        let cap = cmp::max(self.cap.0 * 2, required_cap);
        let cap = cmp::max(Self::MIN_NON_ZERO_CAP, cap);

        let new_layout = Layout::array::<T>(cap);

        // `finish_grow` is non-generic over `T`.
        let ptr = finish_grow(new_layout, self.current_memory(), &mut self.alloc)?;
        // SAFETY: finish_grow would have resulted in a capacity overflow if we tried to allocate more than isize::MAX items
        unsafe { self.set_ptr_and_cap(ptr, cap) };

        Ok(())
    }
}

unsafe impl<#[may_dangle] T, A: Allocator> Drop for RawVec<T, A> {
    fn drop(&mut self) {
        if let Some((ptr, layout)) = self.current_memory() {
            unsafe {
                self.alloc.deallocate(ptr.cast(), layout);
            }
        }
    }
}

#[inline(never)]
fn finish_grow<A>(
    new_layout: Result<Layout, LayoutError>,
    current_memory: Option<(NonNull<u8>, Layout)>,
    alloc: &mut A,
) -> Result<NonNull<[u8]>, TryReserveError>
where
    A: Allocator,
{
    // Check for the error here to minimize the size of `RawVec::grow_*`.
    let new_layout = new_layout.map_err(|_| TryReserveErrorKind::CapacityOverflow)?;

    alloc_guard(new_layout.size())?;

    let memory = if let Some((ptr, old_layout)) = current_memory {
        debug_assert_eq!(old_layout.align(), new_layout.align());
        unsafe {
            // The allocator checks for alignment equality
            hint::assert_unchecked(old_layout.align() == new_layout.align());
            alloc.grow(ptr, old_layout, new_layout)
        }
    } else {
        alloc.allocate(new_layout)
    };

    memory.map_err(|_| {
        TryReserveErrorKind::AllocError {
            layout: new_layout,
            non_exhaustive: (),
        }
        .into()
    })
}

// We need to guarantee the following:
// * We don't ever allocate `> isize::MAX` byte-size objects.
// * We don't overflow `usize::MAX` and actually allocate too little.
#[inline]
fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(TryReserveErrorKind::CapacityOverflow.into())
    } else {
        Ok(())
    }
}

fn handle_reserve(result: Result<(), TryReserveError>) {
    match result.map_err(|e| e.kind()) {
        Err(TryReserveErrorKind::CapacityOverflow) => capacity_overflow(),
        Err(TryReserveErrorKind::AllocError { layout, .. }) => handle_alloc_error(layout),
        Ok(()) => { /* yay */ }
    }
}

// One central function responsible for reporting capacity overflows. This'll
// ensure that the code generation related to these panics is minimal as there's
// only one location which panics rather than a bunch throughout the module.
fn capacity_overflow() -> ! {
    panic!("capacity overflow");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_vec() {
        let raw_vec: RawVec<i64> = RawVec::with_capacity(3);
        assert_eq!(raw_vec.capacity(), 3);
    }
}
