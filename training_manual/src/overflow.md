# Overflow and Wrapping

C++ assumes wrapping, but undefined behavior (which allows for some great compiler optimizations, but is also confusing). Rust considers wrapping and overflow to be well-defined, but behavior you should specify.

You're probably used to this behavior from C++ (it's in the `cpp/byte_overflow` directory):

```c++
#include <iostream>
#include <cstdint>

int main() {
    uint8_t j = 0;
    for (int i = 0; i < 512; i++) {
        j++;
        std::cout << i << " : " << unsigned(j) << std::endl;
    }
    return 0;
}
```

This outputs `0` to `255` twice.

In Rust, the same program:

```rust
fn main() {
    let mut j: u8 = 0;
    for i in 0..512 {
        j += 1;
        println!("{i} : {j}");
    }
}
```

Running the program panics---crashes. It gives the error message "attempt to add with overflow".

> Note that running in release mode (`cargo run --release`) skips his run-time check for performance. It's a great idea to run in debug mode sometimes.

## Opting in to Wrapping

If your algorithm expects wrapping behavior, the easiest option is to use the `wrapping_add` function. That makes it clear that you *expect* wrapping, and acts appropriately:

```rust
fn main() {
    let mut j: u8 = 0;
    for i in 0..512 {
        j = j.wrapping_add(1);
        println!("{i} : {j}");
    }
}
```

If you'd just like to detect that wrapping would have occurred, you can use:

```rust
fn main() {
    let mut j: u8 = 0;
    for i in 0..512 {
        j = j.checked_add(1).unwrap(); // Returns `None` or `Some(result)`
        println!("{i} : {j}");
    }
}
```

This program will crash even on release-mode, because we've used `unwrap` - which deliberately panics on an error. You could detect the problem, choose how to handle it, and not crash.

You can use `saturating` functions also:

```rust
fn main() {
    let mut j: u8 = 0;
    for i in 0..512 {
        j = j.saturating_add(1);
        println!("{i} : {j}");
    }
}
```

If you don't like the extra typing, and want to associate a behavior with a type, you can use saturating and wrappign types.

```rust
use std::num::Wrapping;

fn main() {
    let mut j: Wrapping<u8> = Wrapping(0);
    for i in 0..512 {
        j += Wrapping(1);
        println!("{i} : {j}");
    }
}
```

Rust is being very explicit about behavior, because surprises are a bad thing!