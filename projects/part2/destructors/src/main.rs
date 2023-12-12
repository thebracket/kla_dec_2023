struct MyStruct {
    s: String
}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping: {}", self.s);
    }
}

fn do_it(a: MyStruct) {
    println!("do_it called");
}

fn move_it(a: MyStruct) -> MyStruct {
    println!("move_it called");
    a
}

fn main() {
    let a = MyStruct { s: "1".to_string() };
    do_it(a);
    // a no longer exists

    let b = MyStruct { s: "2".to_string() };
    let b = move_it(b);
    println!("{}", b.s);
}