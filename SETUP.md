# Setting Up Rust

This guide walks you through installing Rust and getting the exercises running.

---

## 1 · Install Rust

Rust is installed via **rustup**, the official toolchain manager.

### Linux / macOS

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the on-screen prompts. When the installer finishes, run:

```bash
source "$HOME/.cargo/env"
```

### Windows

Download and run **rustup-init.exe** from <https://rustup.rs/>.

### Verify the installation

```bash
rustc --version   # e.g. rustc 1.78.0 (...)
cargo --version   # e.g. cargo 1.78.0 (...)
```

---

## 2 · Keep Rust up to date

```bash
rustup update
```

---

## 3 · Clone this repository

```bash
git clone https://github.com/Luczenn/Learning-Rust.git
cd Learning-Rust
```

---

## 4 · Project structure

```
Learning-Rust/
├── Cargo.toml            ← workspace definition
├── SETUP.md              ← this file
├── README.md             ← overview and exercise table
└── exercises/
    ├── 01_hello_world/   ← cargo package
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── 02_primitives/
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── 03_custom_types/
    ├── 04_variable_bindings/
    ├── 05_control_flow/
    ├── 06_functions/
    ├── 07_ownership_borrowing/
    ├── 08_error_handling/
    └── 09_collections/
```

All packages are part of one **Cargo workspace**, so you can build or test
everything from the repository root with a single command.

---

## 5 · Build all exercises

```bash
cargo build
```

---

## 6 · Run a specific exercise

```bash
cargo run -p hello_world
cargo run -p primitives
cargo run -p custom_types
cargo run -p variable_bindings
cargo run -p control_flow
cargo run -p functions
cargo run -p ownership_borrowing
cargo run -p error_handling
cargo run -p collections
```

---

## 7 · Run the tests

```bash
# All exercises
cargo test

# One exercise at a time
cargo test -p hello_world
cargo test -p control_flow
```

---

## 8 · Recommended editor setup

### VS Code

1. Install [VS Code](https://code.visualstudio.com/).
2. Install the **rust-analyzer** extension (search for `rust-analyzer` in the
   Extensions panel).
3. Open the repository folder: `code .`

rust-analyzer provides inline type hints, auto-completion, error highlighting,
and "go to definition" support.

### Other editors

- **IntelliJ / RustRover**: install the [Rust plugin](https://plugins.jetbrains.com/plugin/8182-rust).
- **Neovim / Vim**: use [rust.vim](https://github.com/rust-lang/rust.vim) or configure
  rust-analyzer via an LSP client.

---

## 9 · Useful Cargo commands

| Command | Description |
|---------|-------------|
| `cargo build` | Compile the workspace (debug mode) |
| `cargo build --release` | Compile with optimizations |
| `cargo run -p <name>` | Build and run a specific package |
| `cargo test` | Run all tests |
| `cargo clippy` | Lint the code (install with `rustup component add clippy`) |
| `cargo fmt` | Auto-format source code (install with `rustup component add rustfmt`) |
| `cargo doc --open` | Generate and open HTML documentation |
| `cargo check` | Fast syntax/type check without producing a binary |

---

## 10 · Troubleshooting

**`cargo: command not found`**
> The Cargo bin directory is not in your `PATH`.
> Run `source "$HOME/.cargo/env"` or add `$HOME/.cargo/bin` to your shell's
> profile file (`.bashrc`, `.zshrc`, etc.).

**`error[E0463]: can't find crate for …`**
> Your Rust toolchain may be outdated. Run `rustup update` and try again.

**Compilation is slow the first time**
> Rust compiles all dependencies from source. Subsequent builds are cached and
> much faster.
