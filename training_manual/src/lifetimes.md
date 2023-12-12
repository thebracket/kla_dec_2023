# Lifetimes

The borrow checker not only tracks borrows, it attaches a *lifetime* to every borrow.

In very early versions of Rust, you had to annotate every reference with a lifetime. Be glad you don't have to do this anymore! Code could look like this:

```rust
fn do_it<'a>(s: &'a String) {
    println!("{s}");
}

fn main() {
    let s = String::from("Hello");
    do_it(&s);
}
```

This is still valid Rust, but in *most* cases Rust is able to deduce an "anonymous lifetime" for reference usage. Let's look at the new code:

* `do_it<'a>` *introduces* a new lifetime, named `a`. You can name lifetimes whatever you want, but it's common to use short names.
* In the arguments, `s: &'a String` states that the borrowed `String` adheres to lifetime `a`.

What's really happening here? Rust is tracking that when you call `do_it`, a lifetime is created. The lifetime *must* exceed the lifetime of the object being pointed at. Not doing so is a compiler error.

## Escaping References

In Go, this is a really common idiom. The Go compiler will detect that you're referencing a local variable (via escape analysis), hoist it to the heap without telling you, and let you have your reference.

This compiles in C++:

```c++
#include <iostream>
using namespace std;

int& bar()
{
    int n = 10;
    return n;
}

int main() {
    int& i = bar();
    cout<<i<<endl;
    return 0;
}
```

The code *does* generate a warning, but it actually functioned on 2 of the 3 systems I tried it on! Rust is not so forgiving:

```rust
fn do_it() -> &String {
    let s = String::from("Hello");
    &s
}

fn main() {
    let s = do_it();
}
```

Rust starts by telling you that you need a lifetime specifier, and suggests a special lifetime called `'static`. Static is a special lifetime in which you are promising that a reference will live forever, and Rust can not worry about it. So let's try that:

```rust
fn do_it() -> &'static String {
    let s = String::from("Hello");
    &s
}

fn main() {
    let s = do_it();
}
```

It still doesn't compile, this time with the correct error: `cannot return a reference to local variable`.

The borrow checker prevents this problem.

## Returning References

What if you actually *do* want to return a valid reference? This function won't compile without lifetime specifiers.

```rust
fn largest<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
    if a > b {
        &a
    } else {
        &b
    }
}

fn main() {
    let a = 1;
    let b = 2;
    let ref_to_biggest = largest(&a, &b);
    println!("{ref_to_biggest}");
}
```

You have to clarify to Rust that the function can assume that both references will share a lifetime with the function output. So now for the returned reference to remain valid, both inputs also have to remain valid. (In this example, we're using a type that would be better off being copied anyway!)

## Keeping References

Life starts to get complicated when you want to keep references around. Rust has to validate the lifetimes of each of these references.

```rust
struct Index {
    selected_string: &String
}

fn main() {
    let strings = vec![
        String::from("A"),
        String::from("B"),
    ];
    let index = Index {
        selected_string: &strings[1]
    };
    println!("{}", index.selected_string);
}
```

This fails to compile, but the compiler error tells you what needs to be done. So we apply its suggestions:

```rust
struct Index<'a> {
    selected_string: &'a String
}

fn main() {
    let strings = vec![
        String::from("A"),
        String::from("B"),
    ];
    let index = Index {
        selected_string: &strings[1]
    };
    println!("{}", index.selected_string);
}
```

And that works! You've tied the structure to the lifetime of the references it holds. If the strings table goes away, then the `Index` is invalid. Rust won't let this compile:

```rust
struct Index<'a> {
    selected_string: &'a String
}

fn main() {
    let index = {
        let strings = vec![
            String::from("A"),
            String::from("B"),
        ];
        let index = Index {
            selected_string: &strings[1]
        };
        index
    };
    println!("{}", index.selected_string);
}
```

The error message helpfully explains that `strings does not live long enough`---which is true. This is the primary purpose of the borrow checker: dangling references become a compile-time error, rather than a long head-scratching session at runtime.