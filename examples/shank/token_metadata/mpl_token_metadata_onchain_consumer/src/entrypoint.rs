use borsh::BorshDeserialize;
use mpl_token_metadata_interface::*;
use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    ix_data: &[u8],
) -> ProgramResult {
    let cpi_slice: &[AccountInfo; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = accounts
        .get(..CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN)
        .ok_or(ProgramError::NotEnoughAccountKeys)?
        .try_into()
        .unwrap();
    let cpi_accounts: CreateMasterEditionAccounts = cpi_slice.into();

    if let Err((acc, err)) = create_master_edition_verify_account_privileges(&cpi_accounts) {
        solana_program::msg!(
            "Writable/signer privilege escalation for {}: {}",
            acc.key,
            err
        );
        return Err(err);
    }

    let mut ix_data_reader = ix_data;
    let create_master_edition_ix_args_without_discm =
        CreateMasterEditionIxArgs::deserialize(&mut ix_data_reader)?;

    create_master_edition_invoke(&cpi_accounts, create_master_edition_ix_args_without_discm)
}
