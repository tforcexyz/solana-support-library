pub mod context;
pub mod event;

use anchor_lang::prelude::*;
use solana_program::{
  instruction::{
    Instruction,
  },
};
use context::*;
use event::*;

declare_id!("TFXeSSo3gA2uXnZfwtHNodvAQnkMMdkZ1soXPqjXaem");

#[program]
mod tfx_test_framework {
  use super::*;

  pub fn announce(
    ctx: Context<AnnounceContext>,
    content: Vec<u8>,
  ) -> Result<()> {

    let sender = &ctx.accounts.sender;
    let clock = Clock::get().unwrap();

    emit!(AnnouncedEvent {
      address: sender.key(),
      content,
      slot: clock.slot,
      timestamp: clock.unix_timestamp,
    });

    Ok(())
  }

  pub fn forward(
    ctx: Context<ForwardContext>,
    data: Vec<u8>,
  ) -> Result<()> {

    let accounts = ctx.remaining_accounts;

    let mut keys: Vec<AccountMeta> = Vec::new();
    let mut account_infos: Vec<AccountInfo> = Vec::new();
    for i in 0..accounts.len()-1 {
      let account_info = &accounts[i];
      account_infos.push(account_info.clone());
      let account_meta = if account_info.is_writable {
        AccountMeta::new(account_info.key(), account_info.is_signer)
      }
      else {
        AccountMeta::new_readonly(account_info.key(), account_info.is_signer)
      };
      keys.push(account_meta);
    }
    let target_program_id = accounts[accounts.len()-1].key();

    let instruction = Instruction{
      data: data,
      accounts: keys,
      program_id: target_program_id,
    };

    solana_program::program::invoke(&instruction, &account_infos[..])
      .expect("CPI call failed");

    Ok(())
  }
}
