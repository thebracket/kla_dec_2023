# Data-Race Protection

Rust makes the bold claim that it offers "fearless concurrency" and no more data-races (within a program; it can't do much about remote calls). That's a very bold claim, and one I've found to be true so far---I'm much more likely to contemplate writing multi-threaded (and async) code in Rust now that I understand how it prevents me from shooting myself in the foot.

## An Example of a Data Race

Here's a little modern C++ program with a very obvious data-racing problem (it's in the `cpp/data_race` directory):

```c++
#include <thread>
#include <iostream>

int main() {
    int counter = 0;
    std::thread t1([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    std::thread t2([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}
```

The program compiled and ran without any warnings (although additional static analysis programs would probably flag this).

The program fires up two threads. Each loops, incrementing a counter. It joins the threads, and prints the result. The predictable result is that every time I run it, I get a different result: 1015717, 1028094, 1062030 from my runs.

This happens because incrementing an integer isn't a single-step operation:

1. The CPU loads the current counter value, into a register.
2. The CPU increments the counter.
3. The CPU writes the counter back into memory.

There's no guaranty that the two threads won't perform these operations while the other thread is also doing part of the same operation. The result is data corruption.

Let's try the same thing in Rust. We'll use "scoped threads" (we'll be covering threading in a later session) to make life easier for ourselves. Don't worry about the semantics yet:

```rust
fn main() {
    let mut counter = 0;
    std::thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter += 1;
            }
        });
        let t2 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter += 1;
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    println!("{counter}");
}
```

And now you see the beauty behind the "single mutabile access" rule: the borrow checker prevents the program from compiling, because the threads are mutably borrowing the shared variable. No data race here!

## Atomics

If you've used `std::thread`, you've probably also run into atomic types. An atomic operation is guaranteed to be completed in one CPU operation, and optionally be synchronized between cores. The following C++ program makes use of an `std::atomic_int` to always give the correct result:

```c++
#include <thread>
#include <iostream>
#include <atomic>

int main() {
    std::atomic_int counter = 0;
    std::thread t1([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    std::thread t2([&counter]() {
        for (int i = 0; i < 1000000; ++i) {
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}
```

Rust gives you a similar option:

> This code is in `projects/part2/atomics`

```rust
use std::sync::atomic::Ordering::Relaxed;
use std::sync::atomic::AtomicU32;

fn main() {
    let counter = AtomicU32::new(0);
    std::thread::scope(|scope| {
        let t1 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
        let t2 = scope.spawn(|| {
            for _ in 0 .. 1000000 {
                counter.fetch_add(1, Relaxed);
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    println!("{}", counter.load(Relaxed));
}
```

So Rust and C++ are equivalent in functionality. Rust is a bit more pedantic---making you specify the ordering (which are taken from the C++ standard!). Rust's benefit is that the unsafe version generates an error---otherwise the two are very similar.

## Why Does This Work?

So how does Rust know that it *isn't* safe to share an integer---but it is safe to share an atomic? Rust has two traits that self-implement (and can be overridden in unsafe code): `Sync` and `Send`.

* A `Sync` type can be modified - it has a *synchronization* primitive.
* A `Send` type can be sent between threads - it isn't going to do bizarre things because it is being accessed from multiple places.

A regular integer is neither. An Atomic integer is both.

Rust provides atomics for all of the primitive types, but does *not* provide a general Atomic wrapper for other types. Rust's atomic primitives are pretty much a 1:1 match with CPU intrinsics, which don't generally offer sync+send atomic protection for complicated types.

## Mutexes

If you want to provide similar thread-safety for complex types, you need a Mutex. Again, this is a familiar concept to C++ users.

Using a Mutex in C++ works like this:

```c++
#include <iostream>
#include <thread>
#include <mutex>

int main() {
    std::mutex mutex;
    int counter = 0;
    std::thread t1([&counter, &mutex]() {
        for (int i = 0; i < 1000000; ++i) {
            std::lock_guard<std::mutex> guard(mutex);
            ++counter;
        }
    });
    std::thread t2([&counter, &mutex]() {
        for (int i = 0; i < 1000000; ++i) {
            std::lock_guard<std::mutex> guard(mutex);
            ++counter;
        }
    });
    t1.join();
    t2.join();

    std::cout << counter << std::endl;

    return 0;
}
```

Notice how using the Mutex is a two-step process:

1. You declare the mutex as a separate variable to the data you are protecting.
2. You create a `lock_guard` by initializing the lock with `lock_guard`'s constructor, taking the mutex as a parameter.
3. The lock is automatically released when the guard leaves scope, using RAII.

This works, and always gives the correct result. It has one inconvenience that can lead to bugs: there's no enforcement that makes you remember to use the lock. You can get around this by building your own type and enclosing the update inside it---but the compiler won't help you if you forget. For example, commenting out one of the mutex locks won't give any compiler warnings.

Let's build the same thing, in Rust. The Rust version is a bit more complicated:

> This code is in `projects/part2/mutex`

```rust
use std::sync::{Arc, Mutex};

fn main() {
    let counter = Arc::new(Mutex::new(0));
    std::thread::scope(|scope| {
        let my_counter = counter.clone();
        let t1 = scope.spawn(move || {
            for _ in 0 .. 1000000 {
                let mut lock = my_counter.lock().unwrap();
                *lock += 1;
            }
        });

        let my_counter = counter.clone();
        let t2 = scope.spawn(move || {
            for _ in 0 .. 1000000 {
                let mut lock = my_counter.lock().unwrap();
                *lock += 1;
            }
        });
        let _ = t1.join();
        let _ = t2.join(); // let _ means "ignore" - we're ignoring the result type
    });
    let lock = counter.lock().unwrap();
    println!("{}", *lock);
}
```

Let's work through what's going on here:

1. `let counter = Arc::new(Mutex::new(0));` is a little convoluted.
    1. Mutexes in Rust *wrap* the data they are protecting, rather than being a separate entity. This makes it impossible to forget to lock the data---you don't have access to the interior without obtaining a lock.
    2. `Mutex` only provides the `Sync` trait---it can be safely accessed from multiple locations, but it doesn't provide any safety for sending the data between threads.
    3. To gain the `Send` trait, we *also* wrap the whole thing in an `Arc`. `Arc` is "atomic reference count"---it's just like an `Rc`, but uses an atomic for the reference counter. Using an `Arc` ensures that there's only a single counter, with safe access to it from the outside.
    4. Note that `counter` isn't mutable---despite the fact that it is mutated. This is called *interior mutability*. The exterior doesn't change, so it doesn't have to be mutable. The *interior* can be changed via the `Arc` and the `Mutex`---which is protected by the `Sync+Send` requirement.
2. Before each thread is created, we call `let my_counter = counter.clone();`. We're making a clone of the `Arc`, which increments the reference count and returns a shared pointer to the enclosed data. `Arc` is designed to be cloned every time you want another reference to it.
3. When we start the thread, we use the `let t1 = scope.spawn(move || {` pattern. Notice the *move*. We're telling the closure *not* to capture references, but instead to move captured variables into the closure. We've made our own clone of the `Arc`, and its the only variable we are referencing---so it is *moved* into the thread's scope. This ensures that the borrow checker doesn't have to worry about trying to track access to the same reference across threads (which won't work). `Sync+Send` protections remain in place, and it's impossible to use the underlying data without locking the mutex---so all of the protections are in place.
4. `let mut lock = my_counter.lock().unwrap();` locks the mutex. It returns a `Result`, so we're unwrapping it (we'll talk about why later). The lock itself is mutable, because we'll be changing its contents.
5. We access the interior variable by dereferencing the lock: `*lock += 1;`

So C++ wins slightly on ergonomics, and Rust wins on preventing you from making mistakes!

## Summary

Rust's data race protection is very thorough. The borrow-checker prevents multiple mutable accesses to a variable, and the `Sync+Send` system ensures that variables that are accessed in a threaded context can both be sent between threads and safely mutated from multiple locations. It's extremely hard to create a data race in safe Rust (you can use the `unsafe` tag and turn off protections if you need to)---and if you succeed in making one, the Rust core team will file it as a bug.

All of these safety guarantees add up to create an environment in which common bugs are hard to create. You do have to jump through a few hoops, but once you are used to them---you can fearlessly write concurrent code knowing that Rust will make the majority of multi-threaded bugs a compilation error rather than a difficult debugging session.
