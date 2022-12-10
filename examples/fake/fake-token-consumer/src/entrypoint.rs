use fake_token_interface::{
    transfer_invoke_signed, TransferAccounts, TransferArgs, TransferIxData,
};
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
    _instruction_data: &[u8],
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();

    let src = next_account_info(account_info_iter)?;
    let dest = next_account_info(account_info_iter)?;

    transfer_invoke_signed(
        &TransferAccounts { src, dest },
        &TransferIxData {
            transfer_args: &TransferArgs { amount: 1_000u64 },
        },
        &[&[&[0u8]]],
    )?;

    Ok(())
}
