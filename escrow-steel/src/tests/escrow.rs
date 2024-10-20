use super::*;
use anchor_lang::prelude::*;
use solana_program_test::*;
use solana_sdk::{signature::Keypair, signer::Signer, transaction::Transaction};

#[tokio::test]
async fn test_create_and_release_escrow() {
    let (mut banks_client, payer, escrow_pubkey) = setup().await;

    // Test Create Escrow
    let create_escrow_tx = Transaction::new_signed_with_payer(
        &[escrow::create_escrow(
            accounts::CreateEscrow {
                escrow: escrow_pubkey,
                payer: payer.pubkey(),
            },
            1000,
        )],
        Some(&payer.pubkey()),
        &[&payer],
        banks_client.get_recent_blockhash().await.unwrap(),
    );

    banks_client.process_transaction(create_escrow_tx).await.unwrap();

    // Test Release Escrow
    let release_escrow_tx = Transaction::new_signed_with_payer(
        &[escrow::release_escrow(
            accounts::ReleaseEscrow {
                escrow: escrow_pubkey,
            },
        )],
        Some(&payer.pubkey()),
        &[&payer],
        banks_client.get_recent_blockhash().await.unwrap(),
    );

    banks_client.process_transaction(release_escrow_tx).await.unwrap();
}

async fn setup() -> (BanksClient, Keypair, Pubkey) {
    let payer = Keypair::new();
    let escrow_pubkey = Keypair::new().pubkey();
    
    // Initialize banks client
    let (mut banks_client, _) = ProgramTest::new(
        "escrow",
        Id::new_unique(),
        processor!(escrow::process_instruction),
    )
    .start()
    .await;

    (banks_client, payer, escrow_pubkey)
}
