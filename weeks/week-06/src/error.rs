use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum MinerError {
    #[error("missing utxo: {0}")]
    MissingUtxo(String),
    #[error("duplicate utxo: {0}")]
    DuplicateUtxo(String),
    #[error("invalid spend: {0}")]
    InvalidSpend(String),
    #[error("duplicate mempool transaction: {0}")]
    DuplicateMempoolTransaction(String),
    #[error("transaction not found: {0}")]
    TransactionNotFound(String),
    #[error("invalid difficulty")]
    InvalidDifficulty,
    #[error("empty candidate block")]
    EmptyCandidate,
    #[error("no solution found")]
    NoSolution,
    #[error("worker channel closed")]
    ChannelClosed,
    #[error("invalid block")]
    InvalidBlock,
}
