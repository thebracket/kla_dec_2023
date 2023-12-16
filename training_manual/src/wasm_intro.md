# Web Assembly

Rust is a first-class WASM (Web Assembly) citizen, and makes working with WASM pretty straightforward. There are two major use-cases for WASM:

* In the browser
* In a WASM runtime (WASI, Docker, etc.)

There's also a few caveats:

* In the browser, threads work differently. You can't use regular threading systems and expect them to work at all.
* In the browser, you usually need to make your WASM system as small as possible.
* Even inside WASI, your capabilities are limited to what the runtime exposes. Not everything will work.
* WASM is really fast by browser standard---but really slow compared to native code. There's a price to pay for being in a sandbox.

On the upside, WASM provides a safe, platform-agnostic way to deploy your code---and can be many times faster than regular JavaScript code, particularly JS that handles math or large amounts of memory.
