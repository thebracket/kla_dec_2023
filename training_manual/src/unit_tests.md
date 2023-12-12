# Unit Tests

You saw an example unit test when you created a library. Rust/Cargo has a built-in unit testing system. Let's explore it a bit.


Let's build a very simple example, and examine how it works:

> The code for this is in `projects/part2/unit_test`

```rust
fn double(n: i32) -> i32 {
    n * 2
}

#[cfg(test)] // Conditional compilation: only build in `test` mode
mod test { // Create a module to hold the tests
    use super::*; // Include everything from the parent module/namespace

    #[test] // This is a test, we want to include in our unit test runs
    fn two_times() {
        assert_eq!(4, double(2)); // Assert that 2*2 = 4
        assert!(5 != double(2)); // Assert that it doesn't equal 5
    }
}
```

You can run tests for the current project with `cargo test`. You can append `--all` to include all projects in the current workspace.

We'll talk about more complicated tests later.