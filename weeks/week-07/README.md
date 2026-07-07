# Week 7 Assignment: Async Miner-to-Node Networking

## Concepts

This week introduces async Rust through a small Bitcoin-style node. The node owns chain state in one task and other parts of the program communicate with it through bounded channels. A tiny text protocol is used so the assignment can focus on async boundaries, parsing, dispatch, validation, and deterministic tests.

You will work with Tokio, `async` functions, `mpsc` channels, `oneshot` responses, TCP listeners, request enums, response enums, peer tracking, and block submission.

## Student Work

Open the files under `src/` and replace every `todo!()` with a working implementation. Keep public names and signatures unchanged.

The block constructor is just setup. The main work is the node pipeline: decode a protocol line, dispatch it into shared state, send a response through bounded async channels, and keep TCP handling separate from state mutation.

The crate is intentionally split into modules:

- `error.rs` for stable node errors
- `types.rs` for block and peer types
- `protocol.rs` for request/response parsing and encoding
- `state.rs` for chain and peer state
- `dispatcher.rs` for request handling and state-manager tasks
- `server.rs` for TCP line handling and shutdown

The structure is inspired by the book project's node component, but the protocol and fixtures here are original to this assignment. Do not copy node code or CBOR block files from the book repository.

Supported request lines:

```text
ping
height
get_tip
get_peers
get_block <hash>
add_peer <address>
submit_block <hash>|<previous_hash>|<height>|<payload>
```

Block wire format is:

```text
<hash>|<previous_hash>|<height>|<payload>
```

Responses must be newline-terminated. You also implement response parsing so the protocol boundary works in both directions. The state manager should serialize access to `NodeState`, and the TCP server should shut down cleanly when its shutdown signal fires.

## Running Tests

From this week:

```bash
cargo test
```

From the repository root:

```bash
cargo test -p week_07_async_node
```

Run formatting before submission:

```bash
cargo fmt --all
```

## Notes

- Keep parsing separate from request handling.
- Do not let malformed network messages crash the node.
- Use bounded channels for backpressure.
- Make shutdown deterministic so tests do not hang.
- Treat request/response parsing as a protocol boundary, not as string utilities sprinkled through the node.
