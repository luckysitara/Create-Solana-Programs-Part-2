use solana_sdk::{pubkey::Pubkey, transaction::Transaction};
use bankrun::{Bankrun, ProgramTestContext};
use hello_solana_bankrun::HelloSolana;

#[tokio::test]
async fn test_integration_hello_solana_bankrun() {
    let program_id = Pubkey::new_unique();

    let mut context = Bankrun::new(
        "hello_solana_bankrun",   // Program name
        program_id,               // Program ID
        HelloSolana::process_instruction // Register program logic
    ).start().await;

    let payer = context.payer.clone();
    let recent_blockhash = context.last_blockhash;

    // Create and sign a transaction to invoke the program
    let mut transaction = Transaction::new_with_payer(
        &[solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![],
            data: vec![], // No data for this test
        }],
        Some(&payer.pubkey()),
    );
    transaction.sign(&[&payer], recent_blockhash);

    // Process the transaction
    let result = context.process_transaction(transaction).await;

    // Ensure the transaction was successful
    assert!(result.is_ok());
}

