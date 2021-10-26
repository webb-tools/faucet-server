use crate::error::TokenError;
use solana_sdk::{
    account_info::AccountInfo, entrypoint::ProgramResult, info, program_error::ProgramError,
    program_utils::next_account_info, pubkey::Pubkey,
};
use std::mem::size_of;

pub fn process(
    _program_id: &Pubkey,
    accounts: &'a [AccountInfo<'a>],
    input: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    Self::process_transfer(account_info_iter, amount);
}

pub fn process_transfer<I: Iterator<Item = &'a AccountInfo<'a>>>(
    account_info_iter: &mut I,
    amount: u64,
) -> ProgramResult {
    let owner_account_info = next_account_info(account_info_iter)?;
    let source_account_info = next_account_info(account_info_iter)?;
    let dest_account_info = next_account_info(account_info_iter)?;

    let mut source_data = source_account_info.data.borrow_mut();
    let mut dest_data = dest_account_info.data.borrow_mut();
    if let (State::Account(mut source_account), State::Account(mut dest_account)) = (
        State::deserialize(&source_data)?,
        State::deserialize(&dest_data)?,
    ) {
        if source_account.token != dest_account.token {
            info!("Error: token mismatch");
            return Err(TokenError::TokenMismatch.into());
        }
        if dest_account.delegate.is_some() {
            info!("Error: destination account is a delegate and cannot accept tokens");
            return Err(ProgramError::InvalidArgument);
        }
        if owner_account_info.key != &source_account.owner {
            info!("Error: source account owner not present");
            return Err(TokenError::NoOwner.into());
        }
        if !owner_account_info.is_signer {
            info!("Error: owner account not a signer");
            return Err(ProgramError::MissingRequiredSignature);
        }
        if source_account.amount < amount {
            return Err(TokenError::InsufficientFunds.into());
        }

        if let Some(ref delegate) = source_account.delegate {
            let source_account_info = next_account_info(account_info_iter)?;
            let mut actual_source_data = source_account_info.data.borrow_mut();
            if let State::Account(mut actual_source_account) =
                State::deserialize(&actual_source_data)?
            {
                if source_account_info.key != &delegate.source {
                    info!("Error: Source account is not a delegate payee");
                    return Err(TokenError::NotDelegate.into());
                }

                if actual_source_account.amount < amount {
                    return Err(TokenError::InsufficientFunds.into());
                }

                actual_source_account.amount -= amount;
                State::Account(actual_source_account).serialize(&mut actual_source_data)?;
            } else {
                info!("Error: payee is an invalid account");
                return Err(ProgramError::InvalidArgument);
            }
        }

        source_account.amount -= amount;
        State::Account(source_account).serialize(&mut source_data)?;

        dest_account.amount += amount;
        State::Account(dest_account).serialize(&mut dest_data)?;
    } else {
        info!("Error: destination and/or source accounts are invalid");
        return Err(ProgramError::InvalidArgument);
    }
    Ok(())
}
