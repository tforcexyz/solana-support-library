use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {

  #[msg("Content is too long")]
  ContentTooLong,
}
