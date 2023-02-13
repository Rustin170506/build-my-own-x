#![feature(dropck_eyepatch)]

use std::fmt::Debug;
use std::marker::PhantomData;

pub struct Boks<T> {
    p: *mut T,
    _t: PhantomData<T>,
}

unsafe impl<#[may_dangle] T> Drop for Boks<T> {
    fn drop(&mut self) {
        // SAFETY: p is valid since it was created by Box::into_raw. And has not been dropped yet.
        // Otherwise, drop could not be called.
        unsafe { Box::from_raw(self.p) };
    }
}

impl<T> Boks<T> {
    pub fn ny(t: T) -> Self {
        Boks {
            p: Box::into_raw(Box::new(t)),
            _t: PhantomData,
        }
    }
}

impl<T> std::ops::Deref for Boks<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // SAFETY: self.p is valid since it was created by Box::into_raw.
        unsafe { &*self.p }
    }
}

impl<T> std::ops::DerefMut for Boks<T> {
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: self.p is valid since it was created by Box::into_raw.
        // Also, since we have &mut self, no other mutable reference to self.p.
        unsafe { &mut *self.p }
    }
}

struct Oisann<T: Debug>(T);

impl<T: Debug> Drop for Oisann<T> {
    fn drop(&mut self) {
        println!("Dropped {:?}", self.0);
    }
}

fn main() {
    let x = 42;
    let b = Boks::ny(x);
    println!("{}", *b);

    let mut y = 42;
    let _b = Boks::ny(&mut y);
    println!("{y}");

    let mut z = 42;
    let _b = Boks::ny(Oisann(&mut z));
    // println!("{z}");
}
