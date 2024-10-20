use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program::invoke,
    program_error::ProgramError,
    pubkey::Pubkey,
    system_instruction,
    sysvar::{rent::Rent, Sysvar},
};
use spl_token::instruction as token_instruction;
use borsh::{BorshDeserialize, BorshSerialize};

// Define Escrow struct
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Escrow {
    pub is_initialized: bool,
    pub maker_pubkey: Pubkey,
    pub taker_pubkey: Option<Pubkey>,
    pub vault_pubkey: Pubkey,
    pub amount: u64,
}

// Entry point for the program
entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let instruction = instruction_data[0];

    match instruction {
        0 => initialize_escrow(program_id, accounts, instruction_data), // Initialize escrow
        1 => deposit_tokens(program_id, accounts, instruction_data),    // Deposit tokens into vault
        2 => take_tokens(program_id, accounts, instruction_data),       // Withdraw tokens by taker
        3 => refund_tokens(program_id, accounts, instruction_data),     // Refund tokens to maker
        _ => Err(ProgramError::InvalidInstructionData),
    }
}

// Function to initialize the escrow
fn initialize_escrow(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let maker_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let rent_sysvar = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let escrow_account = next_account_info(accounts_iter)?;

    let rent = Rent::from_account_info(rent_sysvar)?;
    if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
        return Err(ProgramError::AccountNotRentExempt);
    }

    if !maker_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let mut escrow_info = Escrow::try_from_slice(&escrow_account.data.borrow())?;

    if escrow_info.is_initialized {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    // Store data
    escrow_info.is_initialized = true;
    escrow_info.maker_pubkey = *maker_account.key;
    escrow_info.vault_pubkey = *vault_account.key;
    escrow_info.amount = u64::from_le_bytes(instruction_data[1..9].try_into().unwrap());
    escrow_info.serialize(&mut &mut escrow_account.data.borrow_mut()[..])?;

    msg!("Escrow initialized");
    Ok(())
}

// Function to deposit tokens
fn deposit_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let maker_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;

    if !maker_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let transfer_ix = token_instruction::transfer(
        token_program.key,
        maker_account.key,
        vault_account.key,
        maker_account.key,
        &[],
        1_000_000, // Example: transfer 1,000,000 tokens
    )?;

    invoke(&transfer_ix, &[
        maker_account.clone(),
        vault_account.clone(),
        token_program.clone(),
    ])?;

    msg!("Deposit successful");
    Ok(())
}

// Function to allow taker to take tokens from escrow
fn take_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let taker_account = next_account_info(accounts_iter)?;
    let maker_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let escrow_account = next_account_info(accounts_iter)?;

    let mut escrow_info = Escrow::try_from_slice(&escrow_account.data.borrow())?;
    if escrow_info.taker_pubkey.is_some() {
        return Err(ProgramError::AccountAlreadyInitialized);
    }

    if !taker_account.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
    }

    let transfer_ix = token_instruction::transfer(
        token_program.key,
        vault_account.key,
        taker_account.key,
        maker_account.key,
        &[],
        escrow_info.amount,
    )?;
    invoke(
        &transfer_ix,
        &[
            vault_account.clone(),
            taker_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Tokens withdrawn by taker");
    Ok(())
}

// Function to refund tokens to the maker
fn refund_tokens(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let maker_account = next_account_info(accounts_iter)?;
    let vault_account = next_account_info(accounts_iter)?;
    let token_program = next_account_info(accounts_iter)?;
    let escrow_account = next_account_info(accounts_iter)?;

    let escrow_info = Escrow::try_from_slice(&escrow_account.data.borrow())?;

    if escrow_info.maker_pubkey != *maker_account.key {
        return Err(ProgramError::InvalidAccountData);
    }

    let transfer_ix = token_instruction::transfer(
        token_program.key,
        vault_account.key,
        maker_account.key,
        maker_account.key,
        &[],
        escrow_info.amount,
    )?;
    invoke(
        &transfer_ix,
        &[
            vault_account.clone(),
            maker_account.clone(),
            token_program.clone(),
        ],
    )?;

    msg!("Tokens refunded to maker");
    Ok(())
}
