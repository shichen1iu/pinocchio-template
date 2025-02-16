use helpers::program_test_context;
use myproject_client::instructions::InitializeBuilder;
use solana_sdk::{signer::Signer, transaction::Transaction};

mod helpers;

#[tokio::test]
async fn initialize_success() {
    let ctx = program_test_context().await;

    let ix = InitializeBuilder::new().instruction();

    let transaction = Transaction::new_signed_with_payer(
        &[ix],
        Some(&ctx.payer.pubkey()),
        &[&ctx.payer],
        ctx.last_blockhash,
    );
    ctx.banks_client
        .process_transaction(transaction)
        .await
        .unwrap();
}
