use std::sync::atomic::{
    AtomicPtr,
    Ordering::{Acquire, Release},
};

#[derive(Debug, Default)]
struct Data {
    #[allow(dead_code)]
    name: String,
}

fn get_data() -> &'static Data {
    static PTR: AtomicPtr<Data> = AtomicPtr::new(std::ptr::null_mut());
    let mut p = PTR.load(Acquire);

    if p.is_null() {
        p = Box::into_raw(Box::new(generate_data()));
        if let Err(e) = PTR.compare_exchange(std::ptr::null_mut(), p, Release, Acquire) {
            drop(unsafe { Box::from_raw(p) });
            p = e;
        }
    }

    unsafe { &*p }
}

fn generate_data() -> Data {
    Default::default()
}

fn main() {
    let data = get_data();
    println!("{data:?}");
}
