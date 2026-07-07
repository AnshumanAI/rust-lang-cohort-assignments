use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::BtcLibError;

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

pub trait Validate {
    /// Validate a value and return the first precise library error.
    fn validate(&self) -> Result<(), BtcLibError>;
}

impl TxInput {
    /// Build a transaction input by copying the previous txid and storing the output index.
    pub fn new(previous_txid: &str, previous_vout: u32) -> Self {
        Self {
            previous_txid: previous_txid.to_string(),
            previous_vout,
        }
    }
}

impl TxOutput {
    /// Build a transaction output with a fresh UUID.
    pub fn new(value_sats: u64, recipient: &str, status: TxStatus) -> Self {
        Self {
            value_sats,
            unique_id: Uuid::new_v4(),
            recipient: recipient.to_string(),
            status,
        }
    }

    /// Return true only for unspent outputs.
    pub fn is_unspent(&self) -> bool {
        self.status == TxStatus::Unspent
    }
}

impl Validate for TxOutput {
    /// Reject zero-value outputs.
    fn validate(&self) -> Result<(), BtcLibError> {
        if self.value_sats == 0 {
            return Err(BtcLibError::ZeroValueOutput);
        }
        Ok(())
    }
}

impl Transaction {
    /// Build a transaction by copying the txid and moving inputs and outputs in.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            txid: txid.to_string(),
            inputs,
            outputs,
        }
    }

    /// A simplified coinbase transaction has txid `"coinbase"` and no inputs.
    pub fn is_coinbase(&self) -> bool {
        self.txid == "coinbase" && self.inputs.is_empty()
    }

    /// Sum every output amount in this transaction.
    pub fn total_output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value_sats).sum()
    }

    /// Count outputs whose status is unspent.
    pub fn unspent_output_count(&self) -> usize {
        self.outputs
            .iter()
            .filter(|output| output.is_unspent())
            .count()
    }
}

impl Validate for Transaction {
    /// Validate the transaction without panicking.
    fn validate(&self) -> Result<(), BtcLibError> {
        if self.txid.is_empty() {
            return Err(BtcLibError::EmptyTxId);
        }
        if !self.is_coinbase() && self.inputs.is_empty() {
            return Err(BtcLibError::MissingInputs);
        }
        if self.outputs.is_empty() {
            return Err(BtcLibError::MissingOutputs);
        }
        for output in &self.outputs {
            output.validate()?;
        }
        Ok(())
    }
}

impl BlockHeader {
    /// Build a block header by copying string fields and storing numbers.
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
    /// Build a block from the supplied header, transactions, height, and network.
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

    /// Return how many transactions are in the block.
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }

    /// Return the first transaction with the requested txid.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.transactions
            .iter()
            .find(|transaction| transaction.txid == txid)
    }

    /// Sum every transaction output amount in this block.
    pub fn total_output_value(&self) -> u64 {
        self.transactions
            .iter()
            .map(Transaction::total_output_value)
            .sum()
    }
}

impl Validate for Block {
    /// Validate block structure and contained transactions.
    fn validate(&self) -> Result<(), BtcLibError> {
        if self.transactions.is_empty() {
            return Err(BtcLibError::EmptyBlock);
        }
        let mut seen = std::collections::HashSet::new();
        for transaction in &self.transactions {
            if !seen.insert(transaction.txid.as_str()) {
                return Err(BtcLibError::DuplicateTxId);
            }
            transaction.validate()?;
        }
        Ok(())
    }
}
