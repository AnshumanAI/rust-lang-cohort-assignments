use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct OutPoint {
    pub txid: String,
    pub vout: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxInput {
    pub previous_output: OutPoint,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct TxOutput {
    pub value_sats: u64,
    pub recipient: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Transaction {
    pub txid: String,
    pub inputs: Vec<TxInput>,
    pub outputs: Vec<TxOutput>,
    pub fee_sats: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct WalletUtxo {
    pub outpoint: OutPoint,
    pub value_sats: u64,
    pub owner: String,
    pub confirmations: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeStatus {
    pub height: u64,
    pub tip_hash: String,
}

impl OutPoint {
    /// Build an outpoint from a txid and output index.
    pub fn new(txid: &str, vout: u32) -> Self {
        Self {
            txid: txid.to_string(),
            vout,
        }
    }

    /// Return `txid:vout`.
    pub fn label(&self) -> String {
        format!("{}:{}", self.txid, self.vout)
    }
}

impl TxInput {
    /// Build an input that spends one previous output.
    pub fn new(txid: &str, vout: u32) -> Self {
        Self {
            previous_output: OutPoint::new(txid, vout),
        }
    }
}

impl TxOutput {
    /// Build an output by copying the recipient and storing the amount.
    pub fn new(value_sats: u64, recipient: &str) -> Self {
        Self {
            value_sats,
            recipient: recipient.to_string(),
        }
    }
}

impl Transaction {
    /// Build a transaction from explicit fields.
    pub fn new(txid: &str, inputs: Vec<TxInput>, outputs: Vec<TxOutput>, fee_sats: u64) -> Self {
        Self {
            txid: txid.to_string(),
            inputs,
            outputs,
            fee_sats,
        }
    }

    /// Sum every output amount.
    pub fn total_output_value(&self) -> u64 {
        self.outputs.iter().map(|output| output.value_sats).sum()
    }

    /// Return true when this transaction pays the owner in any output.
    pub fn pays_owner(&self, owner: &str) -> bool {
        self.outputs.iter().any(|output| output.recipient == owner)
    }
}

impl WalletUtxo {
    /// Build a wallet UTXO from explicit fields.
    pub fn new(txid: &str, vout: u32, value_sats: u64, owner: &str, confirmations: u32) -> Self {
        Self {
            outpoint: OutPoint::new(txid, vout),
            value_sats,
            owner: owner.to_string(),
            confirmations,
        }
    }
}
