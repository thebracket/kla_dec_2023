# WASM for the Browser

The oldest use case for WASM is including in the browser. Emscripten (C++) was the first system to popularize it. Browser WASM can be written as regular Rust, with a few exceptions---notably threads don't work in current browser setups.

I recommend keeping this reference handy: [https://rustwasm.github.io/wasm-bindgen/introduction.html](https://rustwasm.github.io/wasm-bindgen/introduction.html)

To work with Rust in the browser, you need two components:

## Installing Required Components

* WASM compiler toolchain. You can add it with `rustup target add wasm32-unknown-unknown`.
* WASM Bindgen, which generates JavaScript/Typescript bindings connecting Rust to the browser. You can install it with `cargo install wasm-bindgen-cli`.

Your project will also need to include `wasm-bindgen` in its dependencies. Note that when you upgrade `wasm-bindgen`, you need to *also* update `wasm-bindgen-cli` to the matching version.

## Testbed Server

> Browsers don't like running WASM from `localhost`, it violates the sandbox rules. So you typically need a webserver from which to test your code. I often keep a small server like `nginx` around while I'm developing WASM for the browser for quick turnaround.

In this case, let's build ourselves a mini Axum server that serves a directory. You can serve a folder named `web` with this short program:

```rust
use axum::Router;
use std::net::SocketAddr;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .fallback_service(ServeDir::new("web"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

And the `Cargo.toml`:

```toml
[package]
name = "wasm_web_server"
version = "0.1.0"
edition = "2021"

[dependencies]
axum = "0.6.18"
tokio = { version = "1.28.2", features = ["full"] }
tower-http = { version = "0.4.0", features = ["fs", "trace", "cors"] }
```

Using the `fallback_service` and `ServeDir` lets you serve a file by name if it didn't match any routes. Since we didn't define any roots, it'll serve any file with a matching name from the `web` directory.

Let's add a file, `web/index.html`:

```html
<html>
    <head>
        <title>Hello World</title>
    </head>
    <body>
        <p>Hello, World!</p>
    </body>
</html>
```

Run the project with `cargo run`, and visit [http://localhost:3001](http://localhost:3001) to verify that the server works.

## Creating a Rust Function to Call From JavaScript

Let's create a project with `cargo new --lib wasm_lib`.

Our `Cargo.toml` file will need a `wasm-bindgen` dependency:

```toml
[package]
name = "wasm_lib"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
wasm-bindgen = "0.2.86"
```

Note that we have to build a `cdylib` - a C compatible dynamic library. Otherwise, we'll get a statically linkable `rlib` (Rust library format) and no `.wasm` file will be created.

In our `lib.rs`, we'll start with the following:

```rust
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn hello_js() {
    log("Hello from Rust!");
}
```

There's a few parts here:

* We're importing the prelude of `wasm_bindgen` - useful imports.
* We have an `extern` block decorated with `wasm_bindgen` - the bindings generator will use this to map calls.
* We defined a `log` function, and indicated that its in the JavaScript namespace `console`. This adds a Rust function named `log`, which is equivalent to calling `console.log` in JavaScript.
* Then we build a regular Rust function that calls it. Decorating the function with `[wasm_bindgen]` instructs the `wasm_bindgen` system to generate a matching call within the generated web assembly wrapper to allow JavaScript to call it.

Now we have to build it. We can instruct Cargo to use the correct output with the `target` flag:

```bash
cargo build --release --target wasm32-unknown-unknown
```

In your `target/wasm32-unknown-unknown/release` directory, you will see `libwasm_lib.*`. This provides raw WASM, but doesn't provide any browser help (you can't really run it yet). You have to use `wasm-bindgen` to read the project, and create the JavaScript for the browser. By default, it will also generate TypeScript and use modern JS modules. We're going to keep it simple today.

```bash
mkdir -p out
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_lib.wasm --out-dir out --no-modules --no-typescript
```

In your `out` folder, you will see two files: `wasm_lib_bg.wasm` (a processed `.wasm` file) and `wasm_lib.js` (a JavaScript binding library to use it).

Now in our webserver, we'll make a quick placeholder to use it:

```html
<html>
  <head>
    <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
  </head>
  <body>
    <script src="./wasm_lib.js"></script>
    <script>
      window.addEventListener("load", async () => {
        await wasm_bindgen("./wasm_lib_bg.wasm");
        wasm_bindgen.hello_js();
      });
    </script>
  </body>
</html>
```

Put this file along with the two generated files into the `web` directory. Open [http://localhost:3001/hello_wasm.html](http://localhost:3001/hello_wasm.html) and check the web console - you will see the message `Hello from Rust!`. That worked, you've called a Rust function from JavaScript --- which in turn has called a JavaScript function. That gives you the basis of calling functions back and forth.

## Passing Types

Now let's add a simple math function:

```rust
#[wasm_bindgen]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}
```

Modify `index.html` to also call:

```js
console.log(wasm_bindgen.add(5, 10));
```

Go through the same build setup:

```bash
cargo build --release --target wasm32-unknown-unknown
mkdir -p out
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_lib.wasm --out-dir out --no-modules --no-typescript
cp out/* ../wasm_web_server/web/
```

And sure enough, your math function outputs `15`. So primitive types work fine. How about strings?

Add another function:

```rust
#[wasm_bindgen]
pub fn greet(s: String) -> String {
    format!("Hello {s}")
}
```

And add a line of JavaScript:

```js
console.log(wasm_bindgen.greet("Herbert"));
```

How about vectors?

```rust
#[wasm_bindgen]
pub fn sum(arr: &[i32]) -> i32 {
    arr.iter().sum()
}
```

```js
console.log(wasm_bindgen.sum([1, 2, 3, 4]));
```

## Custom Types

In other words, normal Rust works very smoothly. What if you want to define a type? That starts to get more complicated. The JS browser environment only has very limited types: classes, 64-bit signed integers and 64-bit floats (there are also some typed memory buffers). Rust has lots of types. So when you pass data between the two contexts, you find yourself needing some conversion code.

### Classes

If you'd like to represent struct + implementations as JavaScript classes, `wasm-bindgen` can help you. For example:

```rust
#[wasm_bindgen]
pub struct Person {
    pub name: String,
    pub age: u8,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old", self.name, self.age)
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}
```

Note that you're marking `wasm_bindgen` on both the structure and its implementation, and have to tag the constructor. Now let's take a look at this from the JavaScript side:

```js
let person = new wasm_bindgen.Person("Herbert", 48);
console.log(person.greet());
console.log(person.age);
console.log(person.get_age());        
```

Creating the `Person` works, and calling `greet` and `get_age` work. But referencing `person.age` does not work! You don't get an automatic bridge to fields, because of type conversion requirements. Getters will do the work for you---but you are back to writing lots of getters and setters.

### Arbitrary Data with Serde

You can work around this by using Serde, and Serde JSON to build a bridge between the systems. Add `serde` and `serde_json` to your project:

```bash
cargo add serde -F derive
cargo add serde_json
```

And now we can serialize our person and return JSON:

```rust
use serde::Serialize;

#[derive(Serialize)]
#[wasm_bindgen]
pub struct Person {
    name: String,
    age: u8,
}

#[wasm_bindgen]
impl Person {
    #[wasm_bindgen(constructor)]
    pub fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }

    pub fn greet(&self) -> String {
        format!("Hello, my name is {} and I am {} years old", self.name, self.age)
    }

    pub fn set_age(&mut self, age: u8) {
        self.age = age;
    }

    pub fn get_age(&self) -> u8 {
        self.age
    }
}

#[wasm_bindgen]
pub fn serialize_person(person: &Person) -> String {
    serde_json::to_string(person).unwrap()
}
```

Now in your JavaScript you can use JSON to fetch the person without having to worry about getters/setters:

```js
let person_json = wasm_bindgen.serialize_person(person);
let person_deserialized = JSON.parse(person_json);
console.log(person_deserialized);
```

You can use this to handle passing complicated types to and from JavaScript via the built-in JSON system. `serde_json` is really fast, but there is a performance penalty to transitioning data between the WASM sandbox and the browser.

## Communicating with Servers via REST

You can handle the API side of things directly from the WASM part of the browser.

Let's add a little more functionality to our webserver. In our `wasm_web_server` project, let's add Serde with `cargo add serde -f derive`. Then we'll add a simple JSON API.

```rust
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tower_http::services::ServeDir;
use serde::Serialize;

#[derive(Serialize)]
struct HelloJson {
    message: String,
}

async fn say_hello_json() -> axum::Json<HelloJson> {
    axum::Json(HelloJson {
        message: "Hello, World!".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/json", get(say_hello_json))
        .fallback_service(ServeDir::new("web"));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
```

Our WASM library needs some JavaScript imports to use the JS `fetch` API. You also need to add `wasm-bindgen-futures`. In `Cargo.toml` add:

```toml
[dependencies]
serde = { version = "1.0.193", features = ["derive"] }
serde_json = "1.0.108"
wasm-bindgen = "0.2.89"
wasm-bindgen-futures = "0.4.39"

[dependencies.web-sys]
version = "0.3.4"
features = [
  'Headers',
  'Request',
  'RequestInit',
  'RequestMode',
  'Response',
  'Window',
]
```

In our WASM library, we can now add the following to call the JSON API:

```rust
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, Response};

#[wasm_bindgen]
pub async fn fetch_hello_json() -> Result<JsValue, JsValue> {
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let url = format!("/json", repo);

    let request = Request::new_with_str_and_init(&url, &opts)?;

    request
        .headers()
        .set("Accept", "application/vnd.github.v3+json")?;

    let window = web_sys::window().unwrap();
    let resp_value = JsFuture::from(window.fetch_with_request(&request)).await?;

    // `resp_value` is a `Response` object.
    assert!(resp_value.is_instance_of::<Response>());
    let resp: Response = resp_value.dyn_into().unwrap();

    // Convert this other `Promise` into a rust `Future`.
    let json = JsFuture::from(resp.json()?).await?;

    // Send the JSON response back to JS.
    Ok(json)
}
```

## Shrinking Your WASM

