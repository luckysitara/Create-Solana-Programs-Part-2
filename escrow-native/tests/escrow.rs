#[cfg(test)]
mod test {
    use super::*;
    use solana_program_test::*;
    use solana_sdk::{
        account::Account,
        instruction::{Instruction, AccountMeta},
        signature::{Keypair, Signer},
        transaction::Transaction,
    };

    #[tokio::test]
    async fn test_init_escrow() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "escrow_program",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let maker = Keypair::new();
        let mint_a = Keypair::new();
        let mint_b = Keypair::new();
        let vault = Keypair::new();
        let escrow = Keypair::new();

        // Prepare transaction for initialization
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &EscrowInstruction::InitEscrow { amount: 1000 },
                vec![
                    AccountMeta::new(maker.pubkey(), true),
                    AccountMeta::new(mint_a.pubkey(), false),
                    AccountMeta::new(mint_b.pubkey(), false),
                    AccountMeta::new(vault.pubkey(), false),
                    AccountMeta::new(escrow.pubkey(), true),
                ],
            )],
            Some(&payer.pubkey()),
        );

        // Sign the transaction
        transaction.sign(&[&payer, &maker], recent_blockhash);
        assert!(banks_client.process_transaction(transaction).await.is_ok());
    }

    #[tokio::test]
    async fn test_deposit_escrow() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "escrow_program",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let maker = Keypair::new();
        let mint_a = Keypair::new();
        let vault = Keypair::new();
        let escrow = Keypair::new();

        // Prepare transaction for deposit
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &EscrowInstruction::Deposit { amount: 1000 },
                vec![
                    AccountMeta::new(maker.pubkey(), true),
                    AccountMeta::new(mint_a.pubkey(), false),
                    AccountMeta::new(vault.pubkey(), false),
                    AccountMeta::new(escrow.pubkey(), true),
                ],
            )],
            Some(&payer.pubkey()),
        );

        // Sign and process transaction
        transaction.sign(&[&payer, &maker], recent_blockhash);
        assert!(banks_client.process_transaction(transaction).await.is_ok());
    }

    #[tokio::test]
    async fn test_complete_escrow() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "escrow_program",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let maker = Keypair::new();
        let taker = Keypair::new();
        let mint_a = Keypair::new();
        let mint_b = Keypair::new();
        let vault = Keypair::new();
        let escrow = Keypair::new();

        // Prepare transaction for transfer
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &EscrowInstruction::CompleteEscrow { amount: 1000 },
                vec![
                    AccountMeta::new(taker.pubkey(), true),
                    AccountMeta::new(mint_a.pubkey(), false),
                    AccountMeta::new(mint_b.pubkey(), false),
                    AccountMeta::new(vault.pubkey(), false),
                    AccountMeta::new(escrow.pubkey(), true),
                ],
            )],
            Some(&payer.pubkey()),
        );

        // Sign and process transaction
        transaction.sign(&[&payer, &taker], recent_blockhash);
        assert!(banks_client.process_transaction(transaction).await.is_ok());
    }

    #[tokio::test]
    async fn test_refund_escrow() {
        let program_id = Pubkey::new_unique();
        let (mut banks_client, payer, recent_blockhash) = ProgramTest::new(
            "escrow_program",
            program_id,
            processor!(process_instruction),
        )
        .start()
        .await;

        let maker = Keypair::new();
        let mint_a = Keypair::new();
        let vault = Keypair::new();
        let escrow = Keypair::new();

        // Prepare transaction for refund
        let mut transaction = Transaction::new_with_payer(
            &[Instruction::new_with_borsh(
                program_id,
                &EscrowInstruction::Refund,
                vec![
                    AccountMeta::new(maker.pubkey(), true),
                    AccountMeta::new(mint_a.pubkey(), false),
                    AccountMeta::new(vault.pubkey(), false),
                    AccountMeta::new(escrow.pubkey(), true),
                ],
            )],
            Some(&payer.pubkey()),
        );

        // Sign and process transaction
        transaction.sign(&[&payer, &maker], recent_blockhash);
        assert!(banks_client.process_transaction(transaction).await.is_ok());
    }

    #[tokio::test]
    async fn test_invalid_scenarios() {
    }
}
