use std::{ops::Deref, ptr::NonNull};

use crate::cell::Cell;

struct RcInner<T> {
    value: T,
    ref_count: Cell<usize>,
}

pub struct Rc<T> {
    inner: NonNull<RcInner<T>>,
}

impl<T> Rc<T> {
    pub fn new(v: T) -> Self {
        let inner = Box::new(RcInner {
            value: v,
            ref_count: Cell::new(1),
        });

        Rc {
            inner: unsafe {
                // SAFETY: Box::into_raw returns a pointer to the heap allocated value.
                // We know that the value is still alive, because we just created it.
                // So we can safely convert it to a NonNull.
                NonNull::new_unchecked(Box::into_raw(inner))
            },
        }
    }
}

impl<T> Deref for Rc<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY: self.inner is a Box that is only deallocated when the last Rc goes away.
        // we have an Rc, therefore the Box is still alive. So dereferencing it is safe.
        &unsafe { self.inner.as_ref() }.value
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.ref_count.get();
        inner.ref_count.set(c + 1);
        Rc { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let inner = unsafe { self.inner.as_ref() };
        let c = inner.ref_count.get();
        if c == 1 {
            // SAFETY: we are the last Rc, so we can drop the inner value.
            unsafe {
                let _ = Box::from_raw(self.inner.as_ptr());
            }
        } else {
            // There are other Rcs, so we just decrement the ref count.
            inner.ref_count.set(c - 1);
        }
    }
}
