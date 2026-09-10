use crate::{Block, BtcLibError, Transaction, TxStatus};

fn status_text(status: TxStatus) -> &'static str {
    match status {
        TxStatus::Spent => "spent",
        TxStatus::Unspent => "unspent",
    }
}

pub trait Hashable {
    /// Return stable material that will be hashed.
    fn hash_material(&self) -> String;

    /// Hash `hash_material()` with SHA-256 and return lowercase hex.
    fn hash_hex(&self) -> String {
        sha256::digest(self.hash_material())
    }
}

impl Hashable for Transaction {
    /// Return deterministic transaction material.
    ///
    /// Use exactly:
    /// `tx:<txid>|inputs:<previous_txid>:<previous_vout>;...|outputs:<value>:<recipient>:<status>;...`
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

impl Hashable for Block {
    /// Return deterministic block material.
    ///
    /// Use exactly:
    /// `block:<block_hash>|prev:<previous_block_hash>|merkle:<merkle_root>|height:<height>|txs:<txid>;...`
    fn hash_material(&self) -> String {
        let mut material = format!(
            "block:{}|prev:{}|merkle:{}|height:{}|txs:",
            self.header.block_hash,
            self.header.previous_block_hash,
            self.header.merkle_root,
            self.height
        );
        for transaction in &self.transactions {
            material.push_str(&format!("{};", transaction.txid));
        }
        material
    }
}

/// Hash two child hashes into their parent merkle node.
pub fn pair_hash(left: &str, right: &str) -> String {
    sha256::digest(format!("{left}{right}"))
}

/// Calculate a simple merkle root from transaction hashes.
///
/// If a level has an odd number of hashes, duplicate the last hash before pairing.
pub fn calculate_merkle_root(transactions: &[Transaction]) -> Result<String, BtcLibError> {
    if transactions.is_empty() {
        return Err(BtcLibError::EmptyBlock);
    }

    let mut level: Vec<String> = transactions
        .iter()
        .map(|transaction| transaction.hash_hex())
        .collect();

    while level.len() > 1 {
        let mut next = Vec::new();
        let mut index = 0;
        while index < level.len() {
            let left = &level[index];
            let right = if index + 1 < level.len() {
                &level[index + 1]
            } else {
                left
            };
            next.push(pair_hash(left, right));
            index += 2;
        }
        level = next;
    }

    Ok(level[0].clone())
}

/// Validate that the block header stores the merkle root for its transactions.
pub fn validate_merkle_root(block: &Block) -> Result<(), BtcLibError> {
    let expected = calculate_merkle_root(&block.transactions)?;
    if expected == block.header.merkle_root {
        Ok(())
    } else {
        Err(BtcLibError::InvalidMerkleRoot)
    }
}
