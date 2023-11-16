#![allow(dead_code)]

fn main() {
    v0::run();
}

mod v0 {
    use std::{
        hint::black_box, sync::atomic::AtomicU64, sync::atomic::Ordering::Relaxed, time::Instant,
    };

    static A: AtomicU64 = AtomicU64::new(0);

    pub fn run() {
        black_box(&A);
        let start = Instant::now();
        for _ in 0..1_000_000_000 {
            black_box(A.load(Relaxed));
        }
        println!("{:?}", start.elapsed());
    }
}

mod v1 {
    use std::{
        hint::black_box,
        sync::atomic::{AtomicU64, Ordering::Relaxed},
        thread,
        time::Instant,
    };

    static A: AtomicU64 = AtomicU64::new(0);

    pub fn run() {
        black_box(&A);

        thread::spawn(|| loop {
            A.store(0, Relaxed);
            black_box(A.compare_exchange(10, 20, Relaxed, Relaxed).is_ok());
        });

        let start = Instant::now();
        for _ in 0..1_000_000_000 {
            black_box(A.load(Relaxed));
        }
        println!("{:?}", start.elapsed());
    }
}

mod v2 {
    use std::{
        hint::black_box,
        sync::atomic::{AtomicU64, Ordering::Relaxed},
        thread,
        time::Instant,
    };

    #[repr(align(64))]
    struct Aligned(AtomicU64);
    static A: [Aligned; 3] = [
        Aligned(AtomicU64::new(0)),
        Aligned(AtomicU64::new(0)),
        Aligned(AtomicU64::new(0)),
    ];

    pub fn run() {
        black_box(&A);
        thread::spawn(|| loop {
            A[0].0.store(1, Relaxed);
            A[2].0.store(1, Relaxed);
        });
        let start = Instant::now();
        for _ in 0..1_000_000_000 {
            black_box(A[1].0.load(Relaxed));
        }
        println!("{:?}", start.elapsed());
    }
}
