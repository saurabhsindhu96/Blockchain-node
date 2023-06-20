use super::blockchain::Blockchain;
use chrono::prelude::*;
use sha2::{Sha256, Digest};
use serde::{Deserialize, Serialize};

// `Block`, A struct that represents a block in a Blockchain.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
   // The index in which the current block is stored.
   pub index: u64,
   // The time the current block is created.
   pub timestamp: u64,

   // The block's proof of work.
   pub proof_of_work: u64,
   // The previous block hash.
   pub previous_hash: String,
   // The current block hash.
   pub hash: String
}
impl Block {}