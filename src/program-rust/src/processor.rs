use crate::{instruction::GlitterLockInstruction, state::GlitterLock};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    clock::Clock,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    program_pack::Pack,
    pubkey::Pubkey,
    system_instruction::transfer, sysvar::Sysvar,
};
use thiserror::Error;
use crate::utils;

#[derive(Error, Debug, Copy, Clone)]
pub enum LockError {
    #[error("Early Unlock")]
    EarlyUnlock,
    #[error("Not Initialized")]
    NotInitialized,
    #[error("IncorrectClaim")]
    IncorrectClaim,
}
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
                Self::process_unlock(accounts, program_id)
            }
        }
    }

    fn process_lock(accounts: &[AccountInfo], amount: u64, program_id: &Pubkey) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let locker = next_account_info(accounts_iter)?;
        let locker_pda = next_account_info(accounts_iter)?;

        if locker_pda.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !locker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let lamports = locker.lamports();

        if lamports.lt(&amount) {
            return Err(ProgramError::InsufficientFunds);
        }

        let mut lock = GlitterLock::unpack_unchecked(&locker_pda.data.borrow())?;

        if lock.is_initialized {
            return Err(ProgramError::AccountAlreadyInitialized);
        }

        let clock = Clock::get()?;
        lock.amount = amount;
        lock.is_initialized = true;
        lock.locker_public_key = locker_pda.key.clone();
        lock.lock_time = clock.unix_timestamp;
        GlitterLock::pack(lock, &mut locker_pda.data.borrow_mut())?;
        transfer(locker.key, &locker_pda.key, amount);

        Ok(())
    }

    fn process_unlock(accounts: &[AccountInfo], program_id: &Pubkey) -> ProgramResult {
        let accounts_iter = &mut accounts.iter();

        let locker = next_account_info(accounts_iter)?;
        let locker_pda = next_account_info(accounts_iter)?;

        if locker_pda.data_len() == 0 {
            return Err(ProgramError::Custom(LockError::NotInitialized as u32));
        }

        if locker_pda.owner != program_id {
            return Err(ProgramError::IncorrectProgramId);
        }

        if !locker.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

        let mut lock = GlitterLock::unpack_unchecked(&locker_pda.data.borrow())?;
        if !lock.is_initialized {
            return Err(ProgramError::Custom(LockError::NotInitialized as u32));
        }

        if lock.locker_public_key.ne(&locker.key) {
            return Err(ProgramError::Custom(LockError::IncorrectClaim as u32));
        }

        let one_min = 60;
        let current_time = Clock::get()?.unix_timestamp;
        if lock.lock_time + one_min > current_time {
            return Err(ProgramError::Custom(LockError::EarlyUnlock as u32));
        }

        lock.is_initialized = false;
        
        utils::transfer_service_fee_lamports(&locker_pda, &locker, lock.amount)?;
        GlitterLock::pack(lock, &mut locker_pda.data.borrow_mut())?;

        Ok(())
    }
}
