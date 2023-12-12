# Reference Counting - Borrow Checker Escape Hatch

Now that we've covered a lot of ground with the borrow checker, moving and lifetimes---let's breathe a sigh of relief knowing that there are some escape hatches if you need them. They do come at a cost, but it's a manageable one.

Move-by-default and borrowing assume *ownership*. This is conceptually similar to a `unique_ptr` in C++: the `unique_ptr` has *ownership* of the data it is holding (and handles clean-up for you). C++ also has `shared_ptr` to handle those times that ownership is murky, and you just want to be sure that the object goes away when nobody is using it anymore.

Rust has `Rc` (for "reference counted") as a wrapper type for this. (There's also `Arc` - atomic reference counted - for multi-threaded situations).

You can turn any variable into a reference-counted variable (on the *heap*) by wrapping it in `Rc`:

> This is in `projects/part2/refcount`

```rust
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
```

So we take a reference, move a clone (the `Rc` type is designed to have `clone()` called whenever you want a new shared pointer to the original)---and the data is only dropped once. It is shared between all the functions. You can use this to spread data widely between functions.

You can't *mutate* the contents of an `Rc` without some additional help. We're going to talk about synchronization protection next, in Data Races.