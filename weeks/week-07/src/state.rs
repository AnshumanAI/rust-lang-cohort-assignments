use std::collections::BTreeMap;

use crate::{Block, NodeError, PeerInfo};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeState {
    pub chain: Vec<Block>,
    pub peers: BTreeMap<String, PeerInfo>,
}

impl NodeState {
    /// Start node state from one validated genesis block.
    pub fn new(genesis: Block) -> Result<Self, NodeError> {
        // Steps:
        // 1. Validate `genesis` with no tip.
        // 2. Store it as the only block.
        // 3. Start with an empty peer map.
        // 4. Return the state.
        todo!()
    }

    /// Return the current chain height.
    pub fn height(&self) -> u64 {
        // Steps:
        // 1. Return the height of the last block.
        // 2. Return 0 if the chain is somehow empty.
        todo!()
    }

    /// Return the current tip hash.
    pub fn tip_hash(&self) -> Option<&str> {
        // Steps:
        // 1. Return the last block's hash as `&str`.
        // 2. Return `None` for an empty chain.
        todo!()
    }

    /// Add or update a peer and return the total peer count.
    pub fn add_peer(&mut self, address: &str) -> usize {
        // Steps:
        // 1. Insert `address` into the peer map.
        // 2. Store the current node height as `last_seen_height`.
        // 3. Return `self.peers.len()`.
        todo!()
    }

    /// Return peer addresses in deterministic sorted order.
    pub fn peer_addresses(&self) -> Vec<String> {
        // Steps:
        // 1. Iterate over the `BTreeMap` keys.
        // 2. Clone each address into a vector.
        // 3. Return the vector.
        todo!()
    }

    /// Append a block if it connects to the current tip.
    pub fn append_block(&mut self, block: Block) -> Result<(), NodeError> {
        // Steps:
        // 1. Validate `block` against `self.chain.last()`.
        // 2. Push it if valid.
        // 3. Return `Ok(())`.
        // 4. Return the validation error if invalid.
        todo!()
    }

    /// Return a block by hash.
    pub fn get_block(&self, hash: &str) -> Option<&Block> {
        // Steps:
        // 1. Iterate over the chain.
        // 2. Return the first block with a matching hash.
        // 3. Return `None` if missing.
        todo!()
    }
}
