# Week 8 Assignment: Wallet Integration and Capstone Readiness

## Concepts

This week connects the library, miner, and node ideas into wallet-facing architecture. The goal is not to build a flashy interface. The goal is to build clean wallet core logic that a CLI or TUI can call without owning the business rules.

You will work with wallet UTXOs, deterministic UTXO selection, transaction construction, change outputs, fee calculation, a small node-client boundary, structured event logging, async submission, and sync flows.

## Student Work

Open the files under `src/` and replace every `todo!()` with a working implementation. Keep public names and signatures unchanged.

The basic data constructors are only setup for tests. The main work is wallet architecture: deterministic UTXO selection, fee-aware transaction construction, command parsing, fallible node submission, sync tasks, and event logging.

The crate is intentionally split into modules:

- `types.rs` for transaction, UTXO, and status-like data
- `wallet.rs` for wallet state, UTXO selection, history, and summaries
- `fees.rs` for deterministic txid and fee estimation helpers
- `node_client.rs` for the fallible node boundary
- `tasks.rs` for async submit and sync flows
- `log.rs` for structured wallet events
- `command.rs` for parsing thin CLI/TUI-style commands
- `error.rs` for wallet errors

Important exact formats:

```text
wallet-tx:<owner>|to:<recipient>|amount:<amount>|fee:<fee>|inputs:<txid>:<vout>;...
<txid>|outputs:<total_output>|fee:<fee>
owner:<owner>|confirmed:<confirmed>|pending_in:<pending>|pending_txs:<count>|history:<count>
send:<recipient>:<amount_sats>:<fee_sats>
```

`build_transaction` should not mutate wallet state. State changes happen when a transaction is accepted by the node and recorded as pending. A rejected node submission should not remove any UTXOs.

This assignment deliberately uses original simplified fixtures and protocol strings. Do not copy code or data files from the book repository; use the book project only as a reference for how separate wallet, node, miner, and library components fit together.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_08_wallet_integration
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Keep UI concerns out of wallet core logic.
- Make UTXO selection deterministic.
- Log major events as structured data.
- Treat the node boundary as fallible.
- Be ready to explain the differences between this toy wallet and a real Bitcoin wallet.
- Treat CLI/TUI-style commands as a thin boundary over wallet core behavior.
