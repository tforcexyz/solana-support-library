use solana_program_test::{
  ProgramTestContext,
};
use crate::framework::{
  context::{
    create_context,
  }
};
use test_framework::{
  ID as PROGRAM_ID,
};

pub async fn create_test_context(
) -> ProgramTestContext {
  let context = create_context(
    &[
      ("test_framework", PROGRAM_ID)
    ]
  ).await;
  context
}
