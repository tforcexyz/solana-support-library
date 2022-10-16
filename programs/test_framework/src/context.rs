use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AnnounceContext<'info> {

  pub sender: Signer<'info>,
}

#[derive(Accounts)]
pub struct ForwardContext {}

#[derive(Accounts)]
pub struct MultiplyContext {}
