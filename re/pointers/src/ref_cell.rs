use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
};

use crate::cell::Cell;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum RefCellState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct Ref<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for Ref<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Unshared => unreachable!(),
            RefCellState::Shared(1) => {
                self.refcell.state.set(RefCellState::Unshared);
            }
            RefCellState::Shared(n) => {
                self.refcell.state.set(RefCellState::Shared(n - 1));
            }
            RefCellState::Exclusive => unreachable!(),
        }
    }
}

impl<T> Deref for Ref<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // a Ref is only create if no exclusive references have given out.
        // once it is given out, state is set to Shared, so no exclusive references are given out.
        // so dereferencing into a shared reference is fine.
        unsafe { &*self.refcell.value.get() }
    }
}

pub struct RefMut<'refcell, T> {
    refcell: &'refcell RefCell<T>,
}

impl<T> Drop for RefMut<'_, T> {
    fn drop(&mut self) {
        match self.refcell.state.get() {
            RefCellState::Unshared => unreachable!(),
            RefCellState::Shared(_) => unreachable!(),
            RefCellState::Exclusive => {
                self.refcell.state.set(RefCellState::Unshared);
            }
        }
    }
}

impl<T> Deref for RefMut<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // SAFETY:
        // See safety for DerefMut
        unsafe { &*self.refcell.value.get() }
    }
}

impl<T> DerefMut for RefMut<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        // SAFETY:
        // a RefMut is only create if no exclusive references have given out.
        // once it is given out, state is set to Exclusive, so no other references are given out.
        // so dereferencing into a shared reference is fine.
        unsafe { &mut *self.refcell.value.get() }
    }
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: Cell<RefCellState>,
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> RefCell<T> {
        RefCell {
            value: UnsafeCell::new(value),
            state: Cell::new(RefCellState::Unshared),
        }
    }

    pub fn borrow(&self) -> Option<Ref<'_, T>> {
        match self.state.get() {
            RefCellState::Unshared => {
                self.state.set(RefCellState::Shared(1));
                Some(Ref { refcell: self })
            }
            RefCellState::Shared(n) => {
                self.state.set(RefCellState::Shared(n + 1));
                Some(Ref { refcell: self })
            }
            RefCellState::Exclusive => None,
        }
    }

    pub fn borrow_mut(&self) -> Option<RefMut<'_, T>> {
        if self.state.get() == RefCellState::Unshared {
            // SAFETY: no other reference have been given out since state would be
            // Shared or Exclusive.
            self.state.set(RefCellState::Exclusive);
            return Some(RefMut { refcell: self });
        }
        None
    }
}
