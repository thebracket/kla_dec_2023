# WASM with WASI

WASI offers a way to use Web Assembly as a container, for secure remote deployment.

Building a WASI project is quite familiar:

```bash
cargo new wasm_hello_world
```

Then edit the `main.rs` file:

```rust
// Import rust's io and filesystem module
use std::io::prelude::*;
use std::fs;

// Entry point to our WASI applications
fn main() {

  // Print out hello world!
  println!("Hello world!");

  // Create a file
  // We are creating a `helloworld.txt` file in the `/helloworld` directory
  // This code requires the Wasi host to provide a `/helloworld` directory on the guest.
  // If the `/helloworld` directory is not available, the unwrap() will cause this program to panic.
  // For example, in Wasmtime, if you want to map the current directory to `/helloworld`,
  // invoke the runtime with the flag/argument: `--mapdir /helloworld::.`
  // This will map the `/helloworld` directory on the guest, to  the current directory (`.`) on the host
  let mut file = fs::File::create("/helloworld/helloworld.txt").unwrap();

  // Write the text to the file we created
  write!(file, "Hello world!\n").unwrap();
}
```

To actuall build the project you need to install the WASI target:

```bash
rustup target add wasm32-wasi
```

And add a dependency to your project:

```bash
cargo add wasmtime
```

You can then build it with:

```bash
cargo build --target wasm32-wasi
```

To execute WASI projects locally, use `wasmtime`. You can install WASMTime by going to [https://wasmtime.dev/](https://wasmtime.dev/) and following the instructions there. Typically, just run (on Linux or MacOS):

```bash
curl https://wasmtime.dev/install.sh -sSf | bash
```

Finally, you can run the WASI program inside the `wasmtime` host:

```bash
wasmtime --mapdir /helloworld::. target/wasm32-wasi/debug/wasi_hello_world.wasm
```

If all went well, a file was created. You are running a `wasm` binary in a container!
