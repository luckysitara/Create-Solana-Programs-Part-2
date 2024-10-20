use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey, msg,
};
use steel::{SteelProcessor, ProgramContext};

// The main program structure implementing SteelProcessor
pub struct HelloSolana;

impl SteelProcessor for HelloSolana {
    fn process_instruction(ctx: ProgramContext) -> ProgramResult {
        let ProgramContext { accounts: _accounts, program_id: _program_id, instruction_data: _data } = ctx;

        // Log a "Hello, Solana!" message
        msg!("Hello, Solana from Steel!");

        Ok(())
    }
}

// Define the entry point to the program
entrypoint!(HelloSolana::process_instruction);

#[cfg(test)]
mod test {
    use super::*;
    use bankrun::{Bankrun, ProgramTestContext};
    use solana_sdk::{transaction::Transaction, pubkey::Pubkey};

    // Bankrun test for Hello Solana
    #[tokio::test]
    async fn test_hello_solana_with_bankrun() {
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
        context.process_transaction(transaction).await.unwrap();
    }
}

