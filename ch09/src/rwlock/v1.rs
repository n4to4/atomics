use std::{cell::UnsafeCell, sync::atomic::AtomicU32};

pub struct RwLock<T> {
    state: AtomicU32,
    value: UnsafeCell<T>,
}
