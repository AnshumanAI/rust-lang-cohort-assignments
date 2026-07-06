# Rust Cohort Assignments

This repository contains the first four weekly assignments for the Rust cohort built around the early chapters of *Building Bitcoin in Rust* and the Rust Book.

Each week is a separate Cargo library crate under `weeks/`. The starter code lives in `src/lib.rs`, and the visible tests live in that week's `tests/` directory.

## Weeks

- `weeks/week-01`: Rust syntax, primitives, vectors, hash maps, and simple Bitcoin data.
- `weeks/week-02`: Ownership, borrowing, string slices, basic lifetimes, and small utilities.
- `weeks/week-03`: Structs, enums, pattern matching, Serde derives, UUIDs, dispatch, and Bitcoin domain modeling.
- `weeks/week-04`: Extends the Week 3 model with parsing, validation, `thiserror`, SHA-256 hashing, hex decoding, and block-building helpers.

## Running Tests

From the repository root:

```bash
cargo test --workspace
```

Run a single week:

```bash
cargo test -p week_01_foundations
cargo test -p week_02_ownership
cargo test -p week_03_modeling
cargo test -p week_04_errors_traits
```

Check formatting:

```bash
cargo fmt --all -- --check
```

The starter code intentionally contains `todo!()` implementations. Tests are expected to fail until students complete the assignment.
