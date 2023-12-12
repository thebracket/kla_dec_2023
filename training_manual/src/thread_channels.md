# Sending Data Between Threads with Channels

[Parking a thread](./ParkingThreads.md) is great, but you often need to tell a thread *why* you woke it up, or give it some data to work with. This is where channels come in.

If you're used to Go, channels should sound familiar. They are very similar to Go's channels. A few differences:

* Rust Channels are strongly typed. So you can use a sum type/enum to act like a command pattern.
* Rust Channels are bounded by size, and will block if you try to send data to a full channel.
* Rust Channels are unidirectional. You can't send data back to the sender. (You can make another channel)
* You can't forget to close a channel. Once a channel is out of scope, the "drop" system (we'll talk about that in a couple of weeks) will close the channel for you.

## Multi-Producer, Single Consumer Channels

> See the `mpsc` project in the `code/02_threads` directory.

The most basic type of channel is the MPSC channel: any number of producers can send a message to a single consumer. Let's build a simple example:

```rust
use std::sync::mpsc;

enum Command {
    SayHello, Quit
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();

    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::SayHello => println!("Hello"),
                Command::Quit => {
                    println!("Quitting now");
                    break;
                }
            }
        }
    });

    for _ in 0 .. 10 {
        tx.send(Command::SayHello).unwrap();
    }
    println!("Sending quit");
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
```

This is a relatively simple example. We're only sending messages to one thread, and not trying to send anything back. We're also not trying to send anything beyond a simple command. But this is a great pattern---you can extend the `Command` to include lots of operations, and you can send data along with the command. Threads can send to other threads, and you can `clone` the `tx` handle to have as many writers as you want.

We're going to build on the channel system after the break.

# Channels and Ownership

Channels are an easy way to send data between threads, but ownership becomes a question.

Trying to pass a reference into a channel becomes problematic fast. Unless you can guarantee that the calling thread will outlive the data---and retain the data in a valid state---you can't pass a reference. The "lifetime checker" part of the borrow checker will complain.

The easiest approach is to **move** the data. The data arrives in one thread, which owns it. Rather than cloning, we *move* the data into the channel. The channel then owns the data, and can move it to another thread if needs-be. There's never any question of ownership, it's always clear who owns the data.

Let's look at an example:

```rust
use std::sync::mpsc;

// Not copyable or clone-able
struct MyData {
    data: String,
    n: u32,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            println!("--- IN THE THREAD ---");
            println!("Message number {}", data.n);
            println!("Received: {}", data.data);
        }
    });

    let mut n = 0;
    loop {
        println!("Enter a string");
        let input = read_line();
        let data_to_move = MyData {
            data: input,
            n,
        };
        n += 1;

        tx.send(data_to_move).unwrap();
    }
}
```

This pattern is also fast. Moving data generates a `memcpy` command behind the scenes, but most of the time the optimizer is able to remove it.

Let's benchmark it:

```rust
use std::sync::mpsc;

// Not copyable or clone-able
struct MyData {
    start: std::time::Instant,
}

pub fn read_line() -> String {
    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    input.trim().to_string()
}

fn main() {
    let (tx, rx) = mpsc::channel::<MyData>();

    std::thread::spawn(move || {
        while let Ok(data) = rx.recv() {
            let elapsed = data.start.elapsed();
            println!("--- IN THE THREAD ---");
            println!("Message passed in {} us", elapsed.as_micros());
        }
    });

    loop {
        println!("Enter a string");
        let _ = read_line();
        let data_to_move = MyData {
            start: std::time::Instant::now(),
        };

        tx.send(data_to_move).unwrap();
    }
}
```

On my development box, it averages 17 us per message. That's pretty fast. Definitely enough that if you are doing some serious work, you can afford to move the data.

# Sending Functions to Threads

We've focused on sending commands indicating that there's work to do. But what about sending whole functions? We can do that too!

> The code for this is in `sending_functions` in the `code/02_threads` folder.

```rust
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

fn hi_there() {
    println!("Hi there!");
}

fn main() {
    let (tx, rx) = mpsc::channel::<Job>();
    let handle = std::thread::spawn(move || {
        while let Ok(job) = rx.recv() {
            job();
        }
    });

    let job = || println!("Hello from the thread!");
    let job2 = || {
        for i in 0..10 {
            println!("i = {i}");
        }
    };
    tx.send(Box::new(job)).unwrap();
    tx.send(Box::new(job2)).unwrap();
    tx.send(Box::new(hi_there)).unwrap();
    tx.send(Box::new(|| println!("Inline!"))).unwrap();
    handle.join().unwrap();
}
```

There's a bit to unwrap here:

* What is a `Box`? A `Box` is a smart pointer to an area of the heap. So the function pointer is placed inside a smart pointer, and then sent to the thread. The thread then takes ownership of the smart pointer, and when it's done with it, the smart pointer is dropped, and the function pointer is dropped with it. Without a box, you run into lifetime issues. You'll learn all about Boxes in a couple of weeks.
* What about `dyn`? `dyn` is a special marker indicating that the contents is dynamic. In this case, it's a dynamic function pointer. It doesn't necessarily point to just one function, and the exact form of the function is dynamic.
* How about `FnOnce`? This is a function that indicates that it will run once, and won't try to change the world around it. You quickly get into trouble when you need scope capture when you are passing function pointers around.
* What about `Send`? This indicates that the function pointer can be sent to another thread. This is a special marker trait that indicates that the function pointer is safe to send to another thread.

## Send and Sync

Types in Rust can implement the `Sync` and `Send` traits. `Sync` means that a type is synchronized, and can be safely modified. `Mutex` is a good example of a `Sync` type. `Send` means that a type can be sent to another thread. In this case, we're requiring that the function can be safely sent between threads.

Most of the time, `Sync` and `Send` are figured out for you. If anything in a structure isn't sync or send, the structure won't be. (You can override it if you really need to!)

If you're moving data along channels and run into a `Sync` or `Send` error it's usually a clue that you need to add protection---like a Mutex---around the data.

We'll look at `Send` and `Sync` later.

## Sending Commands and Functions

You can mix and match commands and functions using the same channel. Rust Enumerations can hold functions, just like any other type. Let's fix the issue of the program never quitting:

> This is the `sending_commands_and_functions` example in `code/02_threads`.

```rust
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;

enum Command {
    Run(Job),
    Quit,
}

fn main() {
    let (tx, rx) = mpsc::channel::<Command>();
    let handle = std::thread::spawn(move || {
        while let Ok(command) = rx.recv() {
            match command {
                Command::Run(job) => job(),
                Command::Quit => break,
            }
        }
    });

    let job = || println!("Hello from the thread!");
    tx.send(Command::Run(Box::new(job))).unwrap();
    tx.send(Command::Quit).unwrap();
    handle.join().unwrap();
}
```