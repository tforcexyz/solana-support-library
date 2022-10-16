use anchor_lang::prelude::*;

#[event]
pub struct AnnouncedEvent {
  pub address: Pubkey,
  pub content: Vec<u8>,
  pub timestamp: i64,
  pub slot: u64,
}

#[event]
pub struct MathCalculatonEvent {
  pub first_number: u16,
  pub second_number: u16,
  pub result: u16,
}
