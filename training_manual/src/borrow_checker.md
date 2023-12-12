# The Borrow Checker

The borrow checker gets a bad name from people who run into it and discover "I can't do anything!". The borrow checker *does* take a bit of getting used to - but in the medium term it really helps.

I went through a cycle going from C++ to Rust, and many people I've talked to went through the same:

* First week or two: *I hate the borrow checker! This is awful! I can't do anything!*
* Next: *I see how to work within what it wants, I can live with this*
* Then: Wow, I'm writing Rust-like C++ and Go now - and my code is failing less frequently.

The good news is that if you are familiar with Modern C++, you've run into a lot of the same issues that the borrow checker helps with. Let's work through some examples that show how life with Rust is different.

## Immutable by Default

This one trips a few people up when they start with Rust. This won't compile:

```rust
fn main() {
    let i = 5;
    i += 1;
}
```

Variables are *immutable by default*. In C++ terms, you just tried to write:

```c++
int main() {
    const i = 5;
    i += 1;
    return 0;
}
```

You can make `i` mutable and it works as you'd expect:

```rust
fn main() {
    let mut i = 5;
    i += 1;
}
```

In other words: C++ and Rust have exactly the opposite defaults. In C++, everything is mutable unless you `const` it. Rust, everything is immutable unless you `mut` it.

> You could simply declare everything to be mutable. The linter will regularly remind you that things can be immutable. It's considered good Rust style to minimize mutability, so you aren't surprised by mutations.

## Move by Default

> Quick show of hands. Who knows what `std::move` does? Who *really* likes `std::move`?

This one surprises everyone. The following code does what you'd expect:

```rust
fn do_it(a: i32) {
    // Do something
}

fn main() {
    let a = 42;
    do_it(a);
    println!("{a}");
}
```

So why doesn't this work?

```rust
fn do_it(a: String) {
    // Do something
}

fn main() {
    let a = String::from("Hello");
    do_it(a);
    println!("{a}");
}
```

So why did this work with `i32`? `i32` is a *primitive* - and implements a trait named `Copy`. Types can only implement `Copy` if they are equal to or smaller than a register---it's actually faster to just copy them than to use a pointer to their value. This is the same as C++ copying primitive types. When you work with a complex type (`String` and C++'s `std::string` are very similar; a size, a heap-allocated buffer of characters. In Rust's case they are UTF-8).

The error message `borrow of moved value`, with a long explanation isn't as helpful as you might like.

The key is: **Rust is move by default**, and Rust is more strict about moving than C++. Here's what you wrote in C++ terms:

```c++
#include <string>

void do_it(std::string s) {
    // Do something
}

int main() {
    std::string s = "Hello";
    do_it(std::move(s));
    // s is now in a valid but unspecified state
    return 0;
}
```

What happens if you use `s`? Nobody knows, it's undefined behavior. `std::move` in C++ converts an object to an `xvalue`---a type that has "been moved out of", and may not may not be in a valid state. Rust takes this to the logical conclusion, and prevents access to a "moved out of" type.

## Moving Values Around

If you want to, you can move variables in and out of functions:

```rust
fn do_it(a: String) -> String {
    // Do something
    a
}

fn main() {
    let a = String::from("Hello");
    let a = do_it(a);
    println!("{a}");
}
```

This code is valid. Moving will generate `memcpy` that is usually removed by compiler optimizations, and LLVM applies the same returned-value optimizations as C++ for returning from a function.

Usually, I recommend moving out of a variable if you are genuinely done with it. Conceptually, you are *giving ownership of the object to another function* - it's not yours anymore, so you can't do much with it.

> This is conceptually very similar to using `unique_ptr` in C++. The smart pointer *owns* the contained data. You can move it between functions, but you can't copy it.

## Destructors and Moving

In C++, you can have move constructors---and moving structures around can require some thought as move constructors fire. Rust simplifies this. Moving a structure *does not* fire any sort of constructor. We haven't talked about destructors yet, so let's do that.

In Rust, destructors are implemented by a trait named `Drop`. You an add `Drop` to your own types. Let's use this to illustrate the lifetime of a type as we move it around:

> The code is in `projects/part2/destructors`

```rust
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
```

As you can see, `Drop` is called when the structure ceases to be in scope:

* `do_it` runs, and receives ownership of the object. The destructor fires as soon as the function exits.
* `move_it` runs, and the object remains in-scope. The destructor fires when the program exits.

> RAII is central to Rust's safety model. It's used everywhere. I try to remember to credit C++ with its invention every time I mention it!

## Borrowing (aka References)

So with that in mind, what if you don't *want* to move your data around a lot (and pray that the optimizer removes as many `memcpy` calls as possible)? This introduces *borrowing*. Here's a very simple function that takes a *borrowed* parameter:

```rust
fn do_it(s: &String) {
    println!("{s}");
}

fn main() {
    let s = "42".to_string();
    do_it(&s);
}
```

Predictably, this prints `42`. The semantics are similar to C++: you indicate a borrow/reference with `&`. Unlike C++, you have to indicate that you are passing a reference at both the call-site and the function signature---there's no ambiguity (which helps to avoid accidental passing by value/copying). This is the same as the following C++:

```c++
#include <string>
#include <iostream>

void do_it(const std::string &s) {
    std::cout << s << std::endl;
}

int main() {
    std::string s = "42";
    do_it(s);
    return 0;
}
```

Once again, notice that the reference is implicitly *immutable*.

If you want a mutable borrow---permitted to change the borrowed value---you have to indicate so.

```rust
fn do_it(s: &mut String) {
    s.push_str("1");
}

fn main() {
    let mut s = String::from("42");
    do_it(&mut s);
    println!("{s}");
}
```

Notice that you are:
* Making `s` mutable in the `let mut` declaration. You can't mutably lend an immutable variable.
* Explicitly decorating the *lend* as `&mut` at the call-site.
* Explicitly borrowing as mutable in the parameters (`(s: &mut String)`).

Rust doesn't leave any room for ambiguity here. You have to mean it when you allow mutation!

## Why Mutability Matters

The borrow checker enforces a very strict rule: a variable can only be borrowed mutably once at a time. You can have as many immutable borrows as you want---but only one current effective owner who can change the variable. This can take a little bit of getting used to.

So this is invalid code:

```rust
fn main() {
    let mut i: i32 = 1;
    let ref_i = &mut i;
    let second_ref_i = &mut i;
    println!("{i}");
    println!("{ref_i}");
    println!("{second_ref_i}");
}
```

> The print statements are included to prevent the optimizer from realizing that variables are unused and silently removing them.

For example, this is an example of some code that triggers borrow-checker rage:

```rust
fn main() {
    let mut data = vec![1,2,3,4,5];
    for (idx, value) in data.iter().enumerate() {
        if *value > 3 {
            data[idx] = 3;
        }
    }
    println!("{data:?}");
}
```

Look at the error message:

```
error[E0502]: cannot borrow `data` as mutable because it is also borrowed as immutable
 --> src/main.rs:5:13
  |
3 |     for (idx, value) in data.iter().enumerate() {
  |                         -----------------------
  |                         |
  |                         immutable borrow occurs here
  |                         immutable borrow later used here
4 |         if *value > 3 {
5 |             data[idx] = 3;
  |             ^^^^ mutable borrow occurs here
```

Using an iterator (with `.iter()`) immutably borrows each record in the vector in turn. But when we index into `data[idx]` to change the value, we're mutably borrowing. Since you can't have a mutable borrow and other borrows, this is invalid.

You have to be careful to limit access. You could rewrite this code a few ways. The most Rustacean way is probably:

> This is a good thing. Changing an underlying structure while you iterate it risks iterator invalidation.

### Option 1: The Rustacean Iterators Way

```rust
fn main() {
    let mut data = vec![1,2,3,4,5];
    data.iter_mut().filter(|d| **d > 3).for_each(|d| *d = 3);
    println!("{data:?}");
}
```

This is similar to how you'd do it with `ranges3` or the C++20 `ranges` feature. You are pipelining:

* You obtain a mutable iterator (it will pass an `&mut` reference to each entry in turn).
* You filter the target records with a predicate. `|d| **d > 3` is a closure (lambda function) - `d` is the parameter, which will arrive as `&&mut` because the iterator takes a reference (`&mut`) and the filter then passes a reference to the reference. (Good news: the compiler clean that up. I still think its ugly!)
* Then you run `for_each` on the remaining entries.

That's great for problems that naturally fit into an iterator solution.

### Option 2: Do the two-step

Another option is to separate the operations:

```rust
fn main() {
    let mut data = vec![1,2,3,4,5];
    let mut to_fix = Vec::new();
    for (idx, value) in data.iter().enumerate() {
        if *value > 3 {
            to_fix.push(idx);
        }
    }
    for idx in to_fix { // Note: no .iter(). We're *moving* through each entry, invalidating the vector!
        data[idx] = 3;
    }
    println!("{data:?}");
}
```

This is pretty typical: you "beat" the borrow checker by breaking your task down into specific stages. In this case, we avoided a potential iterator invalidation. We also made it a lot easier for the compiler to perform static analysis and prevent data races.

## Dangling Pointers

The borrow checker prevents a lot of dangling pointer and reference errors. For example:

```rust
fn main() {
    let s = String::from("Hello");
    let s_ref = &s;
    std::mem::drop(s);
    println!("{s_ref}");
}
```

Dropping `s` terminates its existence (it's the same as `delete`, it still calls destructors). Trying to print `s` after it is dropped is a compiler error: `s` no longer exists. Try the same in C++ and you don't get any warning by default (most static analysis will catch this):

```c++
#include <iostream>

int main() {
    std::string * s = new std::string("Hello");
    delete s;
    std::cout << *s << std::endl;
}
```

## Summary

The borrow checker does take some getting used to, but it's surprising how long you can go without running it into if you go with idiomatic, straight-forward code. It's especially hard coming from C++, which allows you to get by with a *lot*.

In this section, we've covered:

* Move by default, and Rust curing all "use after move" errors.
* Explicit borrowing, and no more "oops, I copied by value by mistake".
* Explicit mutability, to avoid surprises.
* The "one mutable access at a time" rule, which prevents hidden bugs like iterator invalidation.
* No more dangling pointers/references --- but still no garbage collector.

Now let's look at the second half of the borrow checker, *lifetimes*.