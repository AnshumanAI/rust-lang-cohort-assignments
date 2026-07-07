use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum WalletError {
    #[error("missing utxo: {0}")]
    MissingUtxo(String),
    #[error("insufficient funds")]
    InsufficientFunds,
    #[error("invalid amount")]
    InvalidAmount,
    #[error("node rejected transaction: {0}")]
    NodeRejected(String),
    #[error("malformed data")]
    MalformedData,
}
