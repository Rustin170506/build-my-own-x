use std::sync::atomic::{AtomicBool, Ordering};

pub struct SpinLock {
    locked: AtomicBool,
}

impl SpinLock {
    pub const fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
        }
    }

    pub fn lock(&self) {
        while self.locked.swap(true, Ordering::Acquire) {
            std::hint::spin_loop();
        }
    }

    pub fn unlock(&self) {
        self.locked.store(false, Ordering::Release);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use std::thread;

    #[test]
    fn test_spinlock() {
        let spinlock = Arc::new(SpinLock::new());
        let spinlock_clone = spinlock.clone();

        let handle = thread::spawn(move || {
            spinlock_clone.lock();
            spinlock_clone.unlock();
        });

        spinlock.lock();
        spinlock.unlock();

        handle.join().unwrap();
    }
}
