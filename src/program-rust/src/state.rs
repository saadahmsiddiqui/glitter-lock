use solana_program::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};

use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
pub struct GlitterLock {
    pub is_initialized: bool,
    pub locker_public_key: Pubkey,
    pub amount: u64,
    pub lock_time: i64
}

impl Sealed for GlitterLock {}

impl IsInitialized for GlitterLock {
    fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Pack for GlitterLock {
    const LEN: usize = 49;

    fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
        let src = array_ref![src, 0, GlitterLock::LEN];
        let (
            is_initialized,
            locker_public_key,
            amount,
            lock_time,
        ) = array_refs![src, 1, 32, 8, 8];
        
        let is_initialized = match is_initialized {
            [0] => false,
            [1] => true,
            _ => return Err(ProgramError::InvalidAccountData),
        };

        Ok(GlitterLock {
            is_initialized,
            locker_public_key: Pubkey::new_from_array(*locker_public_key),
            amount: u64::from_le_bytes(*amount),
            lock_time: i64::from_le_bytes(*lock_time)
        })
    }


    fn pack_into_slice(&self, dst: &mut [u8]) {
        let dst = array_mut_ref![dst, 0, GlitterLock::LEN];
        let (
            is_initialized_dst,
            locker_public_key_dst,
            amount_dst,
            lock_time_dst,
        ) = mut_array_refs![dst, 1, 32, 8, 8];

        let GlitterLock {
            is_initialized,
            locker_public_key,
            amount,
            lock_time,
        } = self;

        is_initialized_dst[0] = *is_initialized as u8;
        locker_public_key_dst.copy_from_slice(locker_public_key.as_ref());
        *amount_dst = amount.to_le_bytes();
        *lock_time_dst = lock_time.to_le_bytes();
    }
}