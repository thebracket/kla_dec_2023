use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    std::thread::scope(|scope| {
        let my_counter = counter.clone();
        let t1 = scope.spawn(move || {
            for _ in 0 .. 1000000 {
                let mut lock = my_counter.lock().unwrap();
                *lock += 1;
            }
        });

        let my_counter = counter.clone();
        let t2 = scope.spawn(move || {
            for _ in 0 .. 1000000 {
                let mut lock = my_counter.lock().unwrap();
                *lock += 1;
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    let lock = counter.lock().unwrap();
    println!("{}", *lock);
}