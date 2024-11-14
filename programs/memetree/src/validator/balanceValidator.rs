use anchor_lang::prelude::Signer;
use anchor_lang::prelude::*;

use crate::{constants::TREASURY_FEE, errors::FundError};

pub fn balance_validator(wallet: AccountInfo) -> Result<()> {
    msg!("Current wallet balance in lamports: {}", wallet.get_lamports());
    if wallet.get_lamports() < TREASURY_FEE as u64 {
        return Err(FundError::InsufficiencyError.into());
    }
    Ok(())
}

