# Functions, Variables and Scopes

> Let's keep hacking at our Hello World project, and play around a bit to get used to Rust's handling of language concepts.

Let's take a look at some of the syntax differences you'll encounter.

## Functions and Variables

Rust and C++ both use lots of functions! They work in a very similar fashion, but the syntax is quite different.

Here's a really simple Rust function example:

> The source code for this is in `projects/part2/double_fn`.

```rust
fn double_it(n: i32) -> i32 {
    n * 2
}

fn main() {
    let i = 5;
    let j = double_it(i);
    println!("{i} * 2 = {j}");
}
```

Let's go through line-by-line quickly:

* `fn double_it` declares a function named `double_it`
    * `(n: i32)` declares a function argument/parameter named `n`, of the type `i32`. Rust uses explicit sizing for variable types except for `usize` and `isize`---which work exactly like `usize` in C++ (the size of a pointer on your platform)
    * `-> i32` indicates that the function returns an `i32`.
* `n * 2` returns `n`, multiplied by 2. Note that there's no `;`. If you don't specify a semicolon, you are returning the result of an expression.
* `fn main` is the main function.
* `let i = 5;` creates a new variable binding named `i` and assigns the value `5` to it. You don't need to specify a type, because the Rust compiler can infer it from the fact that you used `i` with a function call. (It will default to `i32` if there are no clues).
* `let j = double_it(i)` calls `double_it` with the value of `i` (we'll talk about copy, reference and move later), and assigns the result to `j`.
* `println!("{i}` is using the Rust format macro to add the value of `i` into the output string. You can use named variables but not expressions inside the format string. If you prefer, `println!("{} * 2 = {}", i, j)` is also valid. You can replace the `j` with a direct call to `double_it` if you prefer.

Here's some equivalent C++, using modern C++ equivalents:

> The source code is in `cpp/double_fn`

```c++
#include <iostream>

int double_it(int x) {
    return x * 2;
}

int main() {
    auto i = 5;
    auto j = double_it(i);
    std::cout << i << " * 2 = " << j << std::endl;
    return 0;
}
```

It's very similar. Most of the concepts are the same. Variable declarations are the other way around, function declarations declare their return type first. Overall, though---you shouldn't have too much trouble getting used to the new way of arranging things.

## Primitive Types

Rust is a *lot* more strict than C++ defaults about coercing types. Take the following C++ (it's in `cpp/type_changes`):

```c++
#include <iostream>

int double_it(long n) {
    return n * 2;
}

int main() {
    int i = 5;
    int j = double_it(i);
    std::cout << "i = " << i << ", j = " << j << std::endl;
    return 0;
}
```

The project compiles without warnings or errors, and outputs `i = 5, j = 10` as you'd expect.

Let's do a line-by-line conversion to Rust:

```rust
fn double_it(n: i64) -> i32 {
    n * 2
}

fn main() {
    let i: i32 = 5;
    let j: i32 = double_it(i);
    println!("i = {i}, j = {j}");
}
```

The Rust project fails to compile. The error message is:

```
error[E0308]: mismatched types
 --> src/main.rs:2:5
  |
1 | fn double_it(n: i64) -> i32 {
  |                         --- expected `i32` because of return type
2 |     n * 2
  |     ^^^^^ expected `i32`, found `i64`
  |
help: you can convert an `i64` to an `i32` and panic if the converted value doesn't fit
  |
2 |     (n * 2).try_into().unwrap()
  |     +     +++++++++++++++++++++

error[E0308]: mismatched types
 --> src/main.rs:7:28
  |
7 |     let j: i32 = double_it(i);
  |                  --------- ^ expected `i64`, found `i32`
  |                  |
  |                  arguments to this function are incorrect
  |
note: function defined here
 --> src/main.rs:1:4
  |
1 | fn double_it(n: i64) -> i32 {
  |    ^^^^^^^^^ ------
help: you can convert an `i32` to an `i64`
  |
7 |     let j: i32 = double_it(i.into());
  |                             +++++++
```

The error message helpfully tells you how to fix the program, but the key here is that `i32` and `i64` are *not the same type*, so you **can't** pass one as the other.

### Converting Types

> If you see a lot of these error messages, it's a code smell. That is code that may not be such a great idea! Try to settle on types that are appropriate for what you are doing.

You actually have a few options for type conversion. 

#### Converting with `as`

The first is `as`:

```rust
fn double_it(n: i64) -> i32 {
    n as i32 * 2
}

fn main() {
    let i: i32 = 5;
    let j: i32 = double_it(i as i64);
    println!("i = {i}, j = {j}");
}
```

`as` works, but it is the *least safe* option. `as` does a direct conversion, ignoring any overflow, data-loss, or precision loss. It's always safe to go from `i32` to `i64`---you can't lose any data. Going from `i64` to `i32` may not be what you intended:

```rust
fn main() {
    let i: i64 = 2_147_483_648; // One more than i32 can hold
    let j = i as i32;
    println!("{j}");
}
```

You probably guessed that the result is `-2147483648`...

Takeaway: you can use `as` for safe conversions, it's not always the best idea.

#### Using `into`

The compiler error messages suggest using `into`. `into` is *only* provided for conversions where the type-conversion is safe and won't lose your data. We could use it like this:

```rust
fn double_it(n: i64) -> i32 {
    n as i32 * 2
}

fn main() {
    let i: i32 = 5;
    let j: i32 = double_it(i.into());
    println!("i = {i}, j = {j}");
}
```

This works, but we're still using `n as i32`. Why? `i64` to `i32` conversion can lose data---so Rust doesn't implement `into()`. Still, we're half way there.

#### Using `try_into`

For fallible conversions, Rust provides the `try_into` operation:

```rust
use std::convert::TryInto;

fn double_it(n: i64) -> i32 {
    let n: i32 = n.try_into().expect("{n} could not be converted safely into an i32");
    n * 2
}

fn main() {
    let i: i32 = 5;
    let j: i32 = double_it(i.into());
    println!("i = {i}, j = {j}");
}
```

`try_into` returns a `Result` type. We're going to go into those in detail later. For now, just think of it as equivalent to `std::expected`---it's either the expected result, or an error. You can use `unwrap()` to crash immediately on an error, or `expect` to crash with a nicer error message. There's lots of *good* ways to handle errors, too---but we'll get to that later.

Yes, that's a lot more pedantic. On the other hand, Rust makes you jump through a few hoops before you are confused by:

```c++
std::cout << double_it(2147483648) << std::endl;
```

> It outputs 0


## Expressions, Scopes and Return Values

Rust and C++ are both scope-heavy languages. Rust borrows its scope concept from O'Caml, and this tends to make idiomatic Rust code a little different. Any non-semicolon line in a scope is an implicit return. These are the same:

```rust
fn func1(n: i32) -> i32 { n }
fn func2(n: i32) -> i32 {
    return n;
}

fn main() {
    println!("{}, {}", func1(5), func2(5));
}
```

You can return out of scopes, too. But you can't use the `return` keyword, because that is setup to return from the function. This works:

```rust
fn main() {
    let i = {
        5
    };

    println!("{i}");
}
```

This doesn't:

```rust
fn main() {
    let i = {
        return 5;
    };

    println!("{i}");
}
```

Even functions that don't return anything, actually return the *unit type* (expressed as `()`):

```rust
fn main() {
    let i = println!("Hello");
    println!("{i:?}");
}
```

> The `:?` in the format specifier means "debug format". Any time that implements a trait named `Debug`---we'll cover those later---can be debug-formatted in this fashion. You can also use `:#?` to pretty-print.

The result of allowing scopes and expressions to return is that you can have conditional assignment (there's no ternary assignment in Rust):

```rust
fn main() {
    const n: i32 = 6;
    let i = if n == 6 {
        5
    } else {
        7
    };
    println!("{i}");
}
```