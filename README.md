# rust-ls

A minimal Unix-style `ls` written in Rust using only the standard library.

## What it does

Lists directory entry names (one per line), sorted. Defaults to the current directory; accepts an optional path argument.

## Build & run

```bash
cd docs/rust-ls   # or the absolute path to this folder
cargo build
cargo run
cargo run -- /tmp
```

If the project lives on a filesystem that cannot create `target/` (e.g. some agent stores), point Cargo elsewhere:

```bash
CARGO_TARGET_DIR=/tmp/rust-ls-target cargo build
CARGO_TARGET_DIR=/tmp/rust-ls-target cargo run -- /tmp
```

## Usage

```
rust-ls [PATH]
```

Exit codes: `0` success, `1` I/O error (e.g. missing path), `2` bad usage.
