fn double_it(n: i32) -> i32 {
    n * 2
}

fn main() {
    let i = 5;
    let j = double_it(i);
    println!("{i} *2 = {j}");
}