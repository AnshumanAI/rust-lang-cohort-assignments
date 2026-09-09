#![allow(unused_variables)]

use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

pub const COINBASE_PREVIOUS_OUTPUT: &str = "-";

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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AmountSummary {
    pub output_count: usize,
    pub total_sats: u64,
    pub spent_sats: u64,
    pub unspent_sats: u64,
}

#[derive(Clone, Debug, Error, PartialEq, Eq)]
pub enum BtcLibError {
    #[error("malformed transaction data")]
    MalformedData,
    #[error("missing transaction")]
    MissingTransaction,
    #[error("empty transaction id")]
    EmptyTxId,
    #[error("missing transaction inputs")]
    MissingInputs,
    #[error("missing transaction outputs")]
    MissingOutputs,
    #[error("zero value output")]
    ZeroValueOutput,
    #[error("empty block")]
    EmptyBlock,
    #[error("duplicate transaction id")]
    DuplicateTxId,
    #[error("invalid hash hex")]
    InvalidHash,
    #[error("io error: {0}")]
    Io(String),
}

pub trait Hashable {
    /// Return stable string material that should be fed into `hash_hex`.
    fn hash_material(&self) -> String;

    /// Compute a SHA-256 hex digest for this value.
    ///
    /// This default implementation is complete; students do not need to edit it.
    fn hash_hex(&self) -> String {
        sha256::digest(self.hash_material())
    }
}

pub trait Validate {
    /// Validate a value and return a specific `BtcLibError` for bad data.
    fn validate(&self) -> Result<(), BtcLibError>;
}

fn status_text(status: TxStatus) -> &'static str {
    match status {
        TxStatus::Spent => "spent",
        TxStatus::Unspent => "unspent",
    }
}

impl From<std::io::Error> for BtcLibError {
    /// Convert an IO error into `BtcLibError::Io` while preserving its message.
    fn from(error: std::io::Error) -> Self {
        BtcLibError::Io(error.to_string())
    }
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
    /// Build a transaction output with a fresh UUID.
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

impl Validate for TxOutput {
    /// Reject zero-value outputs.
    fn validate(&self) -> Result<(), BtcLibError> {
        if self.value_sats == 0 {
            Err(BtcLibError::ZeroValueOutput)
        } else {
            Ok(())
        }
    }
}

impl Transaction {
    /// Build a transaction by copying the txid and storing inputs/outputs.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>) -> Self {
        Self {
            txid: txid.to_string(),
            inputs,
            outputs,
        }
    }

    /// Return true for the simplified coinbase rule used in this assignment.
    ///
    /// A coinbase transaction has txid `"coinbase"` and no inputs.
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
}

impl Hashable for Transaction {
    /// Return deterministic transaction hash material.
    ///
    /// Use exactly this format:
    /// `tx:<txid>|inputs:<prev_txid>:<vout>;...|outputs:<value>:<recipient>:<status>;...`
    ///
    /// Status text must be lowercase: `spent` or `unspent`.
    fn hash_material(&self) -> String {
        let mut material = format!("tx:{}|inputs:", self.txid);
        for input in &self.inputs {
            material.push_str(&format!("{}:{};", input.previous_txid, input.previous_vout));
        }
        material.push_str("|outputs:");
        for output in &self.outputs {
            material.push_str(&format!(
                "{}:{}:{};",
                output.value_sats,
                output.recipient,
                status_text(output.status)
            ));
        }
        material
    }
}

impl Validate for Transaction {
    /// Validate a transaction using the Week 3 model plus Week 4 errors.
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

    /// Sum the total output value of every transaction in the block.
    pub fn total_output_value(&self) -> u64 {
        self.transactions
            .iter()
            .map(|transaction| transaction.total_output_value())
            .sum()
    }

    /// Return the first transaction with the matching txid, if one exists.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.transactions
            .iter()
            .find(|transaction| transaction.txid == txid)
    }
}

impl Hashable for Block {
    /// Return deterministic block hash material.
    ///
    /// Use exactly this format:
    /// `block:<block_hash>|prev:<previous_block_hash>|height:<height>|txs:<txid>;...`
    fn hash_material(&self) -> String {
        let mut material = format!(
            "block:{}|prev:{}|height:{}|txs:",
            self.header.block_hash, self.header.previous_block_hash, self.height
        );
        for transaction in &self.transactions {
            material.push_str(&format!("{};", transaction.txid));
        }
        material
    }
}

impl Validate for Block {
    /// Validate a block and its transactions.
    fn validate(&self) -> Result<(), BtcLibError> {
        if self.transactions.is_empty() {
            return Err(BtcLibError::EmptyBlock);
        }
        for i in 0..self.transactions.len() {
            for j in (i + 1)..self.transactions.len() {
                if self.transactions[i].txid == self.transactions[j].txid {
                    return Err(BtcLibError::DuplicateTxId);
                }
            }
        }
        validate_all(&self.transactions)
    }
}

/// Parse `spent` or `unspent` into a `TxStatus`.
///
/// Trim whitespace, ignore ASCII case, and reject unknown values.
pub fn parse_status(input: &str) -> Result<TxStatus, BtcLibError> {
    match input.trim().to_ascii_lowercase().as_str() {
        "spent" => Ok(TxStatus::Spent),
        "unspent" => Ok(TxStatus::Unspent),
        _ => Err(BtcLibError::MalformedData),
    }
}

/// Parse a previous output reference.
///
/// The coinbase marker is `-`. Normal outpoints use `previous_txid:vout`.
pub fn parse_outpoint(input: &str) -> Result<Option<TxInput>, BtcLibError> {
    let input = input.trim();
    if input == COINBASE_PREVIOUS_OUTPUT {
        return Ok(None);
    }
    let Some((previous_txid, vout)) = input.split_once(':') else {
        return Err(BtcLibError::MalformedData);
    };
    if previous_txid.is_empty() || vout.is_empty() {
        return Err(BtcLibError::MalformedData);
    }
    let previous_vout = match vout.parse::<u32>() {
        Ok(value) => value,
        Err(_) => return Err(BtcLibError::MalformedData),
    };
    Ok(Some(TxInput::new(previous_txid, previous_vout)))
}

/// Parse a row into the Week 3 transaction model.
///
/// Row format: `txid,previous_txid:vout,recipient,amount_sats,status`.
/// For coinbase, use `coinbase,-,recipient,amount_sats,status`.
pub fn parse_transaction(input: &str) -> Result<Transaction, BtcLibError> {
    let fields: Vec<&str> = input.split(',').collect();
    if fields.len() != 5 {
        return Err(BtcLibError::MalformedData);
    }

    let txid = fields[0].trim();
    let previous_output = fields[1].trim();
    let recipient = fields[2].trim();
    let amount = fields[3].trim();
    let status = fields[4].trim();

    if txid.is_empty() || recipient.is_empty() || amount.is_empty() || status.is_empty() {
        return Err(BtcLibError::MalformedData);
    }

    let previous = parse_outpoint(previous_output)?;
    if previous.is_none() && txid != "coinbase" {
        return Err(BtcLibError::MalformedData);
    }

    let amount_sats = match amount.parse::<u64>() {
        Ok(value) => value,
        Err(_) => return Err(BtcLibError::MalformedData),
    };
    if amount_sats == 0 {
        return Err(BtcLibError::MalformedData);
    }

    let status = parse_status(status)?;
    let inputs = match previous {
        Some(input) => vec![input],
        None => Vec::new(),
    };
    let outputs = vec![TxOutput::new(amount_sats, recipient, status)];
    Ok(Transaction::new(txid, inputs, outputs))
}

/// Parse every row into a transaction.
///
/// Stop and return the first error if any row is malformed.
pub fn parse_transactions(lines: &[&str]) -> Result<Vec<Transaction>, BtcLibError> {
    let mut transactions = Vec::new();
    for line in lines {
        transactions.push(parse_transaction(line)?);
    }
    Ok(transactions)
}

/// Parse all valid rows and skip malformed rows.
pub fn valid_transactions_only(lines: &[&str]) -> Vec<Transaction> {
    lines
        .iter()
        .filter_map(|line| parse_transaction(line).ok())
        .collect()
}

/// Build and validate a block from parsed transaction rows.
pub fn build_block_from_rows(
    header: BlockHeader,
    rows: &[&str],
    height: u64,
    network: Network,
) -> Result<Block, BtcLibError> {
    let transactions = parse_transactions(rows)?;
    let block = Block::new(header, transactions, height, network);
    block.validate()?;
    Ok(block)
}

/// Validate every item in order.
///
/// Stop and return the first validation error, otherwise return `Ok(())`.
pub fn validate_all<T: Validate>(items: &[T]) -> Result<(), BtcLibError> {
    for item in items {
        item.validate()?;
    }
    Ok(())
}

/// Return the SHA-256 hex hash for every hashable item, preserving input order.
pub fn hash_all<T: Hashable>(items: &[T]) -> Vec<String> {
    items.iter().map(|item| item.hash_hex()).collect()
}

/// Decode a 64-character SHA-256 hex string into 32 bytes.
pub fn decode_hash_hex(input: &str) -> Result<[u8; 32], BtcLibError> {
    let bytes = match hex::decode(input.trim()) {
        Ok(bytes) => bytes,
        Err(_) => return Err(BtcLibError::InvalidHash),
    };
    bytes.try_into().map_err(|_| BtcLibError::InvalidHash)
}

/// Sum unspent output amounts across all transactions.
pub fn total_unspent(transactions: &[Transaction]) -> u64 {
    transactions
        .iter()
        .flat_map(|transaction| transaction.outputs.iter())
        .filter(|output| output.is_unspent())
        .map(|output| output.value_sats)
        .sum()
}

/// Return a borrowed transaction with the matching txid, if one exists.
pub fn find_by_txid<'a>(transactions: &'a [Transaction], txid: &str) -> Option<&'a Transaction> {
    transactions
        .iter()
        .find(|transaction| transaction.txid == txid)
}

/// Return the matching transaction or `BtcLibError::MissingTransaction`.
pub fn require_transaction<'a>(
    transactions: &'a [Transaction],
    txid: &str,
) -> Result<&'a Transaction, BtcLibError> {
    find_by_txid(transactions, txid).ok_or(BtcLibError::MissingTransaction)
}

/// Build an amount summary from all transaction outputs.
pub fn summarize_amounts(transactions: &[Transaction]) -> AmountSummary {
    let mut output_count = 0;
    let mut total_sats = 0;
    let mut spent_sats = 0;
    let mut unspent_sats = 0;

    for transaction in transactions {
        for output in &transaction.outputs {
            output_count += 1;
            total_sats += output.value_sats;
            match output.status {
                TxStatus::Spent => spent_sats += output.value_sats,
                TxStatus::Unspent => unspent_sats += output.value_sats,
            }
        }
    }

    AmountSummary {
        output_count,
        total_sats,
        spent_sats,
        unspent_sats,
    }
}
