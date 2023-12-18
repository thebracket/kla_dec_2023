# Debugging

Good news! Rust emits platform-standard debug information in binaries (unless you turn them off), so your existing debugging solution will work.

For Rust-specific debugging, `Rust Rover` from JetBrains is the nicest I've found so far. It sets everything up nicely for you, and seamlessly handles stepping into non-Rust code.

On Visual Studio Code, you need the `CodeLLDB` extension.

> Confession: I don't actually do a lot of single-step, breakpoint debugging. I tend to emit tracing messages and use those for debugging unless I'm really, really stuck!

Quick walkthrough of using both debuggers.