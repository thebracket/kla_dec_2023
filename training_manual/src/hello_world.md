# A Quick Hello World

Let's do a quick exercise. This is very simple, and you've probably already done this---we'll make `Hello World` and take a quick look at it. This will ensure that you have a working Rust installation. Then we'll compare it to a C++20 equivalent.

> Source code for this section is in `projects/part2/hello_world`.

## Step 1: Select a Parent Directory

Create a directory on your computer in which you will be placing projects we work on. We'll be placing projects underneath this directory. For example:

```bash
cd /home/herbert/rust
mkdir kla_dec_2023_live
cd kla_dec_2023_live
```

## Step 2: Invoke `cargo` to Create a New Project

In your project directory, type:

```bash
cargo new hello_world
```

## Step 3: Run Your Program!

Cargo creates `hello world` by default when you create a new project. It's all written! Invoke it by typing:

```bash
cargo run
```

You should see the following output:

```
   Compiling hello_world v0.1.0 (/home/herbert/Documents/Ardan/KLA Training 3 Day Milipitas/projects/hello_world)
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `/home/herbert/Documents/Ardan/KLA Training 3 Day Milipitas/target/debug/hello_world`
Hello, world!
```

### If This Didn't Work!

*If This Didn't Work*, your Rust tooling isn't working. Some troubleshooting steps:

1. Make sure that you installed Rust from [rustup.rs](https://rustup.rs/) and either followed the "source" instruction or opened a new terminal session.
2. Make sure that typing `rustc --version` shows you a version number.
3. If you received the message `linker not found`, you need to install `build-essential` (on Ubuntu/Debian type distributions) or an equivalent on Linux. If you are using MacOS, you need the build package. On Windows, Rustup will show you a link to the appropriate runtime.

## What Did Cargo Do?

Cargo has created several files in your project folder:

|Filename|Description|
|--|--|
|hello_world/Cargo.toml|The build manifest. Equivalent to a CMake file or Makefile.|
|hello_world/src/|The source directory.|
|hello_world/src/main.rs|The main source code file. Every executable needs a `main.rs` file (libraries have a `lib.rs`)---you can override this, but it's a good default.|

### Cargo.toml

Rust has created a `Cargo.toml` file for you:

```toml
[package]
name = "hello_world"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

* The `[package]` section defines the program itself. 
    * The `name` will be the name of the emitted executable (with `.exe` on the end for Windows). 
    * `version` uses semantic versioning. When referring to versions, "0" is special - every bump counts as a release. Once you hit 1.0, the dependency checker is a bit more strict. We'll talk about that later.
    * `edition` tells the Rust compiler which edition of the language you are using. Rust promises not to break language compatibility except when the edition number increases (roughly every 2-4 years). `rustc` retains compatibility with previous editions, unless a dangerous security reason appeared to remove something. This is designed to avoid the C++ difficulty of "we can never take away features" and "we can never change the interface".
* The `dependencies` section determines dependent packages to download. We'll worry about that later.

### main.rs

The `main.rs` file is a basic "Hello, world!" program:

```rust
fn main() {
    println!("Hello, world!");
}
```

If you've never seen Rust before, it might be a little confusing.

* `fn` is "function". Unlike C++, it doesn't specify the return type---just that it is a function.
* `main` is the name of the function. `main` is special, just like C++ --- it's the default invocation point for an executable program.
* `println!` has an exclamation mark, indicating that its a *macro*. Formatting strings is a pretty big job---see the C++20 format system! Rust's formatting system uses the macro system to allow for extreme flexibility for parameters. It's very powerful, but it's also a poor example for the first thing you see because macros are not an introductory topic.

## Equivalent C++

> C++ source code for this project is in `cpp/hello_world`.

A simple C++ equivalent program is as follows:

```c++
#include <iostream>

int main() {
    std::cout << "Hello, World!" << std::endl;
    return 0;
}
```

> Not everyone likes `iostream`. If you prefer `printf` or any of the other output systems, that's cool too.

It is accompanied by a CMakeLists.txt file:

```haskell
cmake_minimum_required(VERSION 3.5)
project(HelloWorld)

add_executable(hello hello.cpp)
```

And you can build the project and execute it with the following:

```bash
# First time only
mkdir build
cd build
cmake ..

# And then
cd build
make
./hello
```

This will give you the expected output: `Hello, World!`.

### Comparing Cargo.toml and CMake

The `Cargo.toml` and `CMakeLists.txt` files are similar: you specify the project details, and `CMake` builds the builder for you. Rust is doing a few more things:

* Your executable is statically linked, and includes the Rust standard library (that's why the executable is so much larger).
* `Cargo` includes dependency management, so your CMake file really includes `vcpkg`, `Conan` or one of the other build tools.
* `Cargo` doesn't offer to create makefiles, Ninja build systems, etc. --- it's an all in one tool.

So in reality, your `CMakeLists.txt` file would be a bit bigger:

```haskell
cmake_minimum_required(VERSION 3.5)
project(HelloWorld)

# Example vcpkg (commented out because I don't have it installed)
#set(CMAKE_TOOLCHAIN_FILE ~/vcpkg/scripts/buildsystems/vcpkg.cmake CACHE FILEPATH "Path to toolchain")

# Example static linking
set(CMAKE_EXE_LINKER_FLAGS "-static-libgcc -static-libstdc++ -static")

add_executable(hello hello.cpp)
```

### Comparing main.rs with hello.cpp

The files are quite similar. 

1. C++ brings in `iostream` with `#include <iostream>`. You don't need to do that for `println!`---Rust makes it available by default.
2. Both define a main function. `fn main()` and `int main()` are almost equivalent---but the Rust version doesn't return anything.
3. `println!` and `std::cout << "Hello, World!" << std::endl;` are equivalent. `println!` adds a `\n` to the end for you, which triggers a flush (`println!` is unbuffered). If you want to not emit a `\n`, you can use `print!`.
4. `return 0` returns an exit code of 0. Rust programs do that for you by default.

So despite the syntax being different, it's not all that different.

If you *really* want to be returning an exit code, you can use the following:

```rust
use std::process:ExitCode;

fn main() -> ExitCode {
    println!("Hello, world!");
    return ExitCode::from(0);
}
```

And that concludes our quick "Hello World" tour. We've covered:

* How to create a Rust program with Cargo.
* What the resulting files and structure cover.
* An equivalent C++ and CMake setup.
* Exit codes.
