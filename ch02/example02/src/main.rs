use std::{
    sync::atomic::{AtomicUsize, Ordering::Relaxed},
    thread,
    time::Duration,
};

fn main() {
    let num_done = AtomicUsize::new(0);

    thread::scope(|s| {
        // A background thread to process all 100 items.
        s.spawn(|| {
            for i in 0..100 {
                process_item(i); // Assuming this takes some time.
                num_done.store(i + 1, Relaxed);
            }
        });

        // The main thread shows status updates, every second.
        loop {
            let n = num_done.load(Relaxed);
            if n == 100 {
                break;
            }
            println!("Working.. {n}/100 done");
            thread::sleep(Duration::from_millis(1000));
        }
    });

    println!("Done!");
}

fn process_item(_i: usize) {
    //println!("process_item: {i}");
    thread::sleep(Duration::from_millis(100));
}
