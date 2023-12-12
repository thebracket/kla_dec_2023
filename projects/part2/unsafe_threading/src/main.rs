fn main() {
    static mut COUNTER: u32 = 0;
    std::thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        let t2 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                unsafe {
                    COUNTER += 1;
                }
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    unsafe {
        println!("{COUNTER}");
    }
}