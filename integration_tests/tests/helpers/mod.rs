use solana_program_test::{ProgramTest, ProgramTestContext};

/// Get ProgramTestContext with myproject loaded.
pub async fn program_test_context() -> ProgramTestContext {
    let mut program_test = ProgramTest::default();
    program_test.add_program("myproject", myproject_client::programs::MYPROJECT_ID, None);
    let ctx = program_test.start_with_context().await;
    ctx
}
