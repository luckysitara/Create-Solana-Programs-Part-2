use steel::{program, accounts, state};
use anchor_lang::prelude::*;

#[program]
mod escrow {
    use super::*;

    pub fn create_escrow(ctx: Context<CreateEscrow>, amount: u64) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow;

        // Initialize escrow account
        escrow_account.amount = amount;
        escrow_account.is_active = true;
        Ok(())
    }

    pub fn release_escrow(ctx: Context<ReleaseEscrow>) -> Result<()> {
        let escrow_account = &mut ctx.accounts.escrow;

        // Ensure the escrow is active
        require!(escrow_account.is_active, "Escrow is not active");

        // Release funds logic
        escrow_account.is_active = false; // Mark escrow as inactive
        Ok(())
    }
}

#[account]
pub struct EscrowAccount {
    pub amount: u64,
    pub is_active: bool,
}

#[derive(Accounts)]
pub struct CreateEscrow<'info> {
    #[account(init)]
    pub escrow: ProgramAccount<'info, EscrowAccount>,
    pub payer: Signer<'info>,
}

#[derive(Accounts)]
pub struct ReleaseEscrow<'info> {
    #[account(mut)]
    pub escrow: ProgramAccount<'info, EscrowAccount>,
}
