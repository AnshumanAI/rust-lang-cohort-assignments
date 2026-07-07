use crate::{NodeStatus, Transaction, WalletError};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NodeClient {
    pub accepted_transactions: Vec<Transaction>,
    pub height: u64,
    pub tip_hash: String,
    pub history: Vec<Transaction>,
    pub reject_next: Option<String>,
}

impl NodeClient {
    /// Create a mock node client for assignment tests.
    pub fn new(height: u64, tip_hash: &str) -> Self {
        // Steps:
        // 1. Store height and tip hash.
        // 2. Start with empty accepted transaction and history lists.
        // 3. Set `reject_next` to `None`.
        todo!()
    }

    /// Submit a transaction to the node.
    pub async fn submit_transaction(
        &mut self,
        transaction: Transaction,
    ) -> Result<String, WalletError> {
        // Steps:
        // 1. If `reject_next` contains a reason, take it and return `NodeRejected(reason)`.
        // 2. Otherwise clone or move the transaction into `accepted_transactions`.
        // 3. Also add it to node history.
        // 4. Return the accepted txid.
        todo!()
    }

    /// Fetch current node status.
    pub async fn status(&self) -> Result<NodeStatus, WalletError> {
        // Steps:
        // 1. Return `NodeStatus { height, tip_hash }`.
        todo!()
    }

    /// Return transactions from node history that involve `owner`.
    pub async fn wallet_history(&self, owner: &str) -> Result<Vec<Transaction>, WalletError> {
        // Steps:
        // 1. Iterate over node history.
        // 2. Keep transactions where any output pays `owner`.
        // 3. Return the matching transactions.
        todo!()
    }
}
