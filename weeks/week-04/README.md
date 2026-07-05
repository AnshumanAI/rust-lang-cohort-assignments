# Week 4 Assignment: Traits, Generics, Iterators, and Error Handling

## Concepts

This assignment covers:

- `Option<T>` for absence
- `Result<T, E>` for recoverable failures
- Custom error enums
- Implementing `From<std::io::Error>`
- Traits and generic helper functions
- Iterator-based calculations
- Avoiding panics in parser code

The parser works with a simple transaction row:

```text
txid_123,500,unspent
```

The fields are `txid`, `amount_sats`, and `status`.

## Student Work

Open `src/lib.rs` and replace each `todo!()` with a working implementation. Keep all public names and signatures unchanged.

Parsing should return `Err(BtcLibError::MalformedData)` for the wrong number of fields, empty fields, invalid amounts, zero amounts, unknown status values, and other malformed rows.

Parser functions must not panic.

Behavior expected by the tests:

- Status parsing trims whitespace and is case-insensitive.
- `Hashable::hash_material` for a transaction should be `txid:amount_sats:status`, where status is lowercase.
- `toy_hash` starts at `0`, then for each byte does `hash = hash * 31 + byte`, using wrapping arithmetic.
- `require_transaction` returns `Err(BtcLibError::MissingTransaction)` when no matching txid exists.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_04_errors_traits
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Use only the Rust standard library.
- Use iterators where they make the code clearer.
- Prefer explicit errors over `unwrap()` and `expect()`.
