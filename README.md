# Running a Rust Program and Using the Compiler

Rust uses the `rustc` compiler and the `cargo` build system to compile and run programs. Below is a guide to help you understand how to run a Rust program and the different ways you can interact with the Rust compiler.

---

## 🦀 Writing Your First Rust Program

Create a file called `main.rs`:

```rust
fn main() {
    println!("Hello, world!");
}
```

---

## ✅ Compiling with `rustc`

You can compile a simple Rust program using the `rustc` command directly:

```bash
rustc main.rs
```

This will produce an executable file (`main` on Linux/macOS, `main.exe` on Windows), which you can run:

```bash
./main
```

---

## ⚙️ Using Cargo (Recommended)

Cargo is Rust’s official build system and package manager. It handles compilation, dependency resolution, and more.

### 1. Create a New Project

```bash
cargo new hello_rust
cd hello_rust
```

This creates a project structure like:

```
hello_rust/
├── Cargo.toml
└── src/
    └── main.rs
```

### 2. Build and Run

To build the project:

```bash
cargo build
```

To run the project:

```bash
cargo run
```

To build in release mode (with optimizations):

```bash
cargo build --release
```

---

## 🔍 Exploring the Compiler Options

You can pass various flags to `rustc` for debugging, optimization, and customization:

### Compile with Debug Info

```bash
rustc -g main.rs
```

### Enable Optimizations

```bash
rustc -O main.rs
```

### Emit LLVM IR, Assembly, or MIR

```bash
rustc --emit=llvm-ir main.rs
rustc --emit=asm main.rs
rustc --emit=mir main.rs
```

### Specify Output File

```bash
rustc main.rs -o my_program
```

---

## 📦 Cargo vs. rustc

| Feature                | `rustc`          | `cargo`                     |
|------------------------|------------------|-----------------------------|
| Manual Compilation     | ✅                | ❌ (uses `rustc` internally) |
| Dependency Management  | ❌                | ✅                           |
| Project Management     | ❌                | ✅                           |
| Build Profiles         | Limited          | ✅ (`dev`, `release`, etc.) |

**Tip**: Use `cargo` for real projects and `rustc` for quick tests or learning compiler internals.

---

## 📚 Further Reading

- [The Rust Programming Language Book (The Book)](https://doc.rust-lang.org/book/)
- [Cargo Book](https://doc.rust-lang.org/cargo/)
- `rustc --help` and `cargo --help`
