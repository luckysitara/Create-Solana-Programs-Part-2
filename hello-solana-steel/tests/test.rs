use solana_program_test::*;
use solana_sdk::{
    signature::{Keypair, Signer},
    transaction::Transaction,
    pubkey::Pubkey,
};

use hello_solana_steel::HelloSolana;

#[tokio::test]
async fn test_integration_hello_solana() {
    let program_id = Pubkey::new_unique();
    let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
        "hello_solana_steel", // Name of the program
        program_id,
        processor!(HelloSolana::process_instruction),
    )
    .start()
    .await;

    // Create and sign a transaction to invoke the program
    let mut transaction = Transaction::new_with_payer(
        &[solana_sdk::instruction::Instruction {
            program_id,
            accounts: vec![],
            data: vec![], // No data passed for this test
        }],
        Some(&payer.pubkey()),
    );

    transaction.sign(&[&payer], recent_blockhash);

    // Process the transaction
    let result = banks_client.process_transaction(transaction).await;

    // Assert the transaction was successful
    assert!(result.is_ok());
}
