use std::{
    alloc::{handle_alloc_error, Allocator, Global, Layout},
    collections::{TryReserveError, TryReserveErrorKind},
    mem::SizedTypeProperties,
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

pub struct RawVec<T, A: Allocator = Global> {
    ptr: NonNull<T>,
    cap: Cap,
    alloc: A,
}

impl<T> RawVec<T, Global> {
    #[must_use]
    pub fn new() -> Self {
        Self::new_in(Global)
    }

    #[must_use]
    pub fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_in(cap, Global)
    }

    #[must_use]
    pub fn with_capacity_zeroed(capacity: usize) -> Self {
        Self::with_capacity_zeroed_in(capacity, Global)
    }
}

impl<T, A: Allocator> RawVec<T, A> {
    pub const fn new_in(alloc: A) -> Self {
        Self {
            ptr: NonNull::dangling(),
            cap: Cap::ZERO,
            alloc,
        }
    }

    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self::allocate_in(capacity, AllocInit::Uninitialized, alloc)
    }

    pub fn with_capacity_zeroed_in(capacity: usize, alloc: A) -> Self {
        Self::allocate_in(capacity, AllocInit::Zeroed, alloc)
    }

    fn allocate_in(capacity: usize, init: AllocInit, alloc: A) -> Self {
        if T::IS_ZST || capacity == 0 {
            Self::new_in(alloc)
        } else {
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

            Self {
                ptr: unsafe { NonNull::new_unchecked(ptr.cast().as_ptr()) },
                cap: Cap(capacity),
                alloc,
            }
        }
    }

    pub fn capacity(&self) -> usize {
        if T::IS_ZST {
            usize::MAX
        } else {
            self.cap.0
        }
    }

    pub fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn allocator(&self) -> &A {
        &self.alloc
    }
}

fn alloc_guard(alloc_size: usize) -> Result<(), TryReserveError> {
    if usize::BITS < 64 && alloc_size > isize::MAX as usize {
        Err(TryReserveErrorKind::CapacityOverflow.into())
    } else {
        Ok(())
    }
}

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
