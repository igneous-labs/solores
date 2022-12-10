use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
};

use crate::*;

pub struct TransferIxData<'me> {
    pub transfer_args: &'me TransferArgs,
}

impl BorshSerialize for TransferIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        // discriminant
        writer.write(&[0u8])?;
        self.transfer_args.serialize(writer)?;
        Ok(())
    }
}

pub fn transfer_ix<'a, T: Into<TransferKeys<'a>>>(
    accounts: T,
    data: &TransferIxData,
) -> std::io::Result<Instruction> {
    let keys: TransferKeys = accounts.into();
    let metas: [AccountMeta; 2] = (&keys).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}

pub fn transfer_invoke<'a>(
    accounts: &TransferAccounts<'_, 'a, 'a>,
    data: &TransferIxData,
) -> ProgramResult {
    let ix = transfer_ix(accounts, data)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke(&ix, &account_info)
}

pub fn transfer_invoke_signed<'a>(
    accounts: &TransferAccounts<'_, 'a, 'a>,
    data: &TransferIxData,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = transfer_ix(accounts, data)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
