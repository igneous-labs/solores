use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    pubkey::Pubkey,
};
use unstake_interface::*;

entrypoint!(process_instruction);
fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    _ix_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let payer = next_account_info(account_info_iter)?;
    let unstaker = next_account_info(account_info_iter)?;
    let stake_account = next_account_info(account_info_iter)?;
    let destination = next_account_info(account_info_iter)?;
    let pool_account = next_account_info(account_info_iter)?;
    let pool_sol_reserves = next_account_info(account_info_iter)?;
    let fee_account = next_account_info(account_info_iter)?;
    let stake_account_record_account = next_account_info(account_info_iter)?;
    let protocol_fee_account = next_account_info(account_info_iter)?;
    let protocol_fee_destination = next_account_info(account_info_iter)?;
    let clock = next_account_info(account_info_iter)?;
    let stake_program = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    unstake_invoke(
        &UnstakeAccounts {
            payer,
            unstaker,
            stake_account,
            destination,
            pool_account,
            pool_sol_reserves,
            fee_account,
            stake_account_record_account,
            protocol_fee_account,
            protocol_fee_destination,
            clock,
            stake_program,
            system_program,
        },
        UnstakeIxArgs {},
    )
}
