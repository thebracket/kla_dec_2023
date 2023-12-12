# Workspaces, Crates, Programs, Libraries and Modules

Let's talk about some terminology:

* A `crate` is a Rust package. It can either be a program or a library---it's a package of code managed by Cargo.
* A `program` is an executable program. A `crate` produces a program if it has a `main.rs` file, and usually a `main` function (you can change the main function name, but it does need an entry point)
* A `library` is a crate with a `lib.rs` file. It compiles as a static library by default, you can override this if you need dynamic libraries (Rust is very much oriented towards self-contained statically linked systems).
* A `module` is a unit-of-work for the compiler. Programs and libraries are divided into modules.
* A `workspace` is a Cargo helper that lets you include multiple crates in one environment with a shared compilation target directory and better incremental compilation.

This is quite unlike C++'s system. `#include` is almost a cut-and-paste; the new C++20 modules system is a bit more similar--but I had troubles getting it to work consistently across platforms.

## Workspaces

The example code uses a workspace, and I'd encourage you to do the same. Workspaces are a great mechanism for storing related code together.

Let's create a workspace.

1. `cd` to your parent directory.
2. Create a new Rust project with `cargo new my_workspace`.
3. `cd` into `my_workspace`.
4. Edit `src/main.rs` to change "Hello, World!" to something like "You probably intended to run a workspace member". This is optional, but helps avoid confusion.
5. While in `my_workspace`, create a new project. `cargo new hello`.
6. Edit `my_workspace/Cargo.toml`:

```toml
[workspace]
members = [ "hello" ]
```

Now change directory to `my_workspace/hello` and run the program with `cargo run`.

Take a look at `my_workspace` and you will see that a `target` directory has appeared. Within a workspace, all compiler artifacts are shared. For large projects, this can save a huge amount of disk space. It can also save on re-downloading dependencies, and will only recompile portions of the workspace that have changed.

> While working on Hands-on Rust, I initially had 55 projects in separate crates without a workspace. I noticed that my book's `code` folder was using nearly 6 gigabytes of disk space, which was crazy. So I added a workspace, and that shrunk to a few hundred megabytes. Every single project was downloading all of the dependencies and building them separately.

Workspaces are safe to upload to `github` or your preferred Git repo. You can even access dependencies within a workspace remotely (we'll cover that in [dependencies](./dependencies.md)). 

# Libraries

Let's workshop through creating our first library. Keep the `my_workspace` and `hello` projects.

Change directory back to the workspace root (`my_workspace/`). Create a new library project;

```bash
cargo new hello_library --lib
```

> Notice the `--lib` flag. You are creating a library.

Open `my_workspace/Cargo.toml` and add `hello_library` as a workspace member:

```toml
[workspace]
members = [ "hello", "hello_library" ]
```

Now open `hello_library/src/lib.rs`. Notice that Rust has auto-generated an example unit test system. We'll cover that in [unit tests](./unit_tests.md) shortly. For now, delete it all and replace with the following code:

```rust
pub fn say_hello() {
    println!("Hello, world!");
}
```

The `pub` marks the function as "public"---available from outside the current module. Since it is in `lib.rs`, it will be exported in the library.

Now open `hello/Cargo.toml` and we'll add a dependency:

```toml
[dependencies]
hello_libary = { path = "../hello_library" }
```

And open `hello/src/main.rs` and we'll use the dependency. Replace the default code with:

```rust
use hello_library::say_hello;

fn main() {
    say_hello();
}
```

Congratulations! You've made your first statically linked library.

## Modules and Access

Rust can subdivide code into *modules*, which can both be and contain `public` and `private` (private being the default). Coming from C++, I found this a little confusing. You can also create modules in-place (as namespaces) or in separate files. This can be confusing, so let's work through some examples.

### Inline Module (Namespace)

Open `hello_library/src/lib.rs`. Let's add a private module:

```rust
mod private {
    fn hi() {
        println!("Say Hi!");
    }
}

pub fn say_hello() {
    println!("Hello, world!");
}
```

If you try to use `private::hi()` in your `hello/src/main.rs` program---it won't work. The module and the function are both private:

```rust
use hello_library::say_hello;

fn main() {
    say_hello();
    say_hello_library::private::hi(); // Will not compile
}
```

You can fix this by changing the module to be public:

```rust
pub mod private {
    fn hi() {
        println!("Say Hi!");
    }
}

pub fn say_hello() {
    println!("Hello, world!");
}
```

And it still doesn't work! That's because making a module *public* only exposes the *public* members of the module. So you *also* need to decorate the function as public:

```rust
pub mod private {
    pub fn hi() {
        println!("Say Hi!");
    }
}

pub fn say_hello() {
    println!("Hello, world!");
}
```

So that allows you to make a public namespace---and include private parts in the namespace that aren't exposed to the world. What if you want to *write* a function in a module, and expose it in a different namespace?

```rust
pub mod private {
    pub fn hi() {
        println!("Say Hi!");
    }
}

pub use private::hi;

pub fn say_hello() {
    println!("Hello, world!");
}
```

The `use` statement---importing something into the current namespace---can also be decorated with `pub` to re-export that import. You can use this with dependencies or your modules. (It's common to make a `prelude` module and import all of the most-likely to be useful functions and types into it for re-rexport). Now your program can refer to `hello_library::hi` directly.

## File-based modules

If you're working in a team, it's usually a good idea to not all be trying to edit the same file at once. There are other advantages to using multiple files:

* Rust can compile multiple files at the same time.
* Organizing your code with files makes it a lot easier to find things.
* You can use conditional compilation to include different files based on compilation constraints.

Let's make a one-file module. In `hello_library/src` create a new file named `goodbye.rs`. In that file, write:

```rust
pub fn bye() {
    println!("Goodbye");
}
```

Simply having the file doesn't make it do anything, or part of your project. In `hello_library/src/lib.rs` add a line to include the module:

```rust
mod goodbye;
```

The module is now private, even though the `bye` function is public! You will be able to access `bye` elsewhere in your library, but not from consumer applications. You can use the same mechanisms as for inline modules to change that. `pub mod` exports it as a `hello_library::goodbye` (the filename is the namespace). Or you can `pub use goodbye::bye`.

## Directory modules

The final type of module places the module in a directory. The directory must contain a `mod.rs` file to act as the module root---and can include other files or inline modules as above.

Create a new directory, `hello_library/src/dirmod`. In that directory, create `mod.rs`:

```rust
pub fn dir_hello() {
    println!("Hello from dir module");
}
```

Now in `hello_library/src/lib.rs` include the new module:

```rust
pub mod dirmod;
```

You can now access the module in your `hello` project, with `hello_library::dirmod::dir_hello()`.