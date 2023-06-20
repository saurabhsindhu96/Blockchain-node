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
impl Blockchain {}