# Dependencies

Cargo includes dependency management, as opposed to having to integrate `vcpkg`, `conan`, etc.

We've used dependencies already to link a library from the same workspace. Adding other dependencies follows a similar pattern.

## Finding Dependencies

You can search the available public crate repo (crates.io) with `cargo search <term>`. For example, searching for `serde` (a crate we'll be using later) gives the following result:

```
$ cargo search serde
serde = "1.0.193"                       # A generic serialization/deserialization framework
sideko_postman_api = "1.0.0"            # Rust API bindings - spostman_api
discord_typed_interactions = "0.1.0"    # suppose you're working with discord slash commands and you want statically typed reques…
serde_json_experimental = "0.0.0"       # A JSON serialization file format
serde_valid = "0.16.3"                  # JSON Schema based validation tool using with serde.
alt_serde_json = "1.0.61"               # A JSON serialization file format
serde_json = "1.0.108"                  # A JSON serialization file format
serde_jsonc = "1.0.108"                 # A JSON serialization file format
serde_partiql = "1.1.65"                # A PartiQL data model serialization file format
deserr = "0.6.1"                        # Deserialization library with focus on error handling
... and 5301 crates more (use --limit N to see more)
```

It's often more productive to use Google or `crates.io` directly to see what's available. The number of crates is growing rapidly, and they are of varied quality. It's worth doing a little research before picking one!

## Adding Crates to Your Project

You can either use `cargo search` and find the appropriate information and add a crate by hand to your `Cargo.toml`:

```toml
[dependencies]
serde = "1.0.193"
```

Or you can use `cargo add serde` to add the crate to your `Cargo.toml`.

## Feature Flags

Rust crates can have feature flags that enable functionality. For example, when using `serde` most of the time you will also use the `derive` feature flag to enable `#[derive(Serialize)]` type macros that make life much easier.

You'd either edit `Cargo.toml` to read:

```toml
[dependencies]
serde = { version = "1.0.193", features = [ "derive" ] }
```

Or run `cargo add serde -F derive`.

## Updating Crates

You can update to the latest versions with `cargo update`.

## "Vendoring" Crates

For repeatable builds (or working offline), you can run `cargo vendor` and follow the instructions. It downloads the source for all of your dependencies, and provides a snippet to add to your `Cargo.toml` to use the local versions.

## Other Crate Sources

You can connect a crate via a Git repo (and optionally add a `branch=` specifier):

```toml
[dependencies]
bracket-lib = { git = "https://github.com/amethyst/bracket-lib.git" }
```

You can use also use a path, like we did for libraries.

## Viewing dependencies

`cargo tree` will show you all of your dependencies for a project, nested with *their* dependencies. It can get a bit excessive, sometimes!

## Cargo.lock

Cargo maintains a file named `Cargo.lock`. The Rust project recommend *not* including this in your git repo---but many people recommend the opposite!

`Cargo.lock` lists exact versions of every crate that was used by the entire build process. If you have the same `Cargo.lock`, you will download the exact same files (if they haven't been withdrawn).