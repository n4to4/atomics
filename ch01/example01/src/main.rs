use std::thread;

fn main() {
    mutex();
}

#[allow(dead_code)]
fn mutex() {
    use std::sync::Mutex;

    let n = Mutex::new(0);
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut guard = n.lock().unwrap();
                for _ in 0..100 {
                    *guard += 1;
                }
            });
        }
    });
    assert_eq!(n.into_inner().unwrap(), 1000)
}

#[allow(dead_code, unused_variables)]
fn sendsync() {
    use std::cell::Cell;
    use std::marker::PhantomData;

    struct _X {
        handle: i32,
        _not_sync: PhantomData<Cell<()>>,
    }

    struct Y {
        p: *mut i32,
    }

    fn send<T: Send>(_: T) {}
    fn sync<T: Sync>(_: T) {}

    let y = Y {
        p: &mut 42 as *mut i32,
    };

    //send(y);
    //sync(y);
}

#[allow(dead_code)]
fn cell() {
    use std::cell::Cell;

    let a = Cell::new(2);
    f(&a, &a);

    fn f(a: &Cell<i32>, b: &Cell<i32>) {
        let before = a.get();
        b.set(b.get() + 1);
        let after = a.get();
        if before != after {
            unreachable!("might happen");
        }
    }
}

#[allow(dead_code)]
fn sharing() {
    use std::rc::Rc;

    let a = Rc::new([1, 2, 3]);
    let b = a.clone();

    assert_eq!(a.as_ptr(), b.as_ptr());

    use std::sync::Arc;

    let a = Arc::new([1, 2, 3]);
    let b = a.clone();

    let t1 = thread::spawn(move || dbg!(a));
    let t2 = thread::spawn(move || dbg!(b));

    t1.join().unwrap();
    t2.join().unwrap();

    //---

    use std::time::Duration;

    let a = Arc::new([2, 3, 4]);
    let t1 = thread::spawn({
        let a = a.clone();
        move || {
            println!("sleeping...");
            thread::sleep(Duration::from_secs(3));
            dbg!(a);
        }
    });
    let t2 = thread::spawn({
        let a = a.clone();
        move || {
            println!("sleeping...");
            thread::sleep(Duration::from_secs(1));
            dbg!(a);
        }
    });

    t1.join().unwrap();
    t2.join().unwrap();
}

#[allow(dead_code)]
fn leaking() {
    let x: &'static [i32; 3] = Box::leak(Box::new([1, 2, 3]));

    let t1 = thread::spawn(move || dbg!(x));
    let t2 = thread::spawn(move || dbg!(x));
    t1.join().unwrap();
    t2.join().unwrap();
}

#[allow(dead_code)]
fn scoped_threads() {
    let numbers = vec![1, 2, 3];

    thread::scope(|s| {
        s.spawn(|| {
            println!("length: {}", numbers.len());
        });
        s.spawn(|| {
            for n in &numbers {
                println!("{n}");
            }
        });
    });
}

#[allow(dead_code)]
fn thread_examples() {
    let t1 = thread::spawn(f);
    let t2 = thread::spawn(f);

    println!("Hello from the main thread.");

    t1.join().unwrap();
    t2.join().unwrap();

    //---

    let numbers = vec![1, 2, 3];
    thread::spawn(move || {
        for n in &numbers {
            println!("{n}");
        }
    })
    .join()
    .unwrap();

    //---

    let numbers = Vec::from_iter(0..=1000);
    let t = thread::spawn(move || {
        let len = numbers.len();
        let sum = numbers.iter().sum::<usize>();
        sum / len
    });
    let average = t.join().unwrap();
    println!("average: {average}");
}

#[allow(dead_code)]
fn f() {
    println!("Hello from another thread!");

    let id = thread::current().id();
    println!("This is my thread id: {id:?}");
}
