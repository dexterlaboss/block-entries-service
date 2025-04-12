
use std::sync::Arc;
use anyhow::{anyhow, Result};
use solana_ledger::{
    blockstore::Blockstore,
    blockstore_options::{AccessType, BlockstoreOptions},
};
use solana_sdk::hash::Hash;

use crate::config::Config;

pub struct Ledger {
    blockstore: Arc<Blockstore>,
}

impl Ledger {
    /// Opens the ledger (RocksDB) at `config.ledger_path` in Secondary mode.
    pub fn open_ledger(config: &Config) -> Result<Arc<Ledger>> {
        let mut opts = BlockstoreOptions::default();
        opts.access_type = AccessType::Secondary;

        let blockstore = Blockstore::open_with_options(&config.ledger_path, opts)
            .map_err(|e| anyhow!("Failed to open ledger: {:?}", e))?;

        Ok(Arc::new(Self {
            blockstore: Arc::new(blockstore),
        }))
    }

    /// Reads all entries in the given `slot`, returning a list of `BlockEntry`.
    pub fn read_slot_entries(&self, slot: u64) -> Result<Vec<BlockEntry>> {
        let entries = self
            .blockstore
            .get_slot_entries(slot, 0)
            .map_err(|e| anyhow!("Failed to read slot {}: {}", slot, e))?;

        let mut results = Vec::with_capacity(entries.len());
        let mut running_tx_index = 0_usize;

        for (i, entry) in entries.into_iter().enumerate() {
            let block_entry = BlockEntry {
                index: i,
                num_hashes: entry.num_hashes,
                hash: entry.hash,
                num_transactions: entry.transactions.len() as u64,
                starting_transaction_index: running_tx_index,
            };
            running_tx_index += block_entry.num_transactions as usize;
            results.push(block_entry);
        }

        Ok(results)
    }
}

/// Represents a single block entry.
#[derive(Debug, serde::Serialize)]
pub struct BlockEntry {
    pub index: usize,
    pub num_hashes: u64,
    #[serde(serialize_with = "serialize_hash")]
    pub hash: Hash,
    pub num_transactions: u64,
    pub starting_transaction_index: usize,
}

/// Serializer for `Hash` to base58 string.
fn serialize_hash<S>(hash: &Hash, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let base58_str = bs58::encode(hash).into_string();
    serializer.serialize_str(&base58_str)
}