#![allow(dead_code)]

use std::sync::atomic::{AtomicU64, Ordering::Relaxed};

fn main() {
    println!("{}", get_x());
}

fn get_x() -> u64 {
    static X: AtomicU64 = AtomicU64::new(0);
    let mut x = X.load(Relaxed);
    if x == 0 {
        x = calculate_x();
        X.store(x, Relaxed);
    }
    x
}

fn calculate_x() -> u64 {
    42
}
