use std::time::Instant;

fn main() {
    let now = Instant::now();
    let mut i = 0;
    for j in 0 .. 1_000 {
        i += j*j;
    }
    let elapsed = now.elapsed();
    println!("Time elapsed: {} nanos", elapsed.as_nanos());
    println!("{i}");
}