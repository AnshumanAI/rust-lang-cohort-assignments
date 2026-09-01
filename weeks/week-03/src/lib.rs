#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum Network {
    Mainnet,
    Testnet,
    Signet,
    Regtest,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum TxStatus {
    Spent,
    Unspent,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationError {
    EmptyTxId,
    MissingInputs,
    MissingOutputs,
    ZeroValueOutput,
    EmptyBlock,
    DuplicateTxId,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxInput {
    pub previous_txid: String,
    pub previous_vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOutput {
    pub value_sats: u64,
    pub unique_id: Uuid,
    pub recipient: String,
    pub status: TxStatus,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct BlockHeader {
    pub block_hash: String,
    pub previous_block_hash: String,
    pub merkle_root: String,
    pub timestamp: u64,
    pub nonce: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        Self {
            previous_txid: previous_txid.to_string(),
            previous_vout,
        }
    }
}

impl TxOutput {
    /// Build a transaction output by copying the recipient and storing value/status.
    pub fn new(value_sats: u64, recipient: &str, status: TxStatus) -> Self {
        Self {
            value_sats,
            unique_id: Uuid::new_v4(),
            recipient: recipient.to_string(),
            status,
        }
    }

    /// Return true when this output status is `TxStatus::Unspent`.
    pub fn is_unspent(&self) -> bool {
        self.status == TxStatus::Unspent
    }
}

impl Transaction {
    /// Build a transaction by copying the txid and storing the provided inputs
    /// and outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            txid: txid.to_string(),
            inputs,
            outputs,
        }
    }

    /// Return true for the simplified coinbase rule used in this assignment:
    /// txid is `"coinbase"` and there are no inputs.
    pub fn is_coinbase(&self) -> bool {
        self.txid == "coinbase" && self.inputs.is_empty()
    }

    /// Sum the satoshi value of every output in this transaction.
    pub fn total_output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value_sats).sum()
    }

    /// Count outputs whose status is `TxStatus::Unspent`.
    pub fn unspent_output_count(&self) -> usize {
        self.outputs
            .iter()
            .filter(|output| output.status == TxStatus::Unspent)
            .count()
    }

    /// Count outputs whose status is `TxStatus::Spent`.
    pub fn spent_output_count(&self) -> usize {
        self.outputs
            .iter()
            .filter(|output| output.status == TxStatus::Spent)
            .count()
    }

    /// Validate this transaction using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.txid.is_empty() {
            return Err(ValidationError::EmptyTxId);
        }
        if !self.is_coinbase() && self.inputs.is_empty() {
            return Err(ValidationError::MissingInputs);
        }
        if self.outputs.is_empty() {
            return Err(ValidationError::MissingOutputs);
        }
        if self.outputs.iter().any(|output| output.value_sats == 0) {
            return Err(ValidationError::ZeroValueOutput);
        }
        Ok(())
    }
}

impl Identifiable for Transaction {
    /// Return this transaction's txid.
    fn id(&self) -> &str {
        self.txid.as_str()
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
        Self {
            block_hash: block_hash.to_string(),
            previous_block_hash: previous_block_hash.to_string(),
            merkle_root: merkle_root.to_string(),
            timestamp,
            nonce,
        }
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
        Self {
            header,
            transactions,
            height,
            network,
        }
    }

    /// Return how many transactions are in this block.
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Sum the total output value of all transactions in this block.
    pub fn total_output_value(&self) -> u64 {
        self.transactions
            .iter()
            .map(|transaction| transaction.total_output_value())
            .sum()
    }

    /// Return the first coinbase transaction in this block, if one exists.
    pub fn coinbase_transaction(&self) -> Option<&Transaction> {
        self.transactions
            .iter()
            .find(|transaction| transaction.is_coinbase())
    }

    /// Return a borrowed transaction with the matching txid, if one exists.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.transactions
            .iter()
            .find(|transaction| transaction.txid == txid)
    }

    /// Validate this block using the rules in the README.
    ///
    /// Return the first matching `ValidationError`, otherwise `Ok(())`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.transactions.is_empty() {
            return Err(ValidationError::EmptyBlock);
        }
        for i in 0..self.transactions.len() {
            for j in (i + 1)..self.transactions.len() {
                if self.transactions[i].txid == self.transactions[j].txid {
                    return Err(ValidationError::DuplicateTxId);
                }
            }
        }
        for transaction in &self.transactions {
            transaction.validate()?;
        }
        Ok(())
    }
}

impl Identifiable for Block {
    /// Return this block's block hash.
    fn id(&self) -> &str {
        self.header.block_hash.as_str()
    }
}

/// Return the Bitcoin network magic value for a network.
pub fn network_magic(network: Network) -> u32 {
    match network {
        Network::Mainnet => 0xD9B4BEF9,
        Network::Testnet => 0x0709110B,
        Network::Signet => 0x40CF030A,
        Network::Regtest => 0xDAB5BFFA,
    }
}

/// Convert a known network magic value back to a `Network`.
///
/// Return `None` for unknown magic values.
pub fn network_from_magic(magic: u32) -> Option<Network> {
    match magic {
        0xD9B4BEF9 => Some(Network::Mainnet),
        0x0709110B => Some(Network::Testnet),
        0x40CF030A => Some(Network::Signet),
        0xDAB5BFFA => Some(Network::Regtest),
        _ => None,
    }
}

/// Count unspent outputs across all transactions.
pub fn count_unspent_outputs(transactions: &[Transaction]) -> usize {
    transactions
        .iter()
        .map(|transaction| transaction.unspent_output_count())
        .sum()
}

/// Sum output values whose recipient exactly matches `recipient`.
pub fn total_value_for_recipient(transactions: &[Transaction], recipient: &str) -> u64 {
    transactions
        .iter()
        .flat_map(|transaction| transaction.outputs.iter())
        .filter(|output| output.recipient == recipient)
        .map(|output| output.value_sats)
        .sum()
}

/// Compare two values through the `Identifiable` trait.
pub fn have_same_id<T: Identifiable, U: Identifiable>(left: &T, right: &U) -> bool {
    left.id() == right.id()
}

/// Collect ids from dynamic trait objects into owned strings.
pub fn collect_ids(items: &[Box<dyn Identifiable>]) -> Vec<String> {
    items.iter().map(|item| item.id().to_string()).collect()
}
