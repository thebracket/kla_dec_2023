# Embedded Challenges

There's a lot of different types of embedded out there. "Embedded" can mean a full-featured Raspberry PI 4---or a tiny microcontroller. Different platforms will have differing levels of support for embedded Rust. LLVM currently bounds which platforms you can target; Rust on GCC is advancing rapidly but isn't ready for production yet.

# Minimizing Binary Size

For size-constrained builds, Rust has a lot of options:

## Optimize for Size

In `Cargo.toml`, you can specify optimization levels by profile. Add this to the `Cargo.toml` file:

```toml
[profile.release]
opt-level = "s"
```

Run `cargo build --release`. It'll take a moment, it has to recompile every dependency and also optimize the dependency for size.

On Windows, the resulting binary is now: 510,976 bytes (499 kb). A small improvement.

There's also an optimization level named "z". Let's see if it does any better?

```toml
[profile.release]
opt-level = "z"
```

It weighs in at 509,440 bytes (497.5 kb). A very tiny improvement.

## Strip the binary

In `Cargo.toml`, let's also strip the binary of symbols.

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
```

Compiling again, this reduces the binary to 508,928 (497 kb).

## Enable LTO

In `Cargo.toml`, let's enable link-time optimization. This optimizes across crate boundaries, at the expense of a SLOW compile.

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
lto = true
```

We're down to 438,272 bytes (428 kb). Getting better!

## Reduce Codegen Units

By default, Rust parallelizes builds across all of your CPUs - which *can* prevent some optimizations. Let's make our compilation even slower in the name of a small binary:

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
lto = true
codegen-units = 1
```

You may have to run `cargo clean` before building this.

Our binary is now 425,472 bytes (415 kb). Another small improvement.

## Abort on Panic

A surprising amount of a Rust binary is the "panic handler". Similar to an exception handler in C++, it adds some hefty code to unwind the stack and provide detailed traces on crashes. We can turn this behavior off:

```toml
[profile.release]
opt-level = "z"
strip = true # Automatically strip symbols
lto = true
codegen-units = 1
panic = "abort"
```

This reduces my binary to 336,896 bytes (329 kb). That's a big improvement! The downside is that if your program panics, it won't be able to tell you all the details about how it died.

## Heavy Measures: Optimize the Standard Library for Size

If you don't have `nightly` installed, you will need it:

```bash
rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
```

Then find out your current build target:

```bash
rustc -vV
```

And use that target to issue a build that includes the standard library:

```bash
cargo +nightly build -Z build-std=std,panic_abort -Z build-std-features=panic_immediate_abort --target x86_64-apple-darwin --release
```

The binary goes into `target/(platform)/release`. There's a pretty substantial size improvement: 177,152 bytes (173 kb)

That's about as small as it gets without using a different standard library. Let's see what we can do about the dependencies.

## Using Cargo Bloat

Install a tool, `cargo install cargo-bloat`. And run `cargo bloat` to see exactly where your binary size is going.

# Building Without the Standard Library

If you are on a platform without standard library support (or for really small builds), you can combine these steps with adding `#[no_std]` to your binary. You can still opt-in to parts of the library with `core::`---depending upon what is available. This can also be useful for WASM builds in the browser. You can also use `extern crate alloc` to opt-in to a Rust-provided allocator:

```rust
#![no_std]
extern crate alloc;
```

This allows you to use `Vec` and similar in your code. You don't have the full standard library, but it's a pretty pleasant environment.

# Using a Different Allocator

Rust defaults to using your platform's allocator. It used to use `jemallocator`, but that didn't work properly on all platforms. Jem is amazing---it offers memory usage profiling, a pool-based system that minimizes the penalty for reallocation, and can improve the performance of real-time sytems significantly. The LibreQoS project adopted it for real-time packet analysis, and saw runtime performance improvements up to 15%.

To opt-in to `jemalloc`, add the following to `Cargo.toml`:

```toml
[target.'cfg(any(target_arch = "x86", target_arch = "x86_64"))'.dependencies]
jemallocator = "0.5"
```

And add this to your `main.rs` file (outside of any functions):

```rust
// Use JemAllocator only on supported platforms
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
use jemallocator::Jemalloc;

#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;
```

The rest of the Rust system will pickup on these changes and use Jem. There are quite a few other allocation systems available.