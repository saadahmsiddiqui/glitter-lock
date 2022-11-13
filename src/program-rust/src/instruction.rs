
use solana_program::program_error::ProgramError;
use std::convert::TryInto;

pub enum GlitterLockInstruction {
    Lock {
        amount: u64
    },
    Release
}

impl GlitterLockInstruction {
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(ProgramError::InvalidInstructionData)?;

        Ok(match tag {
            0 => Self::Lock {
                amount: Self::unpack_amount(rest)?,
            },
            1 => Self::Release,
            _ => return Err(ProgramError::InvalidInstructionData),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(ProgramError::InvalidInstructionData)?;
        Ok(amount)
    }
}