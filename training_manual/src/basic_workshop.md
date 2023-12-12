# Workshop: Build a Basic Rust System

In this workshop, we're going to collaboratively build a login system---gradualling introducing Rust concepts. The goal is to create a useful system.

1. We'll start by setting up a project with a workspace, a library and an executable.
2. We'll read input from `stdin`, and wrap it in a convenient funciton.
3. That will let us use a basic function "if name =" type of login system.
4. We'll dive into Rust enumerations, which are quite unlike `enum` in other languages.
5. We'll explore storing login information in structures, arrays and vectors---and dabble with iterator functions.
6. Serialization and deserialization to load/save password files.
7. Hashing passwords---or how to use dependencies to make life easier.
8. We'll use `clap`, a framework for CLI functions to build a CRUD controller for our password manager.

> The code for this is presented as a series of projects for each stage. It is in `projects/part3`.