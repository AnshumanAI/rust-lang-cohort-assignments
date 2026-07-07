use crate::{Block, NodeError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeRequest {
    Ping,
    Height,
    GetTip,
    GetBlock(String),
    SubmitBlock(Block),
    AddPeer(String),
    GetPeers,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum NodeResponse {
    Pong,
    Height(u64),
    Tip(String),
    Accepted(String),
    Rejected(String),
    Block(Block),
    NotFound,
    PeerAdded(usize),
    Peers(Vec<String>),
    Error(String),
}

/// Parse a block in `<hash>|<previous_hash>|<height>|<payload>` format.
pub fn parse_block(input: &str) -> Result<Block, NodeError> {
    // Steps:
    // 1. Split `input` into exactly four fields using `|`.
    // 2. Trim each field.
    // 3. Reject empty hash, previous hash, height, or payload.
    // 4. Parse height as `u64`.
    // 5. Return `NodeError::MalformedMessage` on malformed input.
    todo!()
}

/// Parse one text protocol request.
///
/// Supported commands:
/// - `ping`
/// - `height`
/// - `get_tip`
/// - `get_peers`
/// - `get_block <hash>`
/// - `add_peer <address>`
/// - `submit_block <hash>|<previous_hash>|<height>|<payload>`
pub fn parse_request(line: &str) -> Result<NodeRequest, NodeError> {
    // Steps:
    // 1. Trim trailing whitespace.
    // 2. Match exact commands without arguments first.
    // 3. For commands with arguments, split once on the first space.
    // 4. Reject missing arguments with `MalformedMessage`.
    // 5. Reject unknown commands with `UnknownCommand`.
    todo!()
}

/// Encode a response as one newline-terminated protocol line.
pub fn encode_response(response: &NodeResponse) -> String {
    // Steps:
    // 1. Match every response variant.
    // 2. Return exactly one line ending in `\n`.
    // 3. Use `block <wire_format>` for block responses.
    // 4. Use comma-separated peer addresses for `Peers`.
    todo!()
}

/// Parse a response produced by `encode_response`.
///
/// This is intentionally smaller than a real P2P decoder, but it forces students
/// to handle both directions of a protocol boundary.
pub fn parse_response(line: &str) -> Result<NodeResponse, NodeError> {
    // Steps:
    // 1. Trim the response line.
    // 2. Parse `pong`, `not_found`, `height <n>`, `tip <hash>`,
    //    `accepted <hash>`, `rejected <reason>`, `error <message>`,
    //    `block <wire_block>`, and `peers <a,b,c>`.
    // 3. Return `MalformedMessage` for malformed known responses.
    // 4. Return `UnknownCommand` for unrecognized response prefixes.
    todo!()
}
