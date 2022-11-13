use solana_program::{
    entrypoint::{ProgramResult},
    account_info::AccountInfo, program_error::ProgramError, program_pack::Pack};

pub fn amount_to_lamports(mint: &AccountInfo, amount: f64) -> Result<u64, ProgramError> {
    let mint_account_data = spl_token::state::Mint::unpack_from_slice(&mint.try_borrow_data()?)?;
    let mint_decimals = mint_account_data.decimals;

    let lamports = (amount * f64::powf(10., mint_decimals.into())) as u64;
    Ok(lamports)
}

pub fn transfer_service_fee_lamports(
    from_account: &AccountInfo,
    to_account: &AccountInfo,
    amount_of_lamports: u64,
) -> ProgramResult {
    if **from_account.try_borrow_lamports()? < amount_of_lamports {
        return Err(ProgramError::InsufficientFunds);
    }
    // Debit from_account and credit to_account
    **from_account.try_borrow_mut_lamports()? -= amount_of_lamports;
    **to_account.try_borrow_mut_lamports()? += amount_of_lamports;
    Ok(())
}