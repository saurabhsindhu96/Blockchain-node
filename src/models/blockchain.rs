use chrono::prelude::*;
// Internal module
use super::block::Block;

type Blocks = Vec<Block>;
// `Blockchain` A struct that represents the blockchain.
#[derive(Debug)]
pub struct Blockchain {
  // The first block to be added to the chain.
  pub genesis_block: Block,
  // The storage for blocks.
  pub chain: Blocks,
  // Minimum amount of work required to validate a block.
  pub difficulty: usize
}
impl Blockchain {
    pub fn new(difficulty: usize) -> Self {
      // First block in the chain.
      let mut genesis_block = Block {
         index: 0,
         timestamp: Utc::now().timestamp_millis() as u64,
         proof_of_work: u64::default(),
         previous_hash: String::default(),
         hash: String::default()
      };
      // Create chain starting from the genesis chain.
      let mut chain = Vec::new();
      chain.push(genesis_block.clone());
      // Create a blockchain Instance.
      let blockchain = Blockchain {
         genesis_block,
         chain,
         difficulty
      };
      blockchain
    }
 }