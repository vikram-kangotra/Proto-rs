# Proto-rs Compiler

Proto-rs is a compiler written in Rust, utilizing LLVM as its backend. This compiler translates Proto-rs source code into LLVM Intermediate Representation (IR), which can then be further processed by the LLVM toolchain.

# Features

- variable initialization
- `if-else` condition
- `while` loop
- `break` statements
- `return` statements
- arithmetic operations
- scoped block statements
- single line and multiple line comments
- Outputs Assembly and object files

## Prerequisites
Before you proceed, ensure you have the following prerequisites installed:

- [Git](https://git-scm.com/)
- [Rust](https://www.rust-lang.org/)

Clone the Proto-rs repository and build the project:

```rust
git clone https://github.com/vikram-kangotra/Proto-rs.git
cd Proto-rs
cargo build
```

# Usage
Compile Proto-rs source code using the following command:

```bash
proto-rs [OPTIONS] --output <OUTPUT> <INPUT>
```

## Example

```bash
proto-rs source_file.pr -o output_file.o
```

This command will translate the Proto-rs source file (source_file.pr) into to the specified object file (output_file.o).

# Command-Line Options
Proto-rs supports the following command-line options:

```bash
Arguments:
  <INPUT>  source proto file to compile

Options:
  -o, --output <OUTPUT>  output file
  -S                     output assembly file
  -h, --help             Print help
  -V, --version          Print version
```

# Licence
Proto-rs is licensed under the [MIT Licence](./LICENSE)
