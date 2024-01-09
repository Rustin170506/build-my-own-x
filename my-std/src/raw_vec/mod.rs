use std::{
    alloc::{handle_alloc_error, Allocator, Global, Layout},
    collections::{TryReserveError, TryReserveErrorKind},
    ptr::Unique,
};

pub struct RawVec<T, A: Allocator = Global> {
    ptr: Unique<T>,
    cap: usize,
    index: usize,
    alloc: A,
}

impl<T> RawVec<T, Global> {
    pub fn new() -> Self {
        Self::with_capacity(0)
    }
    pub fn with_capacity(cap: usize) -> Self {
        Self::with_capacity_in(cap, Global)
    }
}

impl<T, A: Allocator> RawVec<T, A> {
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        Self::allocate_in(capacity, alloc)
    }

    pub fn ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    pub fn allocator(&self) -> &A {
        &self.alloc
    }

    pub fn push(&mut self, value: T) {
        unsafe {
            self.ptr.as_ptr().add(self.index).write(value);
        }
        self.index += 1;
    }

    pub fn pop(&mut self) -> T {
        self.index -= 1;
        unsafe { self.ptr.as_ptr().add(self.index).read() }
    }

    fn allocate_in(capacity: usize, alloc: A) -> Self {
        if capacity == 0 {
            Self {
                ptr: Unique::dangling(),
                index: 0,
                cap: 0,
                alloc,
            }
        } else {
            let layout = match Layout::array::<T>(capacity) {
                Ok(layout) => layout,
                Err(_) => capacity_overflow(),
            };
            match alloc_guard(layout.size()) {
                Ok(_) => {}
                Err(_) => capacity_overflow(),
            }
            let result = alloc.allocate(layout);
            let ptr = match result {
                Ok(ptr) => ptr,
                Err(_) => handle_alloc_error(layout),
            };

            Self {
                ptr: unsafe { Unique::new_unchecked(ptr.cast().as_ptr()) },
                index: 0,
                cap: capacity,
                alloc,
            }
        }
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
        let mut vec = RawVec::with_capacity(3);
        vec.push(1);
        vec.push(2);
        vec.push(3);
        assert_eq!(vec.pop(), 3);
        assert_eq!(vec.pop(), 2);
        assert_eq!(vec.pop(), 1);
    }
}
