
use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum LockError {
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
    /// Not Rent Exempt
    #[error("Not Rent Exempt")]
    NotRentExempt,
    /// Amount Overflow
    #[error("Amount Overflow")]
    AmountOverflow,
}

impl From<LockError> for ProgramError {
    fn from(e: LockError) -> Self {
        ProgramError::Custom(e as u32)
    }
}