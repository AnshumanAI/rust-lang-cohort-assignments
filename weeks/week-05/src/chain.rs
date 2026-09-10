use serde::{Deserialize, Serialize};

use crate::{validate_merkle_root, Block, BtcLibError, Network, Transaction, Validate};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Blockchain {
    pub network: Network,
    pub blocks: Vec<Block>,
}

impl Blockchain {
    /// Create an empty chain for the selected network.
    pub fn new(network: Network) -> Self {
        Self {
            network,
            blocks: Vec::new(),
        }
    }

    /// Create a chain that starts with a validated genesis block.
    pub fn from_genesis(genesis: Block) -> Result<Self, BtcLibError> {
        genesis.validate()?;
        validate_merkle_root(&genesis)?;
        let mut chain = Self::new(genesis.network);
        chain.blocks.push(genesis);
        Ok(chain)
    }

    /// Return the current chain height.
    ///
    /// Empty chains return 0. Non-empty chains return the height of the tip block.
    pub fn height(&self) -> u64 {
        self.tip().map(|block| block.height).unwrap_or(0)
    }

    /// Return the current tip block.
    pub fn tip(&self) -> Option<&Block> {
        self.blocks.last()
    }

    /// Return the current tip hash, if the chain has a tip.
    pub fn tip_hash(&self) -> Option<&str> {
        self.tip().map(|block| block.header.block_hash.as_str())
    }

    /// Append a validated block to the chain.
    pub fn append_block(&mut self, block: Block) -> Result<(), BtcLibError> {
        block.validate()?;
        validate_merkle_root(&block)?;
        match self.tip() {
            None => {
                if block.height != 0 {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            }
            Some(tip) => {
                if block.header.previous_block_hash != tip.header.block_hash
                    || block.height != tip.height + 1
                {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            }
        }
        self.blocks.push(block);
        Ok(())
    }

    /// Find a block by its header hash.
    pub fn find_block_by_hash(&self, block_hash: &str) -> Option<&Block> {
        self.blocks
            .iter()
            .find(|block| block.header.block_hash == block_hash)
    }

    /// Find a transaction anywhere in the chain.
    pub fn find_transaction(&self, txid: &str) -> Option<&Transaction> {
        self.blocks
            .iter()
            .find_map(|block| block.find_transaction(txid))
    }

    /// Count all transactions across all blocks.
    pub fn total_transactions(&self) -> usize {
        self.blocks
            .iter()
            .map(|block| block.transaction_count())
            .sum()
    }

    /// Validate every block and every link in the chain.
    pub fn validate(&self) -> Result<(), BtcLibError> {
        if self.blocks.is_empty() {
            return Err(BtcLibError::EmptyChain);
        }
        for index in 0..self.blocks.len() {
            let block = &self.blocks[index];
            block.validate()?;
            validate_merkle_root(block)?;
            if index > 0 {
                let previous = &self.blocks[index - 1];
                if block.header.previous_block_hash != previous.header.block_hash
                    || block.height != previous.height + 1
                {
                    return Err(BtcLibError::InvalidPreviousHash);
                }
            }
        }
        Ok(())
    }
}

/// Return a lowercase label for the network.
pub fn network_label(network: Network) -> &'static str {
    match network {
        Network::Mainnet => "mainnet",
        Network::Testnet => "testnet",
        Network::Signet => "signet",
        Network::Regtest => "regtest",
    }
}

/// Build a compact chain summary string.
///
/// Use exactly:
/// `network:<network>|height:<height>|blocks:<count>|tip:<tip_hash>|txs:<total_transactions>`
///
/// For an empty chain, use `tip:none`.
pub fn chain_summary(chain: &Blockchain) -> String {
    format!(
        "network:{}|height:{}|blocks:{}|tip:{}|txs:{}",
        network_label(chain.network),
        chain.height(),
        chain.blocks.len(),
        chain.tip_hash().unwrap_or("none"),
        chain.total_transactions()
    )
}
