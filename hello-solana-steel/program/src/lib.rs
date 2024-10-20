
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult,
    pubkey::Pubkey, msg,
};
use steel::{SteelProcessor, ProgramContext};

pub struct HelloSolana;

impl SteelProcessor for HelloSolana {
    fn process_instruction(ctx: ProgramContext) -> ProgramResult {
        let ProgramContext { accounts: _accounts, program_id: _program_id, instruction_data: _data } = ctx;

        // Log the message
        msg!("Hello, Solana from Steel!");

        Ok(())
    }
}

entrypoint!(HelloSolana::process_instruction);
