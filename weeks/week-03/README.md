# Week 3 Assignment: Modeling Bitcoin Data

## Concepts

This assignment covers:

- Structs with named fields
- Enums and pattern matching
- Methods on structs and enums
- Deriving `Serialize` and `Deserialize` with Serde
- Using `uuid` to give transaction outputs stable unique ids
- Borrowed lookups with `Option`
- Static dispatch through generic trait bounds
- Dynamic dispatch through `Box<dyn Trait>`
- Iterator chains over nested transaction data

The goal is to model a small Bitcoin-like library without networking or mining. From this week onward, external crates are allowed when they match the book project.

## Student Work

Open `src/lib.rs` and implement the TODOs. Keep all public names and signatures unchanged.

This week uses:

- `serde` for data model derives
- `uuid` for `TxOutput::unique_id`

Do not add more crates for this assignment.

The validation rules are intentionally small:

- A transaction must have a non-empty txid.
- A coinbase transaction has txid `"coinbase"` and no inputs.
- Non-coinbase transactions must have at least one input.
- Every transaction must have at least one output.
- Output values must be greater than zero.
- A block must have at least one transaction.
- Transaction ids inside a block must be unique.

Use these Bitcoin network magic values:

- Mainnet: `0xD9B4BEF9`
- Testnet: `0x0709110B`
- Signet: `0x40CF030A`
- Regtest: `0xDAB5BFFA`

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_03_modeling
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Use only the Rust standard library.
- Keep the data model simple and explicit.
- Prefer `match` when handling enums.
