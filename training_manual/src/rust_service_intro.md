# Rust as a Service

Every organization I've ever worked with has had slightly different preferences for containerization,
deployment and managing live services. That makes it tricky to teach a "one size fits all" solution!
In some cases, you are best off using bare metal or close to it for performance---abstracting your
hardware does carry a performance cost. If you're using Rust for massive per-server performance,
being close to the metal has its advantages.

Using containers can also be advantageous: it becomes easier to manage a large set of containers,
deployment can be automated, and you can slide Rust in alongside your existing setup.

## Building for Docker

Docker has great built-in support for Rust. Docker defaults to using a multi-stage builder for Rust:

* The first stage uses the officially provided (by the Rust Foundation) docker build platform for Rust.
* The second stage assembles the resultant binaries into a a small `debian:bullseye-slim` container.

> We'll use the `projects/docker/hello_web_docker` project in the example code. Notice that in the
parent `Cargo.toml`, I've excluded it from the workspace with `exclude = [...]`. This is to avoid
confusion that can lead to Dockerizing the whole workspace---which is often what you want, but not
when teaching a small section!

### Build an Application

We'll start very simple, with a "hello web"---just like we did before:

The **Cargo.toml** file:

```toml
[package]
name = "hello_web_docker"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.32.0", features = ["full"] }
axum = "0.6.20"
anyhow = "1.0.75"
```

The **main.rs** file:

```rust
use axum::{routing::get, Router};
use std::{net::SocketAddr, path::Path};
use axum::response::Html;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(say_hello_html));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn say_hello_html() -> Html<&'static str> {
    Html("<h1>Hello, world!</h1>")
}
```

This is nothing too impressive: it listens on port 3001 and serves `Hello World` to your browser.

### Let's Dockerize it

In your project's parent folder, run `docker init` and:

* Select `Rust` from the platform list.
* Accept the default (the current version) unless you need a different ont.
* We're listening on port 3001 (which is suggested!), so accept that.

With any luck, you'll see the following:

```
✔ Your Docker files are ready!

Take a moment to review them and tailor them to your application.

When you're ready, start your application by running: docker compose up --build

Your application will be available at http://localhost:3001

Consult README.Docker.md for more information about using the generated files.
```

Run:

```bash
docker compose up --build
```

The first time, you'll have to wait while Dockerfiles are downloaded and setup for the build process. Once they are cached, it's very fast.

You can now connect to your docker process, it's running! You can stop it with `ctrl-c`.

Go to [http://localhost:3000](http://localhost:3000). It doesn't work! You get "connection was reset".

You need to make one small change to your program. Your containerized app is only listening to `localhost`, meaning it isn't available outside of its container:

```rust
let addr = SocketAddr::from(([0, 0, 0, 0], 3001));
```

Now rerun `docker compose up --build` and try again. You'll see "Hello world" served from [http://localhost:3000](http://localhost:3000).

Note that `docker init` has created `README.Docker.md` with instructions for supporting other platforms, and a `Dockerfile`.

### Including Other Files

It's relatively unlikely that your program is completely self-contained. Because of the two-step build process, you need to edit your `Dockerfile` to include any files that are part of your program in the build, and in the final version.

For example, to include a `migrations` folder for SQLX you need to find the following section:

```
RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    --mount=type=cache,target=/app/target/,id=rust-cache-${APP_NAME}-${TARGETPLATFORM} \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    <<EOF
```

And add a line to include your migrations scripts:

```
--mount=type=bind,source=migrations,target=migrations \
```

You can also include environment variables (such as `DATABASE_URL`):

```
# Set the DB URL
ENV DATABASE_URL="sqlite::memory:"
```

> You probably have a "secrets" setup for your container solution. Use it as normal. There are too many possible choices to reasonably try and teach that here.