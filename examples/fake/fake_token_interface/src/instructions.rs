use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};

use crate::*;

#[derive(Copy, Clone, Debug)]
pub struct TransferAccounts<'me, 'a1: 'me, 'a2: 'me> {
    pub src: &'me AccountInfo<'a1>,
    pub dest: &'me AccountInfo<'a2>,
}

#[derive(Copy, Clone, Debug)]
pub struct TransferKeys<'me> {
    pub src: &'me Pubkey,
    pub dest: &'me Pubkey,
}

impl<'me> From<&TransferAccounts<'me, '_, '_>> for TransferKeys<'me> {
    fn from(transfer_accounts: &TransferAccounts<'me, '_, '_>) -> Self {
        Self {
            src: transfer_accounts.src.key,
            dest: transfer_accounts.dest.key,
        }
    }
}

impl From<&TransferKeys<'_>> for [AccountMeta; 2] {
    fn from(transfer_keys: &TransferKeys<'_>) -> Self {
        [
            AccountMeta::new(*transfer_keys.src, true),
            AccountMeta::new(*transfer_keys.dest, false),
        ]
    }
}

impl<'a> From<&TransferAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; 2] {
    fn from(transfer_accounts: &TransferAccounts<'_, 'a, 'a>) -> Self {
        [
            transfer_accounts.src.clone(),
            transfer_accounts.dest.clone(),
        ]
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug)]
pub struct TransferArgs {
    pub amount: u64,
}

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
