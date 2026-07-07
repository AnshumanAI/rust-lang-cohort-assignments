use crate::WalletError;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WalletCommand {
    Balance,
    History,
    Sync,
    Send {
        recipient: String,
        amount_sats: u64,
        fee_sats: u64,
    },
}

/// Parse a small wallet command language.
///
/// Supported commands:
/// - `balance`
/// - `history`
/// - `sync`
/// - `send <recipient> <amount_sats> <fee_sats>`
pub fn parse_wallet_command(input: &str) -> Result<WalletCommand, WalletError> {
    // Steps:
    // 1. Trim the input and split on whitespace.
    // 2. Parse the three single-word commands exactly.
    // 3. Parse `send` with exactly three arguments.
    // 4. Reject empty recipient, zero amount, invalid amount, invalid fee,
    //    or extra/missing arguments with `MalformedData` or `InvalidAmount`.
    todo!()
}

/// Render a command into a compact log-friendly label.
pub fn wallet_command_label(command: &WalletCommand) -> String {
    // Steps:
    // 1. Return `balance`, `history`, or `sync` for simple commands.
    // 2. For send, return exactly `send:<recipient>:<amount_sats>:<fee_sats>`.
    todo!()
}
