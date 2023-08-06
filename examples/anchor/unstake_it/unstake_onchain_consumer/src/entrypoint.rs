use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, program_error::ProgramError,
    pubkey::Pubkey,
};
use unstake_interface::*;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _ix_data: &[u8],
) -> ProgramResult {
    let cpi_slice: &[AccountInfo; UNSTAKE_IX_ACCOUNTS_LEN] = accounts
        .get(..UNSTAKE_IX_ACCOUNTS_LEN)
        .ok_or(ProgramError::NotEnoughAccountKeys)?
        .try_into()
        .unwrap();
    let accounts: UnstakeAccounts = cpi_slice.into();

    if let Err((acc, err)) = unstake_verify_account_privileges(&accounts) {
        solana_program::msg!(
            "Writable/signer privilege escalation for {}: {}",
            acc.key,
            err
        );
        return Err(err);
    }

    unstake_invoke(&accounts, UnstakeIxArgs {})
}
