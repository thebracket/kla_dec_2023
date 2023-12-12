# Rust and C++ Tooling Equivalencies

> This is a cheat sheet for you to refer to later.

## Using Cargo

The `cargo` command is a swiss-army knife that handles building projects, testing them, controlling dependencies and more. It is extensible,
you can add more features to it and use it to install programs.

| Cargo Command   | C++ Equivalent | Purpose                                                                           |
|-----------------|--|-----------------------------------------------------------------------------------|
| *Package Commands* | |
| `cargo init`
| *Compilation*   | |                                                                                   |
| `cargo build`   | `make` | Builds your project, placing the output in the `target` directory.                |
| `cargo run`     | `make ; ./my_program` | Runs `cargo build`, and then runs the resulting executable.                       |
| `cargo check`   | | Build only the source, and skip assembly and linking for a quick check of syntax. |
| `cargo clean`   | `make clean` | Removes all build artefacts and empties the `target` directory.                   |
| `cargo rustc` | | Pass extra `rustc` commands to the build process                                  |
| *Formatting*    | |                                                                                   |
| `cargo fmt`     | | Formats your source code according to the Rust defaults.                          |
| *Testing*       | |                                                                                   |
| `cargo test`    | `make test` | Executes all unit tests in the current project                                    |
| `cargo bench`   | | Executes all benchmarks in the current project.                                   |
| *Linting*       | |                                                                                   |
| `cargo clippy`  | | Runs the `Clippy` linter                                                          |
| `cargo fix`     | | Applies all `Clippy` suggestions                                                  |
| *Documentation* | |                                                                                   |
| `cargo doc`     | | Builds a documentation website from the current project's sourcecode.             |
| `cargo rustdoc` | | Run the documentation builder with extra command options. |
| *Dependencies* | |                                                                                   |
| `cargo fetch` | | Downloads all dependencies listed in `Cargo.toml` from the Internet.              |
| `cargo add` | | Add a dependency to the current project's `Cargo.toml` |
| `cargo remove` | | Remove a dependency from the current project's `Cargo.toml` file |
| `cargo update` | | Update dependencies to the latest version in `Cargo.toml` |
| `cargo tree` | | Draw a tree displaying all dependencies, and each dependency's dependencies |
| `cargo vendor` | | Download all dependencies, and provide instructions to modify your `Cargo.toml` to use the downloaded dependencies.|
