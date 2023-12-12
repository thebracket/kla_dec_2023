use std::rc::Rc;

struct MyStruct {}

impl Drop for MyStruct {
    fn drop(&mut self) {
        println!("Dropping");
    }
}

fn move_it(n: Rc<MyStruct>) {
    println!("Moved");
}

fn ref_it(n: &MyStruct) {
    // Do something
}

fn main() {
    let shared = Rc::new(MyStruct{});
    move_it(shared.clone());
    ref_it(&shared);
}