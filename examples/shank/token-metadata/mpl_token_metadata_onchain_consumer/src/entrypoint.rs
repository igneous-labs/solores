use borsh::BorshDeserialize;
use mpl_token_metadata_interface::*;
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let edition = next_account_info(account_info_iter)?;
    let mint = next_account_info(account_info_iter)?;
    let update_authority = next_account_info(account_info_iter)?;
    let mint_authority = next_account_info(account_info_iter)?;
    let payer = next_account_info(account_info_iter)?;
    let metadata = next_account_info(account_info_iter)?;
    let token_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;
    let rent = next_account_info(account_info_iter)?;

    let mut ix_data_reader = ix_data.as_ref();
    let create_master_edition_ix_args_without_discm =
        CreateMasterEditionIxArgs::deserialize(&mut ix_data_reader)?;
    create_master_edition_invoke(
        &CreateMasterEditionAccounts {
            edition,
            mint,
            update_authority,
            mint_authority,
            payer,
            metadata,
            token_program,
            system_program,
            rent,
        },
        create_master_edition_ix_args_without_discm,
    )
}
