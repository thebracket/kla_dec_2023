# Structures

Rust and C++ have similar support for structures (there is no `class` keyword). Like a C++ `struct`, a Rust `struct` is private by default.

```rust
#[derive(Debug)]
struct MyStruct {
    a: i32,
    b: u32,
    c: usize,
    pub d: String, // the `pub` keyword marks that field as "public"
}

fn main() {
    let val = MyStruct {
        a: 1,
        b: 2,
        c: 3,
        d: String::from("Hello"),
    };
    println!("{val:#?}");
}
```

> `#[derive]` executes a *procedural macro* on the type at compilation time. Derive macros automatically implement traits for you---we'll be covering that later. In this case, `#[derive(Debug)]` feels like magic. It reflects on the structure type, and builds a debug formatter. It works as long as everything in the type also supports `Debug`.

## Associated Functions

There's no direct equivalent of C++ constructors---definitely no rule of 0/5/7. Structures can have functions, and by convention constructors are *associated* functions. Associated functions use a structure as a namespace, and don't have access to an *instance* of a type. Here's a constructor:

```rust
#[derive(Debug)]
struct MyStruct {
    a: i32
}

impl MyStruct {
    fn new(a: i32) -> Self {
        Self { a }
    }
}

fn main() {
    let my_struct = MyStruct::new(5);
    println!("{my_struct:#?}");
}
```

We're using the built-in helper `Self`---with a capital `S`---that refers to "the type I'm currently implementing". You can put the full type name in if you prefer.

A nice side effect is that you can have as many constructors as you want, and put anything you like in the namespace. Please only put related functions in the namespace---otherwise, finding things later can be *really* annoying.

## Methods

You can also define methods. For example:

```rust
#[derive(Debug)]
struct MyStruct {
    a: i32
}

impl MyStruct {
    fn new(a: i32) -> Self {
        Self { a }
    }

    fn get_a(&self) -> i32 {
        self.a
    }
}

fn main() {
    let my_struct = MyStruct::new(5);
    println!("{}", my_struct.get_a());
}
```

> Note that writing getters/setters isn't required at all. Some organizations like them, some don't.

## Arrays

Statically-sized arrays (no VLAs from C) are built-in:

```rust
fn main() {
    let array = [1, 2, 3, 4];
    let another_array = [0; 5]; // Repeat the first item a total of 5 times
    println!("{array:?}");
    println!("{another_array:?}");
}
```

What happens if you read outside of an array?

```rust
fn main() {
    let array = [0; 5];
    println!("{}", array[6]);
}
```

In this simple example, the compiler actually detects that it will fail at runtime and refuses to compile it! You can't count on it doing that. As soon as you add some complexity, LLVM won't spot the issue:

```rust
fn main() {
    let array = [0; 5];
    for i in 0..10 {
        println!("{}", array[i]);
    }
}
```

Note that the output is:

```
thread 'main' panicked at src/main.rs:4:24:
index out of bounds: the len is 5 but the index is 5
```

Rust detected the error at runtime and issued a panic, rather than segfaulting and performing potentially undefined behavior. Contrast this with the code from `cpp/array_bounds`:

```c++
#include <iostream>

int main() {
    int a[3] = {1, 2, 3};
    for (int i = 0; i < 10; i++) {
        std::cout << a[i] << std::endl;
    }
    return 0;
}
```

On my Linux workstation, it outputs:

```
1
2
3
-506178560
318461025
1
0
4925130
0
1651076199
```

That's not a good sign for security and memory safety!

## Vectors

Vectors in Rust are a lot like vectors in C++: they store a capacity and size, and a pointer to an area of contiguous heap memory. When capacity is exceeded, they double in size.

Here are some vector examples:

```rust
fn main() {
    // The `vec!` macro helps with assignment using array syntax
    let my_vec = vec![1, 2, 3, 4, 5];
    let my_vec = vec![0; 5];

    // `push_back` has become `push`
    let mut my_vec = Vec::new();
    my_vec.push(1);
    my_vec.push(2);

    println!("{my_vec:?}");
    println!("{}", my_vec[1]);
    println!("{:?}", my_vec.get(2)); // Safer access.
    println!("{}", my_vec[2]); // Safe: Panics but doesn't do anything dangerous
}
```

Again, the default accessor will panic rather than at-best segfaulting.

Now that we've covered a lot of basic Rust syntax, let's dive back into the ecosystem---and the promises of safety.