# Safety

Rust advertises heavily that it offers memory safety as a primary feature. Bjarne Stroustrup has repeatedly pointed out that modern C++ has many similar safety options---but they are *opt in* rather than *opt out*. Rust defaults to the safe code path; C++ defaults to the blazing fast with no seat-belts path.

This section is focused on Rust's memory safety guarantees from a C++ perspective.

We've seen a few examples of Rust offering memory safety already:

* You have to explicitly convert between types. `i32` and `u32` can't be directly assigned without a conversion.
* Overflow behavior is explicit: you opt in to wrapping, saturation, etc. The `checked_op` series of functions make it easy to test for overflow, division by zero and similar at runtime.
* Accessing beyond the bounds of an array or vector panics safely---that is it doesn't trigger potentially dangerous behavior.

Let's look at another one:

## No Null Pointers

Rust does not have a null pointer.* So the common C convention of returning a pointer, or `nullptr` if the operation failed, doesn't exist. Instead, Rust has the `Option` type.

* - Rust actually *does* have several nullable types, but they are all tagged as `unsafe`!

The `Option` type is a *sum type*. Just like a tagged `union`, it contains exactly one value---and is equal to the size of the largest option in memory. Options can be either:

* `None` - they contain no data.
* `Some(T)` - they contain a T.

If you could just use an `Option` like a pointer, we wouldn't have gained any safety. So instead, you have to explicitly access its contents---the compiler is forcing you to perform the null pointer check.

For example:

> This example is in `projects/part2/no_null`

```rust
fn frobnicator() -> Option<i32> {
    Some(3)
}

fn main() {
    // Panic if the option is equal to None
    let a = frobnicator().unwrap();
    
    // Panic with a nice error message if the option is equal to None
    let a = frobnicator().expect("The frobnicator is broken!");

    // Just check to see if we got a value at all
    let a = frobnicator();
    if a.is_some() { 
        // Do something
    }
    if a.is_none() {
        // Do something
    }

    // Use "match" for pattern matching
    let a = frobnicator();
    match a {
        None => {
            // Do Something
        }
        Some(a) => {
            // a now refers to the contents of a
            // Do something
        }
    }

    // Use "if let" for single-arm pattern matching
    let a = frobnicator();
    if let Some(a) = a {
        // Do something
    }
}
```

So yes, that's more typing. On the other hand, C++ doesn't issue any warnings for the following program:

```c++
#include <iostream>

struct A {
    int a;
};

A * frobniactor() {
    return nullptr;
}

int main() {
    A * a = frobniactor();
    std::cout << "a is " << a->a << std::endl;
    return 0;
}
```

The program crashes at runtime with `Segmentation Fault`.

## Summary

So Rust protects you from one of the most common issues. You won't encounter null pointer issues in safe code, bounds-checking and all of the related security issues are protected by default. Type conversion being explicit makes it hard to accidentally change type and lose data (which can be extra "fun" in old C code with pointers of mixed types!), and overflow behavior being opt-in reduces the risk of accidentally overflowing a type.
