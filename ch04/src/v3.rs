use std::{
    cell::UnsafeCell,
    ops::{Deref, DerefMut},
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Release},
    },
};

pub struct SpinLock<T> {
    locked: AtomicBool,
    value: UnsafeCell<T>,
}

pub struct Guard<'a, T> {
    lock: &'a SpinLock<T>,
}

unsafe impl<T> Sync for SpinLock<T> where T: Send {}

impl<T> SpinLock<T> {
    pub const fn new(value: T) -> Self {
        Self {
            locked: AtomicBool::new(false),
            value: UnsafeCell::new(value),
        }
    }

    #[allow(clippy::mut_from_ref, clippy::needless_lifetimes)]
    pub fn lock(&self) -> Guard<T> {
        while self.locked.swap(true, Acquire) {
            std::hint::spin_loop();
        }
        Guard { lock: self }
    }

    /// # Safety
    ///
    /// The &mut T from lock() must be gone!
    /// (And no cheating by keeping reference to fields of that T around!)
    pub unsafe fn unlock(&self) {
        self.locked.store(false, Release);
    }
}

impl<T> Deref for Guard<'_, T> {
    type Target = T;
    fn deref(&self) -> &T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &*self.lock.value.get() }
    }
}

impl<T> DerefMut for Guard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        // Safety: The very existence of this Guard
        // guarantees we've exclusively locked the lock.
        unsafe { &mut *self.lock.value.get() }
    }
}
