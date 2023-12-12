# Create Your First Thread

> This uses the `first_thread` code, in `code/02_threads`.

## Create a new project - with a workspace

Looking back at the [workspaces](../01-GettingStarted/Workspaces.md) class from last week, it's a great idea to have a workspace. Let's create one:

```bash
cargo new LiveWeek2
```

Now edit `Cargo.toml` to include a workspace:

```toml
[workspace]
members = []
```

Now change directory to the `LiveWeek2` directory and create a new project named `FirstThread`:

```bash
cd LiveWeek2
cargo new FirstThread
```

And add the project to the workspace:

```toml
[workspace]
members = [
    "FirstThread"
]
```

## Your First Thread

In `main.rs`, replace the contents with the following:

```rust
fn hello_thread() {
    println!("Hello from thread!");
}

fn main() {
    println!("Hello from main thread!");

    let thread_handle = std::thread::spawn(hello_thread);
    thread_handle.join().unwrap();
}
```

Now run the program:

```bash
Hello from main thread!
Hello from thread!
```

So what's going on here? Let's break it down:

1. The program starts in the main thread.
2. The main thread prints a message.
3. We create a thread using `std::thread::spawn` and tell it to run the function `hello_thread`.
4. The return value is a "thread handle". You can use these to "join" threads---wait for them to finish.
5. We call `join` on the thread handle, which waits for the thread to finish.

### What happens if we don't join the thread?

Run the program a few times. Sometimes the secondary thread finishes, sometimes it doesn't. Threads don't outlive the main program, so if the main program exits before the thread finishes, the thread is killed.

# Spawning Threads with Parameters

> This uses the `thread_closures` code, in `code/02_threads`.

The `spawn` function takes a function without parameters. What if we want to pass parameters to the thread? We can use a closure:

```rust
fn hello_thread(n: u32) {
    println!("Hello from thread {n}!");
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0 .. 5 {
        let thread_handle = std::thread::spawn(move || hello_thread(i));
        thread_handles.push(thread_handle);
    }
    thread_handles.into_iter().for_each(|h| h.join().unwrap());
}
```

Notice three things:

* We're using a *closure*---an inline function that can capture variables from the surrounding scope.
* We've used the shorthand format for closure: `|| code` - parameters live in the `||` (there aren't any), and a single statement goes after the `||`. You can use complex closures with a scope: `|x,y| { code block }`.
* The closure says `move`. Remember when we talked about ownership? You have to *move* variables into the closure, so the closure gains ownership of them. The ownership is then passed to the thread. Otherwise, you have to use some form of synchronization to ensure that data is independently accessed---to avoid race conditions.

The output will look something like this (the order of the threads will vary):

```
Hello from thread 0!
Hello from thread 2!
Hello from thread 1!
Hello from thread 4!
Hello from thread 3!
```

In this case, as we talked about last week in [Rust Fundamentals](../01-GettingStarted/RustFundamentals.md) integers are *copyable*. So you don't have to do anything too fancy to share them.

# Returning Data from Threads

> See the code `thread_return` in `code/02_threads`.

The thread handle will return any value returned by the thread. It's generic, so it can be of any type (that supports sync+send; we'll cover that later). Each thread has its own stack, and can make normal variables inside the thread---and they won't be affected by other threads.

Let's build an example:

```rust
fn do_math(i: u32) -> u32 {
    let mut n = i+1;
    for _ in 0 .. 10 {
        n *= 2;
    }
    n
}

fn main() {
    let mut thread_handles = Vec::new();
    for i in 0..10 {
        thread_handles.push(std::thread::spawn(move || {
            do_math(i)
        }));
    }

    for handle in thread_handles {
        println!("Thread returned: {}", handle.join().unwrap());
    }
}
```

This returns:

```
Thread returned: 1024
Thread returned: 2048
Thread returned: 3072
Thread returned: 4096
Thread returned: 5120
Thread returned: 6144
Thread returned: 7168
Thread returned: 8192
Thread returned: 9216
Thread returned: 10240
```

Notice that each thread is doing its own math, and returning its own value. The `join` function waits for the thread to finish, and returns the value from the thread.