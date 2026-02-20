# Learning Rust 🦀

A structured repository for learning the Rust programming language, based on
[Rust by Example](https://doc.rust-lang.org/rust-by-example/).

Each exercise is a standalone Cargo package inside the `exercises/` folder.
All packages share a single Cargo workspace so you can build and test
everything with one command from the repository root.

---

## Setup

See **[SETUP.md](SETUP.md)** for step-by-step instructions on installing Rust
and running the exercises.

---

## Exercises

| # | Topic | Key Concepts |
|---|-------|-------------|
| [01](exercises/01_hello_world/) | **Hello World** | `println!`, format strings, comments |
| [02](exercises/02_primitives/) | **Primitives** | integers, floats, bool, char, tuples, arrays, type casting |
| [03](exercises/03_custom_types/) | **Custom Types** | structs, enums, `impl`, pattern matching |
| [04](exercises/04_variable_bindings/) | **Variable Bindings** | `let`, mutability, shadowing, scope, constants |
| [05](exercises/05_control_flow/) | **Control Flow** | `if`/`else`, `loop`, `while`, `for`, `match`, `if let` |
| [06](exercises/06_functions/) | **Functions & Closures** | functions, recursion, closures, iterators |
| [07](exercises/07_ownership_borrowing/) | **Ownership & Borrowing** | ownership, move, copy, `&T`, `&mut T`, slices, lifetimes |
| [08](exercises/08_error_handling/) | **Error Handling** | `panic!`, `Option`, `Result`, `?` operator, custom errors |
| [09](exercises/09_collections/) | **Collections** | `Vec`, `String`, `HashMap`, `HashSet` |

---

## Running the Exercises

```bash
# Run a single exercise
cargo run -p hello_world
cargo run -p primitives
cargo run -p custom_types
cargo run -p variable_bindings
cargo run -p control_flow
cargo run -p functions
cargo run -p ownership_borrowing
cargo run -p error_handling
cargo run -p collections

# Run all tests
cargo test

# Run tests for a single exercise
cargo test -p hello_world
```

---

## Learning Path

Work through the exercises **in order** – each one builds on the previous:

1. **Hello World** – Get something on the screen; understand `println!`.
2. **Primitives** – Learn Rust's basic types and how casting works.
3. **Custom Types** – Define your own data shapes with structs and enums.
4. **Variable Bindings** – Understand how Rust handles variables, mutability, and scope.
5. **Control Flow** – Learn `if`, loops, and Rust's powerful `match` expression.
6. **Functions & Closures** – Write reusable code and functional-style pipelines.
7. **Ownership & Borrowing** – The heart of Rust: memory safety without a GC.
8. **Error Handling** – Write robust code with `Option`, `Result`, and `?`.
9. **Collections** – Work with `Vec`, `String`, `HashMap`, and `HashSet`.

Each file contains **"Try it yourself!"** exercises at the bottom – uncomment
the snippets and experiment!

---

## Further Resources

- [The Rust Programming Language (The Book)](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) – small interactive exercises
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [Rust Playground](https://play.rust-lang.org/) – try Rust in the browser

