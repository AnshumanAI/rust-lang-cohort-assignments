# Week 4 Assignment: Extending the Bitcoin Library

## Concepts

This assignment continues directly from Week 3. You still work with Bitcoin-like `Transaction`, `TxInput`, `TxOutput`, `BlockHeader`, and `Block` types, but now the library starts behaving more like a real reusable component.

This week covers:

- Carrying the Week 3 data model forward
- `Option<T>` for absent previous outputs
- `Result<T, E>` for recoverable failures
- Deriving error display text with `thiserror`
- Implementing `From<std::io::Error>`
- Traits and generic helper functions
- Iterator-based calculations over transactions and blocks
- SHA-256 hashing with the `sha256` crate
- Hex decoding with the `hex` crate
- Zero-panic parser and validation code

## Student Work

Open `src/lib.rs` and replace each `todo!()` with a working implementation. Keep all public names and signatures unchanged.

Week 4 should feel like Week 3 made more useful:

- Week 3 modeled the data.
- Week 4 parses transaction rows into that model.
- Week 4 validates transactions and blocks with a centralized error type.
- Week 4 hashes transactions and blocks with SHA-256.
- Week 4 builds a block from parsed transaction rows.

This week uses:

- `serde` for data model derives
- `uuid` for `TxOutput::unique_id`
- `thiserror` for the error enum
- `sha256` for real hash hex strings
- `hex` for decoding hash strings into bytes

Do not add more crates for this assignment.

## Parser Format

Normal transaction row:

```text
tx1,coinbase:0,bob,1200,unspent
```

Coinbase transaction row:

```text
coinbase,-,alice,5000,unspent
```

The fields are:

```text
txid,previous_txid:vout,recipient,amount_sats,status
```

For coinbase transactions, the previous output field is exactly `-`.

Parsing should return `Err(BtcLibError::MalformedData)` for the wrong number of fields, empty required fields, invalid outpoints, non-coinbase transactions with `-` as the previous output, invalid amounts, zero amounts, unknown statuses, and other malformed rows.

Parser functions must not panic.

## Expected Formats

`Transaction::hash_material()` must use exactly:

```text
tx:<txid>|inputs:<prev_txid>:<vout>;...|outputs:<value>:<recipient>:<status>;...
```

`Block::hash_material()` must use exactly:

```text
block:<block_hash>|prev:<previous_block_hash>|height:<height>|txs:<txid>;...
```

Status text in hash material is lowercase: `spent` or `unspent`.

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

- Prefer explicit errors over `unwrap()` and `expect()`.
- Use iterators where they make the code clearer.
- Keep Week 3 model behavior intact while adding parsing, validation, and hashing.
