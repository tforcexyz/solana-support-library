use anchor_lang::prelude::*;

#[event]
pub struct AnnouncedEvent {
  pub address: Pubkey,
  pub content: Vec<u8>,
  pub timestamp: i64,
  pub slot: u64,
}
