use std::{
    alloc::{Allocator, Global},
    ptr,
};

use crate::raw_vec::RawVec;

pub struct MyVec<T, A: Allocator = Global> {
    buf: RawVec<T, A>,
    len: usize,
}

impl<T> MyVec<T> {
    pub const fn new() -> Self {
        MyVec {
            buf: RawVec::NEW,
            len: 0,
        }
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self::with_capacity_in(capacity, Global)
    }
}

impl<T, A: Allocator> MyVec<T, A> {
    pub fn with_capacity_in(capacity: usize, alloc: A) -> Self {
        MyVec {
            buf: RawVec::with_capacity_in(capacity, alloc),
            len: 0,
        }
    }

    pub fn as_mut_ptr(&mut self) -> *mut T {
        self.buf.ptr()
    }

    pub fn push(&mut self, value: T) {
        if self.len == self.buf.capacity() {
            // grow
        }
        unsafe {
            let end = self.as_mut_ptr().add(self.len);
            ptr::write(end, value);
            self.len += 1;
        }
    }

    /// Removes the last element from a vector and returns it, or [`None`] if it is empty.
    pub fn pop(&mut self) -> Option<T> {
        if self.len == 0 {
            return None;
        } else {
            unsafe {
                self.len -= 1;
                core::hint::assert_unchecked(self.len < self.buf.capacity());
                Some(ptr::read(self.as_mut_ptr().add(self.len)))
            }
        }
    }
}