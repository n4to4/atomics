use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed, Release},
    },
};

pub struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Channel<T> {
    pub const fn new() -> Self {
        Self {
            message: UnsafeCell::new(MaybeUninit::uninit()),
            ready: AtomicBool::new(false),
        }
    }

    /// # Safety
    /// Only call this once!
    pub unsafe fn send(&self, message: T) {
        (*self.message.get()).write(message);
        self.ready.store(true, Release);
    }

    pub fn is_ready(&self) -> bool {
        self.ready.load(Relaxed)
    }

    /// Panics if no message is available yet,
    /// or if the message was already consumed.
    ///
    /// Tip: Use `is_ready` to check first.
    pub fn receive(&self) -> T {
        if !self.ready.swap(false, Acquire) {
            panic!("no message available!");
        }
        // Safety: We've just checked (and reset) the ready flag.
        unsafe { (*self.message.get()).assume_init_read() }
    }
}
