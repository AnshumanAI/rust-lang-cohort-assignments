# Week 1 Assignment: Rust Foundations and Bitcoin Basics

## Concepts

This assignment covers:

- Basic Rust syntax and functions
- Primitive types, strings, vectors, and hash maps
- Simple control flow with `if`, `match`, and loops
- Iterating over mock transaction data
- The Bitcoin genesis block, block subsidy, and satoshi-denominated amounts

There is no CLI for this week. All behavior is exercised through functions and tests.

## Student Work

Open `src/lib.rs` and replace each `todo!()` with a working implementation. Keep the public function signatures unchanged because the tests call those exact functions.

The mock transaction helpers should treat only confirmed transactions as final ledger entries when calculating balances and address totals.

For `classify_amount`, use these labels:

- `"dust"` for values below 546 sats
- `"micro"` for values from 546 sats up to, but not including, 100,000 sats
- `"standard"` for values from 100,000 sats up to, but not including, 100,000,000 sats
- `"large"` for values at or above 100,000,000 sats

For `block_subsidy`, start at 50 BTC and halve every 210,000 blocks. Return zero once the halving count reaches 64 or more.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_01_foundations
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Use only the Rust standard library.
- Do not read command-line arguments.
- Prefer clear loops and simple iterator chains over clever one-liners.
- Amounts are represented in satoshis. One BTC is `100_000_000` satoshis.
