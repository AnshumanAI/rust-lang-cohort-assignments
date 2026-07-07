# Week 5 Assignment: Modules, Persistence, and Chain State

## Concepts

This week turns the Week 4 library shape into a maintainable project. Instead of keeping every type and helper in one large file, the crate is split into modules for errors, models, hashing, chain state, and storage. The assignment also introduces JSON persistence and small utility binaries that use the library from the outside.

You will work with `Transaction`, `Block`, `Blockchain`, merkle roots, validation, file I/O, and Serde round trips. The important design goal is that loaded data must be validated before the library accepts it.

## Student Work

Open the files under `src/` and replace each `todo!()` with a working implementation. Keep all public names and function signatures unchanged because the tests call them directly.

Pay attention to the exact string formats documented in the starter comments. For example, `chain_summary` must return:

```text
network:<network>|height:<height>|blocks:<count>|tip:<tip_hash>|txs:<total_transactions>
```

The merkle root helper should hash transaction hashes in pairs. If a level has an odd number of hashes, duplicate the final hash before pairing it.

This week uses:

- `serde` and `serde_json` for persistence
- `uuid` for transaction output identifiers
- `sha256` for transaction, block, and merkle hashing
- `thiserror` for stable library errors

Do not add more crates for this assignment.

## Utility Binaries

Two tiny binaries are included:

```bash
cargo run -p week_05_persistence --bin chain_height -- path/to/chain.json
cargo run -p week_05_persistence --bin validate_chain -- path/to/chain.json
```

The binaries should stay thin. Real behavior belongs in the library.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_05_persistence
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Validate before saving and after loading.
- Do not use `unwrap()` or `expect()` in library code.
- Keep module boundaries clear.
- Prefer small helper functions over duplicating chain validation logic.
