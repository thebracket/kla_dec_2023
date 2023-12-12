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