# Stellar Give Bootcamp

This project contains beginner-friendly Rust programs, each in its own file under `src/bin/`.

## Prerequisites

- Install [Rust](https://www.rust-lang.org/tools/install) (includes `cargo`)
- Verify installation:

```bash
rustc --version
cargo --version
```

## Run the default app

The default program is in `src/main.rs`:

```bash
cargo run
```

## Run individual example files

Each file in `src/bin` is treated as a separate binary.  
Use this format:

```bash
cargo run --bin <file-name-without-.rs>
```

Examples in this repo:

- `cargo run --bin display-name` - Prints first and last name
- `cargo run --bin if-else-if-statements` - Basic `if / else if / else` logic
- `cargo run --bin match-expressions` - `match` with boolean and integer values
- `cargo run --bin grocery-items` - Struct fields and helper display functions
- `cargo run --bin for-loop` - Looping through a vector and conditional output
- `cargo run --bin people` - Struct vector iteration with filtering (`age <= 10`)
- `cargo run --bin student` - `Option` with pattern matching (`Some` / `None`)
- `cargo run --bin cartesian-coordinates` - Tuple destructuring and numeric comparison

## Helpful commands

- Check code compiles:

```bash
cargo check
```

- Build without running:

```bash
cargo build
```
