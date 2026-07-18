use kont::primitives::{RawAccount, RawPubKey};
use kont::views::token_v1::TokenAccountView;

use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};


entrypoint!(process_instruction);


pub fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {

    let account_info_iter = &mut accounts.iter();

    let vault_info = next_account_info(account_info_iter)?;

    if vault_info.owner.to_bytes() != spl_token::id().to_bytes() {
        return Err(ProgramError::IllegalOwner);
    }

    let raw_key = RawPubKey::new(
        &vault_info.key.to_bytes()
    );

    let raw_owner = RawPubKey::new(
        &vault_info.owner.to_bytes()
    );


    let borrowed_data = vault_info
        .try_borrow_data()?;


    let raw_account = RawAccount::new(
        &raw_key,
        &raw_owner,
        &borrowed_data,
    );


    let data_slice = raw_account
        .borrow_data()
        .map_err(|_| ProgramError::InvalidAccountData)?;


    let view = TokenAccountView::try_from_slice(data_slice)
        .map_err(|_| ProgramError::InvalidAccountData)?;



    msg!(
        "kont:owner={:?}",
        view.owner().as_bytes()
    );


    msg!(
        "kont:amount={}",
        view.amount()
    );


    msg!(
        "kont:frozen={}",
        view.is_frozen()
    );


    Ok(())
}