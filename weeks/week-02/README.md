# Week 2 Assignment: Ownership, Borrowing, and Strings

## Concepts

This assignment covers:

- Ownership, moves, borrowing, and cloning decisions
- `String` versus `&str`
- Working with string slices
- Small lifetime annotations on borrowed return values
- `Option<T>` for simple parse failures
- Fee and fee-rate calculations

The functions are intentionally small. The goal is to make each ownership decision visible.

## Student Work

Open `src/lib.rs` and implement every `todo!()` while keeping the public function signatures unchanged.

Pay attention to which functions should allocate a new `String` and which functions can return a borrowed `&str` slice from their input.

Behavior expected by the tests:

- `is_palindrome` ignores ASCII case, spaces, and punctuation.
- `simple_hash` starts at `0`, then for each byte does `hash = hash * 31 + byte`, using wrapping arithmetic.
- `fee_rate` rounds up and returns `None` when `vbytes` is zero.
- `select_longer` returns the left value when both strings have equal length.
- `normalize_label` trims, lowercases, and joins whitespace-separated words with `-`.
- `parse_outpoint` accepts `txid:vout`, trims both fields, and rejects empty txids or non-numeric vouts.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_02_ownership
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Use only the Rust standard library.
- Do not add global mutable state.
- Do not use command-line input.
- Prefer returning `None` for invalid input instead of panicking.
