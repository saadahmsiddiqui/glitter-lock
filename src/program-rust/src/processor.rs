use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program::{invoke},
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack},
    pubkey::Pubkey,
    clock::Clock,
    sysvar::{rent::Rent, Sysvar},
};

use crate::{error::LockError, instruction::GlitterLockInstruction, state::GlitterLock};

pub struct Processor;
impl Processor {
    pub fn process(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        let instruction = GlitterLockInstruction::unpack(instruction_data)?;

        match instruction {
            GlitterLockInstruction::Lock { amount } => {
                msg!("Instruction: Lock");
                Self::process_lock(accounts, amount, program_id)
            }
            GlitterLockInstruction::Release => {
                msg!("Instruction: Release");
                Ok(())
            }
        }
    }

    fn process_lock(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let temp_token_account = next_account_info(account_info_iter)?;

        let token_to_receive_account = next_account_info(account_info_iter)?;
        if *token_to_receive_account.owner != spl_token::id() {
            return Err(ProgramError::IncorrectProgramId);
        }

        let lock_account = next_account_info(account_info_iter)?;
        let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

        if !rent.is_exempt(lock_account.lamports(), lock_account.data_len()) {
            return Err(LockError::NotRentExempt.into());
        }

        let mut lock_info = GlitterLock::unpack_unchecked(&lock_account.try_borrow_data()?)?;
        if lock_info.is_initialized() {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;
        lock_info.is_initialized = true;
        lock_info.initializer_public_key = *initializer.key;
        lock_info.temp_account_key = *temp_token_account.key;
        lock_info.amount = amount;
        lock_info.lock_time = current_timestamp;

        GlitterLock::pack(lock_info, &mut lock_account.try_borrow_mut_data()?)?;
        let (pda, _nonce) = Pubkey::find_program_address(&[b"lock"], program_id);

        let token_program = next_account_info(account_info_iter)?;
        let owner_change_ix = spl_token::instruction::set_authority(
            token_program.key,
            temp_token_account.key,
            Some(&pda),
            spl_token::instruction::AuthorityType::AccountOwner,
            initializer.key,
            &[&initializer.key],
        )?;

        msg!("Calling the token program to transfer token account ownership...");
        invoke(
            &owner_change_ix,
            &[
                temp_token_account.clone(),
                initializer.clone(),
                token_program.clone(),
            ],
        )?;

        Ok(())
    }
}