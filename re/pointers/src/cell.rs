use std::cell::UnsafeCell;

pub struct Cell<T> {
    value: UnsafeCell<T>,
}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // Safety: We know no-one else is concurrently mutating self.value(because !Sync).
        // Safety: We know we're not invalidating any references, because we never give any out.
        unsafe {
            *self.value.get() = value;
        }
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        // Safety: We know no-one else is modifying this value, since only this thread can mutate, and it
        // is executing this function instead.
        unsafe { *self.value.get() }
    }
}
