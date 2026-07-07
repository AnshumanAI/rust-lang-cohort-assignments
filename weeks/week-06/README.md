# Week 6 Assignment: UTXO State, Concurrency, and CPU Mining

## Concepts

This week builds the toy CPU miner on top of the Bitcoin-like library ideas from earlier assignments. The focus is no longer just modeling data; students now maintain UTXO state, construct candidate blocks, search nonces, and coordinate worker threads safely.

The assignment uses only the standard library for concurrency. You will work with `BTreeMap`, `Arc`, `Mutex`, channels, spawned threads, and deterministic hash material. The tests keep proof-of-work deterministic by choosing prefixes derived from known candidate hashes.

## Student Work

Open the files under `src/` and replace every `todo!()` with a working implementation. Keep public names and signatures unchanged.

The small constructors are only there to make the test fixtures easy to assemble. The main work is in the mempool, UTXO mutation, candidate construction, proof-of-work search, and thread coordination modules.

The crate is intentionally split into modules:

- `error.rs` for miner errors
- `types.rs` for transaction, block, candidate, and hashing types
- `utxo.rs` for UTXO set mutation
- `mempool.rs` for deterministic transaction selection
- `pow.rs` for hash, merkle, and difficulty helpers
- `miner.rs` for candidate construction and worker coordination

The structure is inspired by the book project's miner/library split, but the data model and fixtures here are original to this assignment. Do not copy miner code or data files from the book repository.

Important exact formats:

```text
tx:<txid>|inputs:<previous_txid>:<previous_vout>;...|outputs:<value>:<recipient>;...
candidate:<previous_hash>|height:<height>|merkle:<merkle>|time:<timestamp>|nonce:<nonce>|txs:<txid>;...
workers:<worker_count>|nonce:<nonce>|attempts:<attempts>|hash:<hash>
```

UTXO updates should be atomic from the caller's perspective: an invalid transaction should not partially mutate the set. Mining ranges are inclusive, so `start_nonce = 0` and `max_nonce = 9` means ten attempts.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_06_miner
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Keep proof-of-work deterministic and testable.
- Validate difficulty prefixes before mining.
- Prefer message passing for worker results.
- Avoid partial UTXO updates on failed spends.
- Treat simple constructors as setup; spend your design attention on module boundaries and state transitions.
