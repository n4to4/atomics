use std::{
    cell::UnsafeCell,
    mem::MaybeUninit,
    sync::{atomic::AtomicBool, Arc},
};

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    todo!()
}

pub struct Sender<T> {
    channel: Arc<Channel<T>>,
}
pub struct Receiver<T> {
    channel: Arc<Channel<T>>,
}

struct Channel<T> {
    message: UnsafeCell<MaybeUninit<T>>,
    ready: AtomicBool,
}

unsafe impl<T> Sync for Channel<T> where T: Send {}

impl<T> Sender<T> {
    pub fn send(self, message: T) {}
}

impl<T> Receiver<T> {
    pub fn is_ready(&self) -> bool {
        false
    }

    pub fn receive(self) -> T {
        todo!()
    }
}
