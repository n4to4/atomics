use std::{
    sync::atomic::{
        AtomicBool,
        Ordering::{Acquire, Relaxed},
    },
    thread,
};

static mut DATA: String = String::new();
static LOCKED: AtomicBool = AtomicBool::new(false);

fn f() {
    if LOCKED
        .compare_exchange(false, true, Acquire, Relaxed)
        .is_ok()
    {
        unsafe { DATA.push('!') };
        LOCKED.store(false, Relaxed);
    }
}

fn main() {
    thread::scope(|s| {
        for _ in 0..100 {
            s.spawn(f);
        }
    });
}
