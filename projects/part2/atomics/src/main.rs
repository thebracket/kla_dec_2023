use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicU32;

fn main() {
    let counter = AtomicU32::new(0);
    std::thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
        let t2 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    println!("{}", counter.load(Relaxed));
}