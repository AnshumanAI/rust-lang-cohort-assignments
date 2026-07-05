#![allow(unused_variables)]

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxStatus {
    Spent,
    Unspent,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationError {
    EmptyTxId,
    MissingInputs,
    MissingOutputs,
    ZeroValueOutput,
    EmptyBlock,
    DuplicateTxId,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxInput {
    pub previous_txid: String,
    pub previous_vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TxOutput {
    pub value_sats: u64,
    pub recipient: String,
    pub status: TxStatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BlockHeader {
    pub block_hash: String,
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Block {
    pub header: BlockHeader,
    pub transactions: Vec<Transaction>,
    pub height: u64,
    pub network: Network,
}

pub trait Identifiable {
    /// Return the stable identifier for this value.
    fn id(&self) -> &str;
}

impl TxInput {
    /// Build a transaction input by copying the previous txid and storing vout.
    pub fn new(previous_txid: &str, previous_vout: u32) -> Self {
        todo!()
    }
}

impl TxOutput {
    /// Build a transaction output by copying the recipient and storing value/status.
    pub fn new(value_sats: u64, recipient: &str, status: TxStatus) -> Self {
        todo!()
    }

    /// Return true when this output status is `TxStatus::Unspent`.
    pub fn is_unspent(&self) -> bool {
        todo!()
    }
}

impl Transaction {
    /// Build a transaction by copying the txid and storing the provided inputs
    /// and outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        todo!()
    }

    /// Return true for the simplified coinbase rule used in this assignment:
    /// txid is `"coinbase"` and there are no inputs.
    pub fn is_coinbase(&self) -> bool {
        todo!()
    }

    /// Sum the satoshi value of every output in this transaction.
    pub fn total_output_value(&self) -> u64 {
        todo!()
    }

    /// Count outputs whose status is `TxStatus::Unspent`.
    pub fn unspent_output_count(&self) -> usize {
        todo!()
    }

    /// Count outputs whose status is `TxStatus::Spent`.
    pub fn spent_output_count(&self) -> usize {
        todo!()
    }

    /// Validate this transaction using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        todo!()
    }
}

impl Identifiable for Transaction {
    /// Return this transaction's txid.
    fn id(&self) -> &str {
        todo!()
    }
}

impl BlockHeader {
    /// Build a block header by copying the string fields and storing timestamp
    /// and nonce.
    pub fn new(
        block_hash: &str,
        previous_block_hash: &str,
        merkle_root: &str,
        timestamp: u64,
        nonce: u64,
    ) -> Self {
        todo!()
    }
}

impl Block {
    /// Build a block from the provided header, transactions, height, and network.
    pub fn new(
        header: BlockHeader,
        transactions: Vec<Transaction>,
        height: u64,
        network: Network,
    ) -> Self {
        todo!()
    }

    /// Return how many transactions are in this block.
    pub fn transaction_count(&self) -> usize {
        todo!()
    }

    /// Sum the total output value of all transactions in this block.
    pub fn total_output_value(&self) -> u64 {
        todo!()
    }

    /// Return the first coinbase transaction in this block, if one exists.
    pub fn coinbase_transaction(&self) -> Option<&Transaction> {
        todo!()
    }

    /// Return a borrowed transaction with the matching txid, if one exists.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        todo!()
    }

    /// Validate this block using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        todo!()
    }
}

impl Identifiable for Block {
    /// Return this block's block hash.
    fn id(&self) -> &str {
        todo!()
    }
}

/// Return the Bitcoin network magic value for a network.
pub fn network_magic(network: Network) -> u32 {
    todo!()
}

/// Convert a known network magic value back to a `Network`.
///
/// Return `None` for unknown magic values.
pub fn network_from_magic(magic: u32) -> Option<Network> {
    todo!()
}

/// Count unspent outputs across all transactions.
pub fn count_unspent_outputs(transactions: &[Transaction]) -> usize {
    todo!()
}

/// Sum output values whose recipient exactly matches `recipient`.
pub fn total_value_for_recipient(transactions: &[Transaction], recipient: &str) -> u64 {
    todo!()
}

/// Compare two values through the `Identifiable` trait.
pub fn have_same_id<T: Identifiable, U: Identifiable>(left: &T, right: &U) -> bool {
    todo!()
}

/// Collect ids from dynamic trait objects into owned strings.
pub fn collect_ids(items: &[Box<dyn Identifiable>]) -> Vec<String> {
    todo!()
}
