use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
pub const INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitProtocolFeeAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub protocol_fee_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitProtocolFeeKeys {
    pub payer: Pubkey,
    pub protocol_fee_account: Pubkey,
    pub system_program: Pubkey,
}
impl From<&InitProtocolFeeAccounts<'_, '_>> for InitProtocolFeeKeys {
    fn from(accounts: &InitProtocolFeeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            protocol_fee_account: *accounts.protocol_fee_account.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitProtocolFeeKeys> for [AccountMeta; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitProtocolFeeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new(keys.protocol_fee_account, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for InitProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            protocol_fee_account: pubkeys[1],
            system_program: pubkeys[2],
        }
    }
}
impl<'info> From<&InitProtocolFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.protocol_fee_account.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
    for InitProtocolFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            protocol_fee_account: &arr[1],
            system_program: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitProtocolFeeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct InitProtocolFeeIxData<'me>(pub &'me InitProtocolFeeIxArgs);
pub const INIT_PROTOCOL_FEE_IX_DISCM: [u8; 8] = [225, 155, 167, 170, 29, 145, 165, 90];
impl<'me> From<&'me InitProtocolFeeIxArgs> for InitProtocolFeeIxData<'me> {
    fn from(args: &'me InitProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitProtocolFeeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INIT_PROTOCOL_FEE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn init_protocol_fee_ix<K: Into<InitProtocolFeeKeys>, A: Into<InitProtocolFeeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitProtocolFeeKeys = accounts.into();
    let metas: [AccountMeta; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitProtocolFeeIxArgs = args.into();
    let data: InitProtocolFeeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn init_protocol_fee_invoke<'info, A: Into<InitProtocolFeeIxArgs>>(
    accounts: &InitProtocolFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = init_protocol_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn init_protocol_fee_invoke_signed<'info, A: Into<InitProtocolFeeIxArgs>>(
    accounts: &InitProtocolFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = init_protocol_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; INIT_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeAccounts<'me, 'info> {
    pub authority: &'me AccountInfo<'info>,
    pub protocol_fee_account: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeKeys {
    pub authority: Pubkey,
    pub protocol_fee_account: Pubkey,
}
impl From<&SetProtocolFeeAccounts<'_, '_>> for SetProtocolFeeKeys {
    fn from(accounts: &SetProtocolFeeAccounts) -> Self {
        Self {
            authority: *accounts.authority.key,
            protocol_fee_account: *accounts.protocol_fee_account.key,
        }
    }
}
impl From<&SetProtocolFeeKeys> for [AccountMeta; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetProtocolFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.protocol_fee_account, false),
        ]
    }
}
impl From<[Pubkey; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]> for SetProtocolFeeKeys {
    fn from(pubkeys: [Pubkey; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: pubkeys[0],
            protocol_fee_account: pubkeys[1],
        }
    }
}
impl<'info> From<&SetProtocolFeeAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetProtocolFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.authority.clone(),
            accounts.protocol_fee_account.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]>
    for SetProtocolFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            authority: &arr[0],
            protocol_fee_account: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetProtocolFeeIxArgs {
    pub protocol_fee: ProtocolFee,
}
#[derive(Copy, Clone, Debug)]
pub struct SetProtocolFeeIxData<'me>(pub &'me SetProtocolFeeIxArgs);
pub const SET_PROTOCOL_FEE_IX_DISCM: [u8; 8] = [173, 239, 83, 242, 136, 43, 144, 217];
impl<'me> From<&'me SetProtocolFeeIxArgs> for SetProtocolFeeIxData<'me> {
    fn from(args: &'me SetProtocolFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetProtocolFeeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SET_PROTOCOL_FEE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn set_protocol_fee_ix<K: Into<SetProtocolFeeKeys>, A: Into<SetProtocolFeeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetProtocolFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetProtocolFeeIxArgs = args.into();
    let data: SetProtocolFeeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_protocol_fee_invoke<'info, A: Into<SetProtocolFeeIxArgs>>(
    accounts: &SetProtocolFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_protocol_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_protocol_fee_invoke_signed<'info, A: Into<SetProtocolFeeIxArgs>>(
    accounts: &SetProtocolFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_protocol_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_PROTOCOL_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CREATE_POOL_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub fee_authority: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolKeys {
    pub payer: Pubkey,
    pub fee_authority: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub fee_account: Pubkey,
    pub lp_mint: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<&CreatePoolAccounts<'_, '_>> for CreatePoolKeys {
    fn from(accounts: &CreatePoolAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            fee_authority: *accounts.fee_authority.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            fee_account: *accounts.fee_account.key,
            lp_mint: *accounts.lp_mint.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&CreatePoolKeys> for [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreatePoolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.fee_authority, true),
            AccountMeta::new(keys.pool_account, true),
            AccountMeta::new_readonly(keys.pool_sol_reserves, false),
            AccountMeta::new(keys.fee_account, false),
            AccountMeta::new(keys.lp_mint, true),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl From<[Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]> for CreatePoolKeys {
    fn from(pubkeys: [Pubkey; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            fee_authority: pubkeys[1],
            pool_account: pubkeys[2],
            pool_sol_reserves: pubkeys[3],
            fee_account: pubkeys[4],
            lp_mint: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<&CreatePoolAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreatePoolAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.fee_authority.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.fee_account.clone(),
            accounts.lp_mint.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]>
    for CreatePoolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            fee_authority: &arr[1],
            pool_account: &arr[2],
            pool_sol_reserves: &arr[3],
            fee_account: &arr[4],
            lp_mint: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreatePoolIxArgs {
    pub fee: Fee,
}
#[derive(Copy, Clone, Debug)]
pub struct CreatePoolIxData<'me>(pub &'me CreatePoolIxArgs);
pub const CREATE_POOL_IX_DISCM: [u8; 8] = [233, 146, 209, 142, 207, 104, 64, 188];
impl<'me> From<&'me CreatePoolIxArgs> for CreatePoolIxData<'me> {
    fn from(args: &'me CreatePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreatePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CREATE_POOL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn create_pool_ix<K: Into<CreatePoolKeys>, A: Into<CreatePoolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreatePoolKeys = accounts.into();
    let metas: [AccountMeta; CREATE_POOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreatePoolIxArgs = args.into();
    let data: CreatePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_pool_invoke<'info, A: Into<CreatePoolIxArgs>>(
    accounts: &CreatePoolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_pool_invoke_signed<'info, A: Into<CreatePoolIxArgs>>(
    accounts: &CreatePoolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const ADD_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityAccounts<'me, 'info> {
    pub from: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub mint_lp_tokens_to: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityKeys {
    pub from: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub lp_mint: Pubkey,
    pub mint_lp_tokens_to: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<&AddLiquidityAccounts<'_, '_>> for AddLiquidityKeys {
    fn from(accounts: &AddLiquidityAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            lp_mint: *accounts.lp_mint.key,
            mint_lp_tokens_to: *accounts.mint_lp_tokens_to.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&AddLiquidityKeys> for [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddLiquidityKeys) -> Self {
        [
            AccountMeta::new(keys.from, true),
            AccountMeta::new(keys.pool_account, false),
            AccountMeta::new(keys.pool_sol_reserves, false),
            AccountMeta::new(keys.lp_mint, false),
            AccountMeta::new(keys.mint_lp_tokens_to, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]> for AddLiquidityKeys {
    fn from(pubkeys: [Pubkey; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            pool_account: pubkeys[1],
            pool_sol_reserves: pubkeys[2],
            lp_mint: pubkeys[3],
            mint_lp_tokens_to: pubkeys[4],
            token_program: pubkeys[5],
            system_program: pubkeys[6],
        }
    }
}
impl<'info> From<&AddLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.from.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.lp_mint.clone(),
            accounts.mint_lp_tokens_to.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for AddLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            pool_account: &arr[1],
            pool_sol_reserves: &arr[2],
            lp_mint: &arr[3],
            mint_lp_tokens_to: &arr[4],
            token_program: &arr[5],
            system_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct AddLiquidityIxArgs {
    pub amount: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct AddLiquidityIxData<'me>(pub &'me AddLiquidityIxArgs);
pub const ADD_LIQUIDITY_IX_DISCM: [u8; 8] = [181, 157, 89, 67, 143, 182, 52, 72];
impl<'me> From<&'me AddLiquidityIxArgs> for AddLiquidityIxData<'me> {
    fn from(args: &'me AddLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddLiquidityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADD_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn add_liquidity_ix<K: Into<AddLiquidityKeys>, A: Into<AddLiquidityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddLiquidityKeys = accounts.into();
    let metas: [AccountMeta; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddLiquidityIxArgs = args.into();
    let data: AddLiquidityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_liquidity_invoke<'info, A: Into<AddLiquidityIxArgs>>(
    accounts: &AddLiquidityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = add_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_liquidity_invoke_signed<'info, A: Into<AddLiquidityIxArgs>>(
    accounts: &AddLiquidityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; ADD_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityAccounts<'me, 'info> {
    pub burn_lp_tokens_from_authority: &'me AccountInfo<'info>,
    pub to: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub burn_lp_tokens_from: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityKeys {
    pub burn_lp_tokens_from_authority: Pubkey,
    pub to: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub lp_mint: Pubkey,
    pub burn_lp_tokens_from: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<&RemoveLiquidityAccounts<'_, '_>> for RemoveLiquidityKeys {
    fn from(accounts: &RemoveLiquidityAccounts) -> Self {
        Self {
            burn_lp_tokens_from_authority: *accounts.burn_lp_tokens_from_authority.key,
            to: *accounts.to.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            lp_mint: *accounts.lp_mint.key,
            burn_lp_tokens_from: *accounts.burn_lp_tokens_from.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&RemoveLiquidityKeys> for [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemoveLiquidityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.burn_lp_tokens_from_authority, true),
            AccountMeta::new(keys.to, false),
            AccountMeta::new(keys.pool_account, false),
            AccountMeta::new(keys.pool_sol_reserves, false),
            AccountMeta::new(keys.lp_mint, false),
            AccountMeta::new(keys.burn_lp_tokens_from, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]> for RemoveLiquidityKeys {
    fn from(pubkeys: [Pubkey; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            burn_lp_tokens_from_authority: pubkeys[0],
            to: pubkeys[1],
            pool_account: pubkeys[2],
            pool_sol_reserves: pubkeys[3],
            lp_mint: pubkeys[4],
            burn_lp_tokens_from: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
        }
    }
}
impl<'info> From<&RemoveLiquidityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveLiquidityAccounts<'_, 'info>) -> Self {
        [
            accounts.burn_lp_tokens_from_authority.clone(),
            accounts.to.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.lp_mint.clone(),
            accounts.burn_lp_tokens_from.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]>
    for RemoveLiquidityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            burn_lp_tokens_from_authority: &arr[0],
            to: &arr[1],
            pool_account: &arr[2],
            pool_sol_reserves: &arr[3],
            lp_mint: &arr[4],
            burn_lp_tokens_from: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveLiquidityIxArgs {
    pub amount_lp: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveLiquidityIxData<'me>(pub &'me RemoveLiquidityIxArgs);
pub const REMOVE_LIQUIDITY_IX_DISCM: [u8; 8] = [80, 85, 209, 72, 24, 206, 177, 108];
impl<'me> From<&'me RemoveLiquidityIxArgs> for RemoveLiquidityIxData<'me> {
    fn from(args: &'me RemoveLiquidityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveLiquidityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_LIQUIDITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn remove_liquidity_ix<K: Into<RemoveLiquidityKeys>, A: Into<RemoveLiquidityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveLiquidityKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveLiquidityIxArgs = args.into();
    let data: RemoveLiquidityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_liquidity_invoke<'info, A: Into<RemoveLiquidityIxArgs>>(
    accounts: &RemoveLiquidityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_liquidity_invoke_signed<'info, A: Into<RemoveLiquidityIxArgs>>(
    accounts: &RemoveLiquidityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_liquidity_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_LIQUIDITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SET_FEE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAccounts<'me, 'info> {
    pub fee_authority: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFeeKeys {
    pub fee_authority: Pubkey,
    pub pool_account: Pubkey,
    pub fee_account: Pubkey,
    pub system_program: Pubkey,
    pub rent: Pubkey,
}
impl From<&SetFeeAccounts<'_, '_>> for SetFeeKeys {
    fn from(accounts: &SetFeeAccounts) -> Self {
        Self {
            fee_authority: *accounts.fee_authority.key,
            pool_account: *accounts.pool_account.key,
            fee_account: *accounts.fee_account.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&SetFeeKeys> for [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.fee_authority, true),
            AccountMeta::new_readonly(keys.pool_account, false),
            AccountMeta::new(keys.fee_account, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl From<[Pubkey; SET_FEE_IX_ACCOUNTS_LEN]> for SetFeeKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_authority: pubkeys[0],
            pool_account: pubkeys[1],
            fee_account: pubkeys[2],
            system_program: pubkeys[3],
            rent: pubkeys[4],
        }
    }
}
impl<'info> From<&SetFeeAccounts<'_, 'info>> for [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: &SetFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_authority.clone(),
            accounts.pool_account.clone(),
            accounts.fee_account.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]>
    for SetFeeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_authority: &arr[0],
            pool_account: &arr[1],
            fee_account: &arr[2],
            system_program: &arr[3],
            rent: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetFeeIxArgs {
    pub fee: Fee,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFeeIxData<'me>(pub &'me SetFeeIxArgs);
pub const SET_FEE_IX_DISCM: [u8; 8] = [18, 154, 24, 18, 237, 214, 19, 80];
impl<'me> From<&'me SetFeeIxArgs> for SetFeeIxData<'me> {
    fn from(args: &'me SetFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetFeeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn set_fee_ix<K: Into<SetFeeKeys>, A: Into<SetFeeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetFeeKeys = accounts.into();
    let metas: [AccountMeta; SET_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetFeeIxArgs = args.into();
    let data: SetFeeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_invoke<'info, A: Into<SetFeeIxArgs>>(
    accounts: &SetFeeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_fee_invoke_signed<'info, A: Into<SetFeeIxArgs>>(
    accounts: &SetFeeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_FEE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAuthorityAccounts<'me, 'info> {
    pub fee_authority: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub new_fee_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAuthorityKeys {
    pub fee_authority: Pubkey,
    pub pool_account: Pubkey,
    pub new_fee_authority: Pubkey,
}
impl From<&SetFeeAuthorityAccounts<'_, '_>> for SetFeeAuthorityKeys {
    fn from(accounts: &SetFeeAuthorityAccounts) -> Self {
        Self {
            fee_authority: *accounts.fee_authority.key,
            pool_account: *accounts.pool_account.key,
            new_fee_authority: *accounts.new_fee_authority.key,
        }
    }
}
impl From<&SetFeeAuthorityKeys> for [AccountMeta; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &SetFeeAuthorityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.fee_authority, true),
            AccountMeta::new(keys.pool_account, false),
            AccountMeta::new_readonly(keys.new_fee_authority, false),
        ]
    }
}
impl From<[Pubkey; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]> for SetFeeAuthorityKeys {
    fn from(pubkeys: [Pubkey; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_authority: pubkeys[0],
            pool_account: pubkeys[1],
            new_fee_authority: pubkeys[2],
        }
    }
}
impl<'info> From<&SetFeeAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetFeeAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.fee_authority.clone(),
            accounts.pool_account.clone(),
            accounts.new_fee_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for SetFeeAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            fee_authority: &arr[0],
            pool_account: &arr[1],
            new_fee_authority: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetFeeAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SetFeeAuthorityIxData<'me>(pub &'me SetFeeAuthorityIxArgs);
pub const SET_FEE_AUTHORITY_IX_DISCM: [u8; 8] = [31, 1, 50, 87, 237, 101, 97, 132];
impl<'me> From<&'me SetFeeAuthorityIxArgs> for SetFeeAuthorityIxData<'me> {
    fn from(args: &'me SetFeeAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetFeeAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SET_FEE_AUTHORITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn set_fee_authority_ix<K: Into<SetFeeAuthorityKeys>, A: Into<SetFeeAuthorityIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetFeeAuthorityKeys = accounts.into();
    let metas: [AccountMeta; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetFeeAuthorityIxArgs = args.into();
    let data: SetFeeAuthorityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_fee_authority_invoke<'info, A: Into<SetFeeAuthorityIxArgs>>(
    accounts: &SetFeeAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_fee_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_fee_authority_invoke_signed<'info, A: Into<SetFeeAuthorityIxArgs>>(
    accounts: &SetFeeAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_fee_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_FEE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct DeactivateStakeAccountAccounts<'me, 'info> {
    pub stake_account: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeactivateStakeAccountKeys {
    pub stake_account: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub clock: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&DeactivateStakeAccountAccounts<'_, '_>> for DeactivateStakeAccountKeys {
    fn from(accounts: &DeactivateStakeAccountAccounts) -> Self {
        Self {
            stake_account: *accounts.stake_account.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&DeactivateStakeAccountKeys> for [AccountMeta; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &DeactivateStakeAccountKeys) -> Self {
        [
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new_readonly(keys.pool_account, false),
            AccountMeta::new_readonly(keys.pool_sol_reserves, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]> for DeactivateStakeAccountKeys {
    fn from(pubkeys: [Pubkey; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_account: pubkeys[0],
            pool_account: pubkeys[1],
            pool_sol_reserves: pubkeys[2],
            clock: pubkeys[3],
            stake_program: pubkeys[4],
        }
    }
}
impl<'info> From<&DeactivateStakeAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeactivateStakeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_account.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for DeactivateStakeAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_account: &arr[0],
            pool_account: &arr[1],
            pool_sol_reserves: &arr[2],
            clock: &arr[3],
            stake_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeactivateStakeAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct DeactivateStakeAccountIxData<'me>(pub &'me DeactivateStakeAccountIxArgs);
pub const DEACTIVATE_STAKE_ACCOUNT_IX_DISCM: [u8; 8] = [217, 64, 76, 16, 216, 77, 123, 226];
impl<'me> From<&'me DeactivateStakeAccountIxArgs> for DeactivateStakeAccountIxData<'me> {
    fn from(args: &'me DeactivateStakeAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeactivateStakeAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEACTIVATE_STAKE_ACCOUNT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn deactivate_stake_account_ix<
    K: Into<DeactivateStakeAccountKeys>,
    A: Into<DeactivateStakeAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeactivateStakeAccountKeys = accounts.into();
    let metas: [AccountMeta; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeactivateStakeAccountIxArgs = args.into();
    let data: DeactivateStakeAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deactivate_stake_account_invoke<'info, A: Into<DeactivateStakeAccountIxArgs>>(
    accounts: &DeactivateStakeAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deactivate_stake_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deactivate_stake_account_invoke_signed<'info, A: Into<DeactivateStakeAccountIxArgs>>(
    accounts: &DeactivateStakeAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deactivate_stake_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEACTIVATE_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct ReclaimStakeAccountAccounts<'me, 'info> {
    pub stake_account: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub stake_account_record_account: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_history: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct ReclaimStakeAccountKeys {
    pub stake_account: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub stake_account_record_account: Pubkey,
    pub clock: Pubkey,
    pub stake_history: Pubkey,
    pub stake_program: Pubkey,
}
impl From<&ReclaimStakeAccountAccounts<'_, '_>> for ReclaimStakeAccountKeys {
    fn from(accounts: &ReclaimStakeAccountAccounts) -> Self {
        Self {
            stake_account: *accounts.stake_account.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            stake_account_record_account: *accounts.stake_account_record_account.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_program: *accounts.stake_program.key,
        }
    }
}
impl From<&ReclaimStakeAccountKeys> for [AccountMeta; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &ReclaimStakeAccountKeys) -> Self {
        [
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new(keys.pool_account, false),
            AccountMeta::new(keys.pool_sol_reserves, false),
            AccountMeta::new(keys.stake_account_record_account, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_history, false),
            AccountMeta::new_readonly(keys.stake_program, false),
        ]
    }
}
impl From<[Pubkey; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]> for ReclaimStakeAccountKeys {
    fn from(pubkeys: [Pubkey; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_account: pubkeys[0],
            pool_account: pubkeys[1],
            pool_sol_reserves: pubkeys[2],
            stake_account_record_account: pubkeys[3],
            clock: pubkeys[4],
            stake_history: pubkeys[5],
            stake_program: pubkeys[6],
        }
    }
}
impl<'info> From<&ReclaimStakeAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ReclaimStakeAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.stake_account.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.stake_account_record_account.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for ReclaimStakeAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake_account: &arr[0],
            pool_account: &arr[1],
            pool_sol_reserves: &arr[2],
            stake_account_record_account: &arr[3],
            clock: &arr[4],
            stake_history: &arr[5],
            stake_program: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ReclaimStakeAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ReclaimStakeAccountIxData<'me>(pub &'me ReclaimStakeAccountIxArgs);
pub const RECLAIM_STAKE_ACCOUNT_IX_DISCM: [u8; 8] = [47, 127, 90, 221, 10, 160, 183, 117];
impl<'me> From<&'me ReclaimStakeAccountIxArgs> for ReclaimStakeAccountIxData<'me> {
    fn from(args: &'me ReclaimStakeAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ReclaimStakeAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&RECLAIM_STAKE_ACCOUNT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn reclaim_stake_account_ix<
    K: Into<ReclaimStakeAccountKeys>,
    A: Into<ReclaimStakeAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ReclaimStakeAccountKeys = accounts.into();
    let metas: [AccountMeta; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ReclaimStakeAccountIxArgs = args.into();
    let data: ReclaimStakeAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reclaim_stake_account_invoke<'info, A: Into<ReclaimStakeAccountIxArgs>>(
    accounts: &ReclaimStakeAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = reclaim_stake_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn reclaim_stake_account_invoke_signed<'info, A: Into<ReclaimStakeAccountIxArgs>>(
    accounts: &ReclaimStakeAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = reclaim_stake_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; RECLAIM_STAKE_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UNSTAKE_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub unstaker: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub stake_account_record_account: &'me AccountInfo<'info>,
    pub protocol_fee_account: &'me AccountInfo<'info>,
    pub protocol_fee_destination: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UnstakeKeys {
    pub payer: Pubkey,
    pub unstaker: Pubkey,
    pub stake_account: Pubkey,
    pub destination: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub fee_account: Pubkey,
    pub stake_account_record_account: Pubkey,
    pub protocol_fee_account: Pubkey,
    pub protocol_fee_destination: Pubkey,
    pub clock: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
}
impl From<&UnstakeAccounts<'_, '_>> for UnstakeKeys {
    fn from(accounts: &UnstakeAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            unstaker: *accounts.unstaker.key,
            stake_account: *accounts.stake_account.key,
            destination: *accounts.destination.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            fee_account: *accounts.fee_account.key,
            stake_account_record_account: *accounts.stake_account_record_account.key,
            protocol_fee_account: *accounts.protocol_fee_account.key,
            protocol_fee_destination: *accounts.protocol_fee_destination.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&UnstakeKeys> for [AccountMeta; UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UnstakeKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.unstaker, true),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new(keys.destination, false),
            AccountMeta::new(keys.pool_account, false),
            AccountMeta::new(keys.pool_sol_reserves, false),
            AccountMeta::new_readonly(keys.fee_account, false),
            AccountMeta::new(keys.stake_account_record_account, false),
            AccountMeta::new_readonly(keys.protocol_fee_account, false),
            AccountMeta::new(keys.protocol_fee_destination, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl From<[Pubkey; UNSTAKE_IX_ACCOUNTS_LEN]> for UnstakeKeys {
    fn from(pubkeys: [Pubkey; UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            unstaker: pubkeys[1],
            stake_account: pubkeys[2],
            destination: pubkeys[3],
            pool_account: pubkeys[4],
            pool_sol_reserves: pubkeys[5],
            fee_account: pubkeys[6],
            stake_account_record_account: pubkeys[7],
            protocol_fee_account: pubkeys[8],
            protocol_fee_destination: pubkeys[9],
            clock: pubkeys[10],
            stake_program: pubkeys[11],
            system_program: pubkeys[12],
        }
    }
}
impl<'info> From<&UnstakeAccounts<'_, 'info>> for [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.unstaker.clone(),
            accounts.stake_account.clone(),
            accounts.destination.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.fee_account.clone(),
            accounts.stake_account_record_account.clone(),
            accounts.protocol_fee_account.clone(),
            accounts.protocol_fee_destination.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN]>
    for UnstakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            unstaker: &arr[1],
            stake_account: &arr[2],
            destination: &arr[3],
            pool_account: &arr[4],
            pool_sol_reserves: &arr[5],
            fee_account: &arr[6],
            stake_account_record_account: &arr[7],
            protocol_fee_account: &arr[8],
            protocol_fee_destination: &arr[9],
            clock: &arr[10],
            stake_program: &arr[11],
            system_program: &arr[12],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnstakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UnstakeIxData<'me>(pub &'me UnstakeIxArgs);
pub const UNSTAKE_IX_DISCM: [u8; 8] = [90, 95, 107, 42, 205, 124, 50, 225];
impl<'me> From<&'me UnstakeIxArgs> for UnstakeIxData<'me> {
    fn from(args: &'me UnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UnstakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn unstake_ix<K: Into<UnstakeKeys>, A: Into<UnstakeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UnstakeKeys = accounts.into();
    let metas: [AccountMeta; UNSTAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UnstakeIxArgs = args.into();
    let data: UnstakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unstake_invoke<'info, A: Into<UnstakeIxArgs>>(
    accounts: &UnstakeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn unstake_invoke_signed<'info, A: Into<UnstakeIxArgs>>(
    accounts: &UnstakeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = unstake_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UNSTAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UNSTAKE_WSOL_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct UnstakeWsolAccounts<'me, 'info> {
    pub payer: &'me AccountInfo<'info>,
    pub unstaker: &'me AccountInfo<'info>,
    pub stake_account: &'me AccountInfo<'info>,
    pub destination: &'me AccountInfo<'info>,
    pub pool_account: &'me AccountInfo<'info>,
    pub pool_sol_reserves: &'me AccountInfo<'info>,
    pub fee_account: &'me AccountInfo<'info>,
    pub stake_account_record_account: &'me AccountInfo<'info>,
    pub protocol_fee_account: &'me AccountInfo<'info>,
    pub protocol_fee_destination: &'me AccountInfo<'info>,
    pub clock: &'me AccountInfo<'info>,
    pub stake_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UnstakeWsolKeys {
    pub payer: Pubkey,
    pub unstaker: Pubkey,
    pub stake_account: Pubkey,
    pub destination: Pubkey,
    pub pool_account: Pubkey,
    pub pool_sol_reserves: Pubkey,
    pub fee_account: Pubkey,
    pub stake_account_record_account: Pubkey,
    pub protocol_fee_account: Pubkey,
    pub protocol_fee_destination: Pubkey,
    pub clock: Pubkey,
    pub stake_program: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl From<&UnstakeWsolAccounts<'_, '_>> for UnstakeWsolKeys {
    fn from(accounts: &UnstakeWsolAccounts) -> Self {
        Self {
            payer: *accounts.payer.key,
            unstaker: *accounts.unstaker.key,
            stake_account: *accounts.stake_account.key,
            destination: *accounts.destination.key,
            pool_account: *accounts.pool_account.key,
            pool_sol_reserves: *accounts.pool_sol_reserves.key,
            fee_account: *accounts.fee_account.key,
            stake_account_record_account: *accounts.stake_account_record_account.key,
            protocol_fee_account: *accounts.protocol_fee_account.key,
            protocol_fee_destination: *accounts.protocol_fee_destination.key,
            clock: *accounts.clock.key,
            stake_program: *accounts.stake_program.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&UnstakeWsolKeys> for [AccountMeta; UNSTAKE_WSOL_IX_ACCOUNTS_LEN] {
    fn from(keys: &UnstakeWsolKeys) -> Self {
        [
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.unstaker, true),
            AccountMeta::new(keys.stake_account, false),
            AccountMeta::new(keys.destination, false),
            AccountMeta::new(keys.pool_account, false),
            AccountMeta::new(keys.pool_sol_reserves, false),
            AccountMeta::new_readonly(keys.fee_account, false),
            AccountMeta::new(keys.stake_account_record_account, false),
            AccountMeta::new_readonly(keys.protocol_fee_account, false),
            AccountMeta::new(keys.protocol_fee_destination, false),
            AccountMeta::new_readonly(keys.clock, false),
            AccountMeta::new_readonly(keys.stake_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl From<[Pubkey; UNSTAKE_WSOL_IX_ACCOUNTS_LEN]> for UnstakeWsolKeys {
    fn from(pubkeys: [Pubkey; UNSTAKE_WSOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: pubkeys[0],
            unstaker: pubkeys[1],
            stake_account: pubkeys[2],
            destination: pubkeys[3],
            pool_account: pubkeys[4],
            pool_sol_reserves: pubkeys[5],
            fee_account: pubkeys[6],
            stake_account_record_account: pubkeys[7],
            protocol_fee_account: pubkeys[8],
            protocol_fee_destination: pubkeys[9],
            clock: pubkeys[10],
            stake_program: pubkeys[11],
            system_program: pubkeys[12],
            token_program: pubkeys[13],
        }
    }
}
impl<'info> From<&UnstakeWsolAccounts<'_, 'info>>
    for [AccountInfo<'info>; UNSTAKE_WSOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UnstakeWsolAccounts<'_, 'info>) -> Self {
        [
            accounts.payer.clone(),
            accounts.unstaker.clone(),
            accounts.stake_account.clone(),
            accounts.destination.clone(),
            accounts.pool_account.clone(),
            accounts.pool_sol_reserves.clone(),
            accounts.fee_account.clone(),
            accounts.stake_account_record_account.clone(),
            accounts.protocol_fee_account.clone(),
            accounts.protocol_fee_destination.clone(),
            accounts.clock.clone(),
            accounts.stake_program.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNSTAKE_WSOL_IX_ACCOUNTS_LEN]>
    for UnstakeWsolAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UNSTAKE_WSOL_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            payer: &arr[0],
            unstaker: &arr[1],
            stake_account: &arr[2],
            destination: &arr[3],
            pool_account: &arr[4],
            pool_sol_reserves: &arr[5],
            fee_account: &arr[6],
            stake_account_record_account: &arr[7],
            protocol_fee_account: &arr[8],
            protocol_fee_destination: &arr[9],
            clock: &arr[10],
            stake_program: &arr[11],
            system_program: &arr[12],
            token_program: &arr[13],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnstakeWsolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UnstakeWsolIxData<'me>(pub &'me UnstakeWsolIxArgs);
pub const UNSTAKE_WSOL_IX_DISCM: [u8; 8] = [125, 93, 190, 135, 89, 174, 142, 149];
impl<'me> From<&'me UnstakeWsolIxArgs> for UnstakeWsolIxData<'me> {
    fn from(args: &'me UnstakeWsolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UnstakeWsolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_WSOL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn unstake_wsol_ix<K: Into<UnstakeWsolKeys>, A: Into<UnstakeWsolIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UnstakeWsolKeys = accounts.into();
    let metas: [AccountMeta; UNSTAKE_WSOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UnstakeWsolIxArgs = args.into();
    let data: UnstakeWsolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unstake_wsol_invoke<'info, A: Into<UnstakeWsolIxArgs>>(
    accounts: &UnstakeWsolAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = unstake_wsol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UNSTAKE_WSOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn unstake_wsol_invoke_signed<'info, A: Into<UnstakeWsolIxArgs>>(
    accounts: &UnstakeWsolAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = unstake_wsol_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UNSTAKE_WSOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
