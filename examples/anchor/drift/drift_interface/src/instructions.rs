use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
pub const INITIALIZE_USER_IX_ACCOUNTS_LEN: usize = 7usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub user: &'me AccountInfo<'a0>,
    pub user_stats: &'me AccountInfo<'a1>,
    pub state: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub payer: &'me AccountInfo<'a4>,
    pub rent: &'me AccountInfo<'a5>,
    pub system_program: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserKeys {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&InitializeUserAccounts<'me, '_, '_, '_, '_, '_, '_, '_>> for InitializeUserKeys {
    fn from(accounts: &InitializeUserAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeUserKeys> for [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeUserKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&InitializeUserAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_USER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeUserAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeUserIxArgs {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserIxData<'me>(pub &'me InitializeUserIxArgs);
pub const INITIALIZE_USER_IX_DISCM: [u8; 8] = [111, 17, 185, 250, 60, 122, 38, 254];
impl<'me> From<&'me InitializeUserIxArgs> for InitializeUserIxData<'me> {
    fn from(args: &'me InitializeUserIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeUserIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_user_ix<K: Into<InitializeUserKeys>, A: Into<InitializeUserIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeUserKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_USER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeUserIxArgs = args.into();
    let data: InitializeUserIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_user_invoke<'a, A: Into<InitializeUserIxArgs>>(
    accounts: &InitializeUserAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_user_invoke_signed<'a, A: Into<InitializeUserIxArgs>>(
    accounts: &InitializeUserAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserStatsAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub user_stats: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
    pub payer: &'me AccountInfo<'a3>,
    pub rent: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserStatsKeys {
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&InitializeUserStatsAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for InitializeUserStatsKeys
{
    fn from(accounts: &InitializeUserStatsAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeUserStatsKeys> for [AccountMeta; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeUserStatsKeys) -> Self {
        [
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&InitializeUserStatsAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeUserStatsAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeUserStatsIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct InitializeUserStatsIxData<'me>(pub &'me InitializeUserStatsIxArgs);
pub const INITIALIZE_USER_STATS_IX_DISCM: [u8; 8] = [254, 243, 72, 98, 251, 130, 168, 213];
impl<'me> From<&'me InitializeUserStatsIxArgs> for InitializeUserStatsIxData<'me> {
    fn from(args: &'me InitializeUserStatsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeUserStatsIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_USER_STATS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_user_stats_ix<
    K: Into<InitializeUserStatsKeys>,
    A: Into<InitializeUserStatsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeUserStatsKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeUserStatsIxArgs = args.into();
    let data: InitializeUserStatsIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_user_stats_invoke<'a, A: Into<InitializeUserStatsIxArgs>>(
    accounts: &InitializeUserStatsAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_user_stats_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_user_stats_invoke_signed<'a, A: Into<InitializeUserStatsIxArgs>>(
    accounts: &InitializeUserStatsAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_user_stats_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_USER_STATS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN: usize = 7usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeReferrerNameAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub referrer_name: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub payer: &'me AccountInfo<'a4>,
    pub rent: &'me AccountInfo<'a5>,
    pub system_program: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeReferrerNameKeys {
    pub referrer_name: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&InitializeReferrerNameAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for InitializeReferrerNameKeys
{
    fn from(accounts: &InitializeReferrerNameAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            referrer_name: *accounts.referrer_name.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeReferrerNameKeys> for [AccountMeta; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeReferrerNameKeys) -> Self {
        [
            AccountMeta::new(keys.referrer_name, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&InitializeReferrerNameAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeReferrerNameAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.referrer_name.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeReferrerNameIxArgs {
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeReferrerNameIxData<'me>(pub &'me InitializeReferrerNameIxArgs);
pub const INITIALIZE_REFERRER_NAME_IX_DISCM: [u8; 8] = [235, 126, 231, 10, 42, 164, 26, 61];
impl<'me> From<&'me InitializeReferrerNameIxArgs> for InitializeReferrerNameIxData<'me> {
    fn from(args: &'me InitializeReferrerNameIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeReferrerNameIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_REFERRER_NAME_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_referrer_name_ix<
    K: Into<InitializeReferrerNameKeys>,
    A: Into<InitializeReferrerNameIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeReferrerNameKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeReferrerNameIxArgs = args.into();
    let data: InitializeReferrerNameIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_referrer_name_invoke<'a, A: Into<InitializeReferrerNameIxArgs>>(
    accounts: &InitializeReferrerNameAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_referrer_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_referrer_name_invoke_signed<'a, A: Into<InitializeReferrerNameIxArgs>>(
    accounts: &InitializeReferrerNameAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_referrer_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_REFERRER_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPOSIT_IX_ACCOUNTS_LEN: usize = 7usize;
#[derive(Copy, Clone, Debug)]
pub struct DepositAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub spot_market_vault: &'me AccountInfo<'a4>,
    pub user_token_account: &'me AccountInfo<'a5>,
    pub token_program: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&DepositAccounts<'me, '_, '_, '_, '_, '_, '_, '_>> for DepositKeys {
    fn from(accounts: &DepositAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&DepositKeys> for [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &DepositKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&DepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DepositIxArgs {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositIxData<'me>(pub &'me DepositIxArgs);
pub const DEPOSIT_IX_DISCM: [u8; 8] = [242, 35, 198, 137, 82, 225, 242, 182];
impl<'me> From<&'me DepositIxArgs> for DepositIxData<'me> {
    fn from(args: &'me DepositIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DepositIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn deposit_ix<K: Into<DepositKeys>, A: Into<DepositIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositIxArgs = args.into();
    let data: DepositIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_invoke<'a, A: Into<DepositIxArgs>>(
    accounts: &DepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_invoke_signed<'a, A: Into<DepositIxArgs>>(
    accounts: &DepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 8usize;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub spot_market_vault: &'me AccountInfo<'a4>,
    pub drift_signer: &'me AccountInfo<'a5>,
    pub user_token_account: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&WithdrawAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>> for WithdrawKeys {
    fn from(accounts: &WithdrawAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            drift_signer: *accounts.drift_signer.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: &WithdrawKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&WithdrawAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; WITHDRAW_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &WithdrawAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct WithdrawIxArgs {
    pub market_index: u16,
    pub amount: u64,
    pub reduce_only: bool,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawIxData<'me>(pub &'me WithdrawIxArgs);
pub const WITHDRAW_IX_DISCM: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
impl<'me> From<&'me WithdrawIxArgs> for WithdrawIxData<'me> {
    fn from(args: &'me WithdrawIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for WithdrawIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn withdraw_ix<K: Into<WithdrawKeys>, A: Into<WithdrawIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: WithdrawKeys = accounts.into();
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: WithdrawIxArgs = args.into();
    let data: WithdrawIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn withdraw_invoke<'a, A: Into<WithdrawIxArgs>>(
    accounts: &WithdrawAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = withdraw_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; WITHDRAW_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn withdraw_invoke_signed<'a, A: Into<WithdrawIxArgs>>(
    accounts: &WithdrawAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = withdraw_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; WITHDRAW_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct TransferDepositAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me>
{
    pub from_user: &'me AccountInfo<'a0>,
    pub to_user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub state: &'me AccountInfo<'a4>,
    pub spot_market_vault: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct TransferDepositKeys {
    pub from_user: Pubkey,
    pub to_user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub state: Pubkey,
    pub spot_market_vault: Pubkey,
}
impl<'me> From<&TransferDepositAccounts<'me, '_, '_, '_, '_, '_, '_>> for TransferDepositKeys {
    fn from(accounts: &TransferDepositAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            from_user: *accounts.from_user.key,
            to_user: *accounts.to_user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            state: *accounts.state.key,
            spot_market_vault: *accounts.spot_market_vault.key,
        }
    }
}
impl From<&TransferDepositKeys> for [AccountMeta; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &TransferDepositKeys) -> Self {
        [
            AccountMeta::new(keys.from_user, false),
            AccountMeta::new(keys.to_user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market_vault, false),
        ]
    }
}
impl<'a> From<&TransferDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &TransferDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.from_user.clone(),
            accounts.to_user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.state.clone(),
            accounts.spot_market_vault.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct TransferDepositIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct TransferDepositIxData<'me>(pub &'me TransferDepositIxArgs);
pub const TRANSFER_DEPOSIT_IX_DISCM: [u8; 8] = [20, 20, 147, 223, 41, 63, 204, 111];
impl<'me> From<&'me TransferDepositIxArgs> for TransferDepositIxData<'me> {
    fn from(args: &'me TransferDepositIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for TransferDepositIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&TRANSFER_DEPOSIT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn transfer_deposit_ix<K: Into<TransferDepositKeys>, A: Into<TransferDepositIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: TransferDepositKeys = accounts.into();
    let metas: [AccountMeta; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: TransferDepositIxArgs = args.into();
    let data: TransferDepositIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn transfer_deposit_invoke<'a, A: Into<TransferDepositIxArgs>>(
    accounts: &TransferDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = transfer_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn transfer_deposit_invoke_signed<'a, A: Into<TransferDepositIxArgs>>(
    accounts: &TransferDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = transfer_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; TRANSFER_DEPOSIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PLACE_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct PlacePerpOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlacePerpOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&PlacePerpOrderAccounts<'me, '_, '_, '_>> for PlacePerpOrderKeys {
    fn from(accounts: &PlacePerpOrderAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlacePerpOrderKeys> for [AccountMeta; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlacePerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&PlacePerpOrderAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlacePerpOrderAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlacePerpOrderIxArgs {
    pub params: OrderParams,
}
#[derive(Copy, Clone, Debug)]
pub struct PlacePerpOrderIxData<'me>(pub &'me PlacePerpOrderIxArgs);
pub const PLACE_PERP_ORDER_IX_DISCM: [u8; 8] = [69, 161, 93, 202, 120, 126, 76, 185];
impl<'me> From<&'me PlacePerpOrderIxArgs> for PlacePerpOrderIxData<'me> {
    fn from(args: &'me PlacePerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlacePerpOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PLACE_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn place_perp_order_ix<K: Into<PlacePerpOrderKeys>, A: Into<PlacePerpOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlacePerpOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlacePerpOrderIxArgs = args.into();
    let data: PlacePerpOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_perp_order_invoke<'a, A: Into<PlacePerpOrderIxArgs>>(
    accounts: &PlacePerpOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = place_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_perp_order_invoke_signed<'a, A: Into<PlacePerpOrderIxArgs>>(
    accounts: &PlacePerpOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CANCEL_ORDER_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&CancelOrderAccounts<'me, '_, '_, '_>> for CancelOrderKeys {
    fn from(accounts: &CancelOrderAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&CancelOrderKeys> for [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&CancelOrderAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CANCEL_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelOrderAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CancelOrderIxArgs {
    pub order_id: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderIxData<'me>(pub &'me CancelOrderIxArgs);
pub const CANCEL_ORDER_IX_DISCM: [u8; 8] = [95, 129, 237, 240, 8, 49, 223, 132];
impl<'me> From<&'me CancelOrderIxArgs> for CancelOrderIxData<'me> {
    fn from(args: &'me CancelOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn cancel_order_ix<K: Into<CancelOrderKeys>, A: Into<CancelOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelOrderKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelOrderIxArgs = args.into();
    let data: CancelOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_invoke<'a, A: Into<CancelOrderIxArgs>>(
    accounts: &CancelOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = cancel_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CANCEL_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_order_invoke_signed<'a, A: Into<CancelOrderIxArgs>>(
    accounts: &CancelOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CANCEL_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByUserIdAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByUserIdKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&CancelOrderByUserIdAccounts<'me, '_, '_, '_>> for CancelOrderByUserIdKeys {
    fn from(accounts: &CancelOrderByUserIdAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&CancelOrderByUserIdKeys> for [AccountMeta; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelOrderByUserIdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&CancelOrderByUserIdAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelOrderByUserIdAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CancelOrderByUserIdIxArgs {
    pub user_order_id: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelOrderByUserIdIxData<'me>(pub &'me CancelOrderByUserIdIxArgs);
pub const CANCEL_ORDER_BY_USER_ID_IX_DISCM: [u8; 8] = [107, 211, 250, 133, 18, 37, 57, 100];
impl<'me> From<&'me CancelOrderByUserIdIxArgs> for CancelOrderByUserIdIxData<'me> {
    fn from(args: &'me CancelOrderByUserIdIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelOrderByUserIdIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDER_BY_USER_ID_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn cancel_order_by_user_id_ix<
    K: Into<CancelOrderByUserIdKeys>,
    A: Into<CancelOrderByUserIdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelOrderByUserIdKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelOrderByUserIdIxArgs = args.into();
    let data: CancelOrderByUserIdIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_order_by_user_id_invoke<'a, A: Into<CancelOrderByUserIdIxArgs>>(
    accounts: &CancelOrderByUserIdAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = cancel_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_order_by_user_id_invoke_signed<'a, A: Into<CancelOrderByUserIdIxArgs>>(
    accounts: &CancelOrderByUserIdAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CANCEL_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CANCEL_ORDERS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct CancelOrdersAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelOrdersKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&CancelOrdersAccounts<'me, '_, '_, '_>> for CancelOrdersKeys {
    fn from(accounts: &CancelOrdersAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&CancelOrdersKeys> for [AccountMeta; CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: &CancelOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&CancelOrdersAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CANCEL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CancelOrdersAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CancelOrdersIxArgs {
    pub market_type: Option<MarketType>,
    pub market_index: Option<u16>,
    pub direction: Option<PositionDirection>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelOrdersIxData<'me>(pub &'me CancelOrdersIxArgs);
pub const CANCEL_ORDERS_IX_DISCM: [u8; 8] = [238, 225, 95, 158, 227, 103, 8, 194];
impl<'me> From<&'me CancelOrdersIxArgs> for CancelOrdersIxData<'me> {
    fn from(args: &'me CancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelOrdersIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_ORDERS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn cancel_orders_ix<K: Into<CancelOrdersKeys>, A: Into<CancelOrdersIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelOrdersKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_ORDERS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CancelOrdersIxArgs = args.into();
    let data: CancelOrdersIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_orders_invoke<'a, A: Into<CancelOrdersIxArgs>>(
    accounts: &CancelOrdersAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_orders_invoke_signed<'a, A: Into<CancelOrdersIxArgs>>(
    accounts: &CancelOrdersAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MODIFY_ORDER_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&ModifyOrderAccounts<'me, '_, '_, '_>> for ModifyOrderKeys {
    fn from(accounts: &ModifyOrderAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&ModifyOrderKeys> for [AccountMeta; MODIFY_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &ModifyOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&ModifyOrderAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; MODIFY_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ModifyOrderAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ModifyOrderIxArgs {
    pub order_id: Option<u32>,
    pub modify_order_params: ModifyOrderParams,
}
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderIxData<'me>(pub &'me ModifyOrderIxArgs);
pub const MODIFY_ORDER_IX_DISCM: [u8; 8] = [47, 124, 117, 255, 201, 197, 130, 94];
impl<'me> From<&'me ModifyOrderIxArgs> for ModifyOrderIxData<'me> {
    fn from(args: &'me ModifyOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ModifyOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&MODIFY_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn modify_order_ix<K: Into<ModifyOrderKeys>, A: Into<ModifyOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ModifyOrderKeys = accounts.into();
    let metas: [AccountMeta; MODIFY_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ModifyOrderIxArgs = args.into();
    let data: ModifyOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn modify_order_invoke<'a, A: Into<ModifyOrderIxArgs>>(
    accounts: &ModifyOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = modify_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MODIFY_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn modify_order_invoke_signed<'a, A: Into<ModifyOrderIxArgs>>(
    accounts: &ModifyOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = modify_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MODIFY_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderByUserIdAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderByUserIdKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&ModifyOrderByUserIdAccounts<'me, '_, '_, '_>> for ModifyOrderByUserIdKeys {
    fn from(accounts: &ModifyOrderByUserIdAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&ModifyOrderByUserIdKeys> for [AccountMeta; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] {
    fn from(keys: &ModifyOrderByUserIdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&ModifyOrderByUserIdAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ModifyOrderByUserIdAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ModifyOrderByUserIdIxArgs {
    pub user_order_id: u8,
    pub modify_order_params: ModifyOrderParams,
}
#[derive(Copy, Clone, Debug)]
pub struct ModifyOrderByUserIdIxData<'me>(pub &'me ModifyOrderByUserIdIxArgs);
pub const MODIFY_ORDER_BY_USER_ID_IX_DISCM: [u8; 8] = [158, 77, 4, 253, 252, 194, 161, 179];
impl<'me> From<&'me ModifyOrderByUserIdIxArgs> for ModifyOrderByUserIdIxData<'me> {
    fn from(args: &'me ModifyOrderByUserIdIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ModifyOrderByUserIdIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&MODIFY_ORDER_BY_USER_ID_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn modify_order_by_user_id_ix<
    K: Into<ModifyOrderByUserIdKeys>,
    A: Into<ModifyOrderByUserIdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ModifyOrderByUserIdKeys = accounts.into();
    let metas: [AccountMeta; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ModifyOrderByUserIdIxArgs = args.into();
    let data: ModifyOrderByUserIdIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn modify_order_by_user_id_invoke<'a, A: Into<ModifyOrderByUserIdIxArgs>>(
    accounts: &ModifyOrderByUserIdAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = modify_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn modify_order_by_user_id_invoke_signed<'a, A: Into<ModifyOrderByUserIdIxArgs>>(
    accounts: &ModifyOrderByUserIdAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = modify_order_by_user_id_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MODIFY_ORDER_BY_USER_ID_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakePerpOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakePerpOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&PlaceAndTakePerpOrderAccounts<'me, '_, '_, '_, '_>> for PlaceAndTakePerpOrderKeys {
    fn from(accounts: &PlaceAndTakePerpOrderAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndTakePerpOrderKeys> for [AccountMeta; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndTakePerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&PlaceAndTakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndTakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlaceAndTakePerpOrderIxArgs {
    pub params: OrderParams,
    pub maker_order_id: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakePerpOrderIxData<'me>(pub &'me PlaceAndTakePerpOrderIxArgs);
pub const PLACE_AND_TAKE_PERP_ORDER_IX_DISCM: [u8; 8] = [213, 51, 1, 187, 108, 220, 230, 224];
impl<'me> From<&'me PlaceAndTakePerpOrderIxArgs> for PlaceAndTakePerpOrderIxData<'me> {
    fn from(args: &'me PlaceAndTakePerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceAndTakePerpOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_TAKE_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn place_and_take_perp_order_ix<
    K: Into<PlaceAndTakePerpOrderKeys>,
    A: Into<PlaceAndTakePerpOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndTakePerpOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndTakePerpOrderIxArgs = args.into();
    let data: PlaceAndTakePerpOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_take_perp_order_invoke<'a, A: Into<PlaceAndTakePerpOrderIxArgs>>(
    accounts: &PlaceAndTakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = place_and_take_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_take_perp_order_invoke_signed<'a, A: Into<PlaceAndTakePerpOrderIxArgs>>(
    accounts: &PlaceAndTakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_take_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_TAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakePerpOrderAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub taker: &'me AccountInfo<'a3>,
    pub taker_stats: &'me AccountInfo<'a4>,
    pub authority: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakePerpOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub taker: Pubkey,
    pub taker_stats: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&PlaceAndMakePerpOrderAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for PlaceAndMakePerpOrderKeys
{
    fn from(accounts: &PlaceAndMakePerpOrderAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            taker: *accounts.taker.key,
            taker_stats: *accounts.taker_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndMakePerpOrderKeys> for [AccountMeta; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndMakePerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.taker, false),
            AccountMeta::new(keys.taker_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&PlaceAndMakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndMakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.taker.clone(),
            accounts.taker_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlaceAndMakePerpOrderIxArgs {
    pub params: OrderParams,
    pub taker_order_id: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakePerpOrderIxData<'me>(pub &'me PlaceAndMakePerpOrderIxArgs);
pub const PLACE_AND_MAKE_PERP_ORDER_IX_DISCM: [u8; 8] = [149, 117, 11, 237, 47, 95, 89, 237];
impl<'me> From<&'me PlaceAndMakePerpOrderIxArgs> for PlaceAndMakePerpOrderIxData<'me> {
    fn from(args: &'me PlaceAndMakePerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceAndMakePerpOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_MAKE_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn place_and_make_perp_order_ix<
    K: Into<PlaceAndMakePerpOrderKeys>,
    A: Into<PlaceAndMakePerpOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndMakePerpOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndMakePerpOrderIxArgs = args.into();
    let data: PlaceAndMakePerpOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_make_perp_order_invoke<'a, A: Into<PlaceAndMakePerpOrderIxArgs>>(
    accounts: &PlaceAndMakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = place_and_make_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_make_perp_order_invoke_signed<'a, A: Into<PlaceAndMakePerpOrderIxArgs>>(
    accounts: &PlaceAndMakePerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_make_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_MAKE_PERP_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct PlaceSpotOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceSpotOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&PlaceSpotOrderAccounts<'me, '_, '_, '_>> for PlaceSpotOrderKeys {
    fn from(accounts: &PlaceSpotOrderAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceSpotOrderKeys> for [AccountMeta; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&PlaceSpotOrderAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceSpotOrderAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlaceSpotOrderIxArgs {
    pub params: OrderParams,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceSpotOrderIxData<'me>(pub &'me PlaceSpotOrderIxArgs);
pub const PLACE_SPOT_ORDER_IX_DISCM: [u8; 8] = [45, 79, 81, 160, 248, 90, 91, 220];
impl<'me> From<&'me PlaceSpotOrderIxArgs> for PlaceSpotOrderIxData<'me> {
    fn from(args: &'me PlaceSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceSpotOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PLACE_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn place_spot_order_ix<K: Into<PlaceSpotOrderKeys>, A: Into<PlaceSpotOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceSpotOrderIxArgs = args.into();
    let data: PlaceSpotOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_spot_order_invoke<'a, A: Into<PlaceSpotOrderIxArgs>>(
    accounts: &PlaceSpotOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = place_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_spot_order_invoke_signed<'a, A: Into<PlaceSpotOrderIxArgs>>(
    accounts: &PlaceSpotOrderAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakeSpotOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakeSpotOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&PlaceAndTakeSpotOrderAccounts<'me, '_, '_, '_, '_>> for PlaceAndTakeSpotOrderKeys {
    fn from(accounts: &PlaceAndTakeSpotOrderAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndTakeSpotOrderKeys> for [AccountMeta; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndTakeSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&PlaceAndTakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndTakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlaceAndTakeSpotOrderIxArgs {
    pub params: OrderParams,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndTakeSpotOrderIxData<'me>(pub &'me PlaceAndTakeSpotOrderIxArgs);
pub const PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM: [u8; 8] = [191, 3, 138, 71, 114, 198, 202, 100];
impl<'me> From<&'me PlaceAndTakeSpotOrderIxArgs> for PlaceAndTakeSpotOrderIxData<'me> {
    fn from(args: &'me PlaceAndTakeSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceAndTakeSpotOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_TAKE_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn place_and_take_spot_order_ix<
    K: Into<PlaceAndTakeSpotOrderKeys>,
    A: Into<PlaceAndTakeSpotOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndTakeSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndTakeSpotOrderIxArgs = args.into();
    let data: PlaceAndTakeSpotOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_take_spot_order_invoke<'a, A: Into<PlaceAndTakeSpotOrderIxArgs>>(
    accounts: &PlaceAndTakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = place_and_take_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_take_spot_order_invoke_signed<'a, A: Into<PlaceAndTakeSpotOrderIxArgs>>(
    accounts: &PlaceAndTakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_take_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_TAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakeSpotOrderAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub taker: &'me AccountInfo<'a3>,
    pub taker_stats: &'me AccountInfo<'a4>,
    pub authority: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakeSpotOrderKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub taker: Pubkey,
    pub taker_stats: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&PlaceAndMakeSpotOrderAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for PlaceAndMakeSpotOrderKeys
{
    fn from(accounts: &PlaceAndMakeSpotOrderAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            taker: *accounts.taker.key,
            taker_stats: *accounts.taker_stats.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&PlaceAndMakeSpotOrderKeys> for [AccountMeta; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &PlaceAndMakeSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.taker, false),
            AccountMeta::new(keys.taker_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&PlaceAndMakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PlaceAndMakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.taker.clone(),
            accounts.taker_stats.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PlaceAndMakeSpotOrderIxArgs {
    pub params: OrderParams,
    pub taker_order_id: u32,
    pub fulfillment_type: Option<SpotFulfillmentType>,
}
#[derive(Copy, Clone, Debug)]
pub struct PlaceAndMakeSpotOrderIxData<'me>(pub &'me PlaceAndMakeSpotOrderIxArgs);
pub const PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM: [u8; 8] = [149, 158, 85, 66, 239, 9, 243, 98];
impl<'me> From<&'me PlaceAndMakeSpotOrderIxArgs> for PlaceAndMakeSpotOrderIxData<'me> {
    fn from(args: &'me PlaceAndMakeSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PlaceAndMakeSpotOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PLACE_AND_MAKE_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn place_and_make_spot_order_ix<
    K: Into<PlaceAndMakeSpotOrderKeys>,
    A: Into<PlaceAndMakeSpotOrderIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PlaceAndMakeSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PlaceAndMakeSpotOrderIxArgs = args.into();
    let data: PlaceAndMakeSpotOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn place_and_make_spot_order_invoke<'a, A: Into<PlaceAndMakeSpotOrderIxArgs>>(
    accounts: &PlaceAndMakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = place_and_make_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn place_and_make_spot_order_invoke_signed<'a, A: Into<PlaceAndMakeSpotOrderIxArgs>>(
    accounts: &PlaceAndMakeSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = place_and_make_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PLACE_AND_MAKE_SPOT_ORDER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const BEGIN_SWAP_IX_ACCOUNTS_LEN: usize = 11usize;
#[derive(Copy, Clone, Debug)]
pub struct BeginSwapAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub out_spot_market_vault: &'me AccountInfo<'a4>,
    pub in_spot_market_vault: &'me AccountInfo<'a5>,
    pub out_token_account: &'me AccountInfo<'a6>,
    pub in_token_account: &'me AccountInfo<'a7>,
    pub token_program: &'me AccountInfo<'a8>,
    pub drift_signer: &'me AccountInfo<'a9>,
    pub instructions: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct BeginSwapKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub out_spot_market_vault: Pubkey,
    pub in_spot_market_vault: Pubkey,
    pub out_token_account: Pubkey,
    pub in_token_account: Pubkey,
    pub token_program: Pubkey,
    pub drift_signer: Pubkey,
    pub instructions: Pubkey,
}
impl<'me> From<&BeginSwapAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for BeginSwapKeys
{
    fn from(accounts: &BeginSwapAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            out_spot_market_vault: *accounts.out_spot_market_vault.key,
            in_spot_market_vault: *accounts.in_spot_market_vault.key,
            out_token_account: *accounts.out_token_account.key,
            in_token_account: *accounts.in_token_account.key,
            token_program: *accounts.token_program.key,
            drift_signer: *accounts.drift_signer.key,
            instructions: *accounts.instructions.key,
        }
    }
}
impl From<&BeginSwapKeys> for [AccountMeta; BEGIN_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: &BeginSwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.out_spot_market_vault, false),
            AccountMeta::new(keys.in_spot_market_vault, false),
            AccountMeta::new(keys.out_token_account, false),
            AccountMeta::new(keys.in_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.instructions, false),
        ]
    }
}
impl<'a> From<&BeginSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; BEGIN_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &BeginSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.out_spot_market_vault.clone(),
            accounts.in_spot_market_vault.clone(),
            accounts.out_token_account.clone(),
            accounts.in_token_account.clone(),
            accounts.token_program.clone(),
            accounts.drift_signer.clone(),
            accounts.instructions.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct BeginSwapIxArgs {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub amount_in: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct BeginSwapIxData<'me>(pub &'me BeginSwapIxArgs);
pub const BEGIN_SWAP_IX_DISCM: [u8; 8] = [174, 109, 228, 1, 242, 105, 232, 105];
impl<'me> From<&'me BeginSwapIxArgs> for BeginSwapIxData<'me> {
    fn from(args: &'me BeginSwapIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for BeginSwapIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&BEGIN_SWAP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn begin_swap_ix<K: Into<BeginSwapKeys>, A: Into<BeginSwapIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: BeginSwapKeys = accounts.into();
    let metas: [AccountMeta; BEGIN_SWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: BeginSwapIxArgs = args.into();
    let data: BeginSwapIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn begin_swap_invoke<'a, A: Into<BeginSwapIxArgs>>(
    accounts: &BeginSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = begin_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; BEGIN_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn begin_swap_invoke_signed<'a, A: Into<BeginSwapIxArgs>>(
    accounts: &BeginSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = begin_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; BEGIN_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const END_SWAP_IX_ACCOUNTS_LEN: usize = 11usize;
#[derive(Copy, Clone, Debug)]
pub struct EndSwapAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub out_spot_market_vault: &'me AccountInfo<'a4>,
    pub in_spot_market_vault: &'me AccountInfo<'a5>,
    pub out_token_account: &'me AccountInfo<'a6>,
    pub in_token_account: &'me AccountInfo<'a7>,
    pub token_program: &'me AccountInfo<'a8>,
    pub drift_signer: &'me AccountInfo<'a9>,
    pub instructions: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct EndSwapKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub out_spot_market_vault: Pubkey,
    pub in_spot_market_vault: Pubkey,
    pub out_token_account: Pubkey,
    pub in_token_account: Pubkey,
    pub token_program: Pubkey,
    pub drift_signer: Pubkey,
    pub instructions: Pubkey,
}
impl<'me> From<&EndSwapAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>> for EndSwapKeys {
    fn from(accounts: &EndSwapAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            out_spot_market_vault: *accounts.out_spot_market_vault.key,
            in_spot_market_vault: *accounts.in_spot_market_vault.key,
            out_token_account: *accounts.out_token_account.key,
            in_token_account: *accounts.in_token_account.key,
            token_program: *accounts.token_program.key,
            drift_signer: *accounts.drift_signer.key,
            instructions: *accounts.instructions.key,
        }
    }
}
impl From<&EndSwapKeys> for [AccountMeta; END_SWAP_IX_ACCOUNTS_LEN] {
    fn from(keys: &EndSwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.out_spot_market_vault, false),
            AccountMeta::new(keys.in_spot_market_vault, false),
            AccountMeta::new(keys.out_token_account, false),
            AccountMeta::new(keys.in_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.instructions, false),
        ]
    }
}
impl<'a> From<&EndSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; END_SWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &EndSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.out_spot_market_vault.clone(),
            accounts.in_spot_market_vault.clone(),
            accounts.out_token_account.clone(),
            accounts.in_token_account.clone(),
            accounts.token_program.clone(),
            accounts.drift_signer.clone(),
            accounts.instructions.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct EndSwapIxArgs {
    pub in_market_index: u16,
    pub out_market_index: u16,
    pub limit_price: Option<u64>,
    pub reduce_only: Option<SwapReduceOnly>,
}
#[derive(Copy, Clone, Debug)]
pub struct EndSwapIxData<'me>(pub &'me EndSwapIxArgs);
pub const END_SWAP_IX_DISCM: [u8; 8] = [177, 184, 27, 193, 34, 13, 210, 145];
impl<'me> From<&'me EndSwapIxArgs> for EndSwapIxData<'me> {
    fn from(args: &'me EndSwapIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for EndSwapIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&END_SWAP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn end_swap_ix<K: Into<EndSwapKeys>, A: Into<EndSwapIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: EndSwapKeys = accounts.into();
    let metas: [AccountMeta; END_SWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: EndSwapIxArgs = args.into();
    let data: EndSwapIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn end_swap_invoke<'a, A: Into<EndSwapIxArgs>>(
    accounts: &EndSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = end_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; END_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn end_swap_invoke_signed<'a, A: Into<EndSwapIxArgs>>(
    accounts: &EndSwapAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = end_swap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; END_SWAP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct AddPerpLpSharesAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddPerpLpSharesKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&AddPerpLpSharesAccounts<'me, '_, '_, '_>> for AddPerpLpSharesKeys {
    fn from(accounts: &AddPerpLpSharesAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&AddPerpLpSharesKeys> for [AccountMeta; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddPerpLpSharesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&AddPerpLpSharesAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &AddPerpLpSharesAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct AddPerpLpSharesIxArgs {
    pub n_shares: u64,
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct AddPerpLpSharesIxData<'me>(pub &'me AddPerpLpSharesIxArgs);
pub const ADD_PERP_LP_SHARES_IX_DISCM: [u8; 8] = [56, 209, 56, 197, 119, 254, 188, 117];
impl<'me> From<&'me AddPerpLpSharesIxArgs> for AddPerpLpSharesIxData<'me> {
    fn from(args: &'me AddPerpLpSharesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddPerpLpSharesIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADD_PERP_LP_SHARES_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn add_perp_lp_shares_ix<K: Into<AddPerpLpSharesKeys>, A: Into<AddPerpLpSharesIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddPerpLpSharesKeys = accounts.into();
    let metas: [AccountMeta; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddPerpLpSharesIxArgs = args.into();
    let data: AddPerpLpSharesIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_perp_lp_shares_invoke<'a, A: Into<AddPerpLpSharesIxArgs>>(
    accounts: &AddPerpLpSharesAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = add_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_perp_lp_shares_invoke_signed<'a, A: Into<AddPerpLpSharesIxArgs>>(
    accounts: &AddPerpLpSharesAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADD_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&RemovePerpLpSharesAccounts<'me, '_, '_, '_>> for RemovePerpLpSharesKeys {
    fn from(accounts: &RemovePerpLpSharesAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&RemovePerpLpSharesKeys> for [AccountMeta; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] {
    fn from(keys: &RemovePerpLpSharesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&RemovePerpLpSharesAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemovePerpLpSharesAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RemovePerpLpSharesIxArgs {
    pub shares_to_burn: u64,
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesIxData<'me>(pub &'me RemovePerpLpSharesIxArgs);
pub const REMOVE_PERP_LP_SHARES_IX_DISCM: [u8; 8] = [213, 89, 217, 18, 160, 55, 53, 141];
impl<'me> From<&'me RemovePerpLpSharesIxArgs> for RemovePerpLpSharesIxData<'me> {
    fn from(args: &'me RemovePerpLpSharesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemovePerpLpSharesIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PERP_LP_SHARES_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn remove_perp_lp_shares_ix<
    K: Into<RemovePerpLpSharesKeys>,
    A: Into<RemovePerpLpSharesIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemovePerpLpSharesKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemovePerpLpSharesIxArgs = args.into();
    let data: RemovePerpLpSharesIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_perp_lp_shares_invoke<'a, A: Into<RemovePerpLpSharesIxArgs>>(
    accounts: &RemovePerpLpSharesAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = remove_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_perp_lp_shares_invoke_signed<'a, A: Into<RemovePerpLpSharesIxArgs>>(
    accounts: &RemovePerpLpSharesAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_perp_lp_shares_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_PERP_LP_SHARES_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesInExpiringMarketAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesInExpiringMarketKeys {
    pub state: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&RemovePerpLpSharesInExpiringMarketAccounts<'me, '_, '_>>
    for RemovePerpLpSharesInExpiringMarketKeys
{
    fn from(accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'me, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&RemovePerpLpSharesInExpiringMarketKeys>
    for [AccountMeta; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RemovePerpLpSharesInExpiringMarketKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&RemovePerpLpSharesInExpiringMarketAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, 'a, 'a>) -> Self {
        [accounts.state.clone(), accounts.user.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RemovePerpLpSharesInExpiringMarketIxArgs {
    pub shares_to_burn: u64,
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct RemovePerpLpSharesInExpiringMarketIxData<'me>(
    pub &'me RemovePerpLpSharesInExpiringMarketIxArgs,
);
pub const REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM: [u8; 8] =
    [83, 254, 253, 137, 59, 122, 68, 156];
impl<'me> From<&'me RemovePerpLpSharesInExpiringMarketIxArgs>
    for RemovePerpLpSharesInExpiringMarketIxData<'me>
{
    fn from(args: &'me RemovePerpLpSharesInExpiringMarketIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemovePerpLpSharesInExpiringMarketIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn remove_perp_lp_shares_in_expiring_market_ix<
    K: Into<RemovePerpLpSharesInExpiringMarketKeys>,
    A: Into<RemovePerpLpSharesInExpiringMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemovePerpLpSharesInExpiringMarketKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: RemovePerpLpSharesInExpiringMarketIxArgs = args.into();
    let data: RemovePerpLpSharesInExpiringMarketIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_perp_lp_shares_in_expiring_market_invoke<
    'a,
    A: Into<RemovePerpLpSharesInExpiringMarketIxArgs>,
>(
    accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = remove_perp_lp_shares_in_expiring_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_perp_lp_shares_in_expiring_market_invoke_signed<
    'a,
    A: Into<RemovePerpLpSharesInExpiringMarketIxArgs>,
>(
    accounts: &RemovePerpLpSharesInExpiringMarketAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_perp_lp_shares_in_expiring_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_PERP_LP_SHARES_IN_EXPIRING_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_NAME_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserNameAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub user: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserNameKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&UpdateUserNameAccounts<'me, '_, '_>> for UpdateUserNameKeys {
    fn from(accounts: &UpdateUserNameAccounts<'me, '_, '_>) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserNameKeys> for [AccountMeta; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateUserNameKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&UpdateUserNameAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserNameAccounts<'_, 'a, 'a>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserNameIxArgs {
    pub sub_account_id: u16,
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserNameIxData<'me>(pub &'me UpdateUserNameIxArgs);
pub const UPDATE_USER_NAME_IX_DISCM: [u8; 8] = [135, 25, 185, 56, 165, 53, 34, 136];
impl<'me> From<&'me UpdateUserNameIxArgs> for UpdateUserNameIxData<'me> {
    fn from(args: &'me UpdateUserNameIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserNameIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_NAME_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_name_ix<K: Into<UpdateUserNameKeys>, A: Into<UpdateUserNameIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserNameKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserNameIxArgs = args.into();
    let data: UpdateUserNameIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_name_invoke<'a, A: Into<UpdateUserNameIxArgs>>(
    accounts: &UpdateUserNameAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_name_invoke_signed<'a, A: Into<UpdateUserNameIxArgs>>(
    accounts: &UpdateUserNameAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserCustomMarginRatioAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub user: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserCustomMarginRatioKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&UpdateUserCustomMarginRatioAccounts<'me, '_, '_>>
    for UpdateUserCustomMarginRatioKeys
{
    fn from(accounts: &UpdateUserCustomMarginRatioAccounts<'me, '_, '_>) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserCustomMarginRatioKeys>
    for [AccountMeta; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserCustomMarginRatioKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&UpdateUserCustomMarginRatioAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserCustomMarginRatioAccounts<'_, 'a, 'a>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserCustomMarginRatioIxArgs {
    pub sub_account_id: u16,
    pub margin_ratio: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserCustomMarginRatioIxData<'me>(pub &'me UpdateUserCustomMarginRatioIxArgs);
pub const UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM: [u8; 8] = [21, 221, 140, 187, 32, 129, 11, 123];
impl<'me> From<&'me UpdateUserCustomMarginRatioIxArgs> for UpdateUserCustomMarginRatioIxData<'me> {
    fn from(args: &'me UpdateUserCustomMarginRatioIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserCustomMarginRatioIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_custom_margin_ratio_ix<
    K: Into<UpdateUserCustomMarginRatioKeys>,
    A: Into<UpdateUserCustomMarginRatioIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserCustomMarginRatioKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserCustomMarginRatioIxArgs = args.into();
    let data: UpdateUserCustomMarginRatioIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_custom_margin_ratio_invoke<'a, A: Into<UpdateUserCustomMarginRatioIxArgs>>(
    accounts: &UpdateUserCustomMarginRatioAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_custom_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_custom_margin_ratio_invoke_signed<
    'a,
    A: Into<UpdateUserCustomMarginRatioIxArgs>,
>(
    accounts: &UpdateUserCustomMarginRatioAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_custom_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_CUSTOM_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserMarginTradingEnabledAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub user: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserMarginTradingEnabledKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&UpdateUserMarginTradingEnabledAccounts<'me, '_, '_>>
    for UpdateUserMarginTradingEnabledKeys
{
    fn from(accounts: &UpdateUserMarginTradingEnabledAccounts<'me, '_, '_>) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserMarginTradingEnabledKeys>
    for [AccountMeta; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserMarginTradingEnabledKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&UpdateUserMarginTradingEnabledAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserMarginTradingEnabledAccounts<'_, 'a, 'a>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserMarginTradingEnabledIxArgs {
    pub sub_account_id: u16,
    pub margin_trading_enabled: bool,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserMarginTradingEnabledIxData<'me>(pub &'me UpdateUserMarginTradingEnabledIxArgs);
pub const UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM: [u8; 8] =
    [194, 92, 204, 223, 246, 188, 31, 203];
impl<'me> From<&'me UpdateUserMarginTradingEnabledIxArgs>
    for UpdateUserMarginTradingEnabledIxData<'me>
{
    fn from(args: &'me UpdateUserMarginTradingEnabledIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserMarginTradingEnabledIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_MARGIN_TRADING_ENABLED_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_margin_trading_enabled_ix<
    K: Into<UpdateUserMarginTradingEnabledKeys>,
    A: Into<UpdateUserMarginTradingEnabledIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserMarginTradingEnabledKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserMarginTradingEnabledIxArgs = args.into();
    let data: UpdateUserMarginTradingEnabledIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_margin_trading_enabled_invoke<
    'a,
    A: Into<UpdateUserMarginTradingEnabledIxArgs>,
>(
    accounts: &UpdateUserMarginTradingEnabledAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_margin_trading_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_margin_trading_enabled_invoke_signed<
    'a,
    A: Into<UpdateUserMarginTradingEnabledIxArgs>,
>(
    accounts: &UpdateUserMarginTradingEnabledAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_margin_trading_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_MARGIN_TRADING_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserDelegateAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub user: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserDelegateKeys {
    pub user: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&UpdateUserDelegateAccounts<'me, '_, '_>> for UpdateUserDelegateKeys {
    fn from(accounts: &UpdateUserDelegateAccounts<'me, '_, '_>) -> Self {
        Self {
            user: *accounts.user.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateUserDelegateKeys> for [AccountMeta; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateUserDelegateKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&UpdateUserDelegateAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserDelegateAccounts<'_, 'a, 'a>) -> Self {
        [accounts.user.clone(), accounts.authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserDelegateIxArgs {
    pub sub_account_id: u16,
    pub delegate: Pubkey,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserDelegateIxData<'me>(pub &'me UpdateUserDelegateIxArgs);
pub const UPDATE_USER_DELEGATE_IX_DISCM: [u8; 8] = [139, 205, 141, 141, 113, 36, 94, 187];
impl<'me> From<&'me UpdateUserDelegateIxArgs> for UpdateUserDelegateIxData<'me> {
    fn from(args: &'me UpdateUserDelegateIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserDelegateIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_DELEGATE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_delegate_ix<
    K: Into<UpdateUserDelegateKeys>,
    A: Into<UpdateUserDelegateIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserDelegateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserDelegateIxArgs = args.into();
    let data: UpdateUserDelegateIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_delegate_invoke<'a, A: Into<UpdateUserDelegateIxArgs>>(
    accounts: &UpdateUserDelegateAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_delegate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_delegate_invoke_signed<'a, A: Into<UpdateUserDelegateIxArgs>>(
    accounts: &UpdateUserDelegateAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_delegate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_DELEGATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DELETE_USER_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct DeleteUserAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub user: &'me AccountInfo<'a0>,
    pub user_stats: &'me AccountInfo<'a1>,
    pub state: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeleteUserKeys {
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&DeleteUserAccounts<'me, '_, '_, '_, '_>> for DeleteUserKeys {
    fn from(accounts: &DeleteUserAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&DeleteUserKeys> for [AccountMeta; DELETE_USER_IX_ACCOUNTS_LEN] {
    fn from(keys: &DeleteUserKeys) -> Self {
        [
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&DeleteUserAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DELETE_USER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeleteUserAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeleteUserIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct DeleteUserIxData<'me>(pub &'me DeleteUserIxArgs);
pub const DELETE_USER_IX_DISCM: [u8; 8] = [186, 85, 17, 249, 219, 231, 98, 251];
impl<'me> From<&'me DeleteUserIxArgs> for DeleteUserIxData<'me> {
    fn from(args: &'me DeleteUserIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeleteUserIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DELETE_USER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn delete_user_ix<K: Into<DeleteUserKeys>, A: Into<DeleteUserIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeleteUserKeys = accounts.into();
    let metas: [AccountMeta; DELETE_USER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeleteUserIxArgs = args.into();
    let data: DeleteUserIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delete_user_invoke<'a, A: Into<DeleteUserIxArgs>>(
    accounts: &DeleteUserAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = delete_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DELETE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn delete_user_invoke_signed<'a, A: Into<DeleteUserIxArgs>>(
    accounts: &DeleteUserAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = delete_user_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DELETE_USER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const FILL_PERP_ORDER_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct FillPerpOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub filler_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct FillPerpOrderKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl<'me> From<&FillPerpOrderAccounts<'me, '_, '_, '_, '_, '_, '_>> for FillPerpOrderKeys {
    fn from(accounts: &FillPerpOrderAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            filler_stats: *accounts.filler_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&FillPerpOrderKeys> for [AccountMeta; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &FillPerpOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.filler_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl<'a> From<&FillPerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &FillPerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.filler_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct FillPerpOrderIxArgs {
    pub order_id: Option<u32>,
    pub maker_order_id: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub struct FillPerpOrderIxData<'me>(pub &'me FillPerpOrderIxArgs);
pub const FILL_PERP_ORDER_IX_DISCM: [u8; 8] = [13, 188, 248, 103, 134, 217, 106, 240];
impl<'me> From<&'me FillPerpOrderIxArgs> for FillPerpOrderIxData<'me> {
    fn from(args: &'me FillPerpOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for FillPerpOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&FILL_PERP_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn fill_perp_order_ix<K: Into<FillPerpOrderKeys>, A: Into<FillPerpOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: FillPerpOrderKeys = accounts.into();
    let metas: [AccountMeta; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: FillPerpOrderIxArgs = args.into();
    let data: FillPerpOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_perp_order_invoke<'a, A: Into<FillPerpOrderIxArgs>>(
    accounts: &FillPerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = fill_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn fill_perp_order_invoke_signed<'a, A: Into<FillPerpOrderIxArgs>>(
    accounts: &FillPerpOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = fill_perp_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FILL_PERP_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REVERT_FILL_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct RevertFillAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub filler_stats: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevertFillKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
}
impl<'me> From<&RevertFillAccounts<'me, '_, '_, '_, '_>> for RevertFillKeys {
    fn from(accounts: &RevertFillAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            filler_stats: *accounts.filler_stats.key,
        }
    }
}
impl From<&RevertFillKeys> for [AccountMeta; REVERT_FILL_IX_ACCOUNTS_LEN] {
    fn from(keys: &RevertFillKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.filler_stats, false),
        ]
    }
}
impl<'a> From<&RevertFillAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REVERT_FILL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RevertFillAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.filler_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RevertFillIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RevertFillIxData<'me>(pub &'me RevertFillIxArgs);
pub const REVERT_FILL_IX_DISCM: [u8; 8] = [236, 238, 176, 69, 239, 10, 181, 193];
impl<'me> From<&'me RevertFillIxArgs> for RevertFillIxData<'me> {
    fn from(args: &'me RevertFillIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RevertFillIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REVERT_FILL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn revert_fill_ix<K: Into<RevertFillKeys>, A: Into<RevertFillIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RevertFillKeys = accounts.into();
    let metas: [AccountMeta; REVERT_FILL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RevertFillIxArgs = args.into();
    let data: RevertFillIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revert_fill_invoke<'a, A: Into<RevertFillIxArgs>>(
    accounts: &RevertFillAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = revert_fill_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REVERT_FILL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revert_fill_invoke_signed<'a, A: Into<RevertFillIxArgs>>(
    accounts: &RevertFillAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revert_fill_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REVERT_FILL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const FILL_SPOT_ORDER_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct FillSpotOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub filler_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct FillSpotOrderKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub filler_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl<'me> From<&FillSpotOrderAccounts<'me, '_, '_, '_, '_, '_, '_>> for FillSpotOrderKeys {
    fn from(accounts: &FillSpotOrderAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            filler_stats: *accounts.filler_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&FillSpotOrderKeys> for [AccountMeta; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &FillSpotOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.filler_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl<'a> From<&FillSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &FillSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.filler_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct FillSpotOrderIxArgs {
    pub order_id: Option<u32>,
    pub fulfillment_type: Option<SpotFulfillmentType>,
    pub maker_order_id: Option<u32>,
}
#[derive(Copy, Clone, Debug)]
pub struct FillSpotOrderIxData<'me>(pub &'me FillSpotOrderIxArgs);
pub const FILL_SPOT_ORDER_IX_DISCM: [u8; 8] = [212, 206, 130, 173, 21, 34, 199, 40];
impl<'me> From<&'me FillSpotOrderIxArgs> for FillSpotOrderIxData<'me> {
    fn from(args: &'me FillSpotOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for FillSpotOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&FILL_SPOT_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn fill_spot_order_ix<K: Into<FillSpotOrderKeys>, A: Into<FillSpotOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: FillSpotOrderKeys = accounts.into();
    let metas: [AccountMeta; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: FillSpotOrderIxArgs = args.into();
    let data: FillSpotOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn fill_spot_order_invoke<'a, A: Into<FillSpotOrderIxArgs>>(
    accounts: &FillSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = fill_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn fill_spot_order_invoke_signed<'a, A: Into<FillSpotOrderIxArgs>>(
    accounts: &FillSpotOrderAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = fill_spot_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FILL_SPOT_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const TRIGGER_ORDER_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct TriggerOrderAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub user: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct TriggerOrderKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&TriggerOrderAccounts<'me, '_, '_, '_, '_>> for TriggerOrderKeys {
    fn from(accounts: &TriggerOrderAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&TriggerOrderKeys> for [AccountMeta; TRIGGER_ORDER_IX_ACCOUNTS_LEN] {
    fn from(keys: &TriggerOrderKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&TriggerOrderAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; TRIGGER_ORDER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &TriggerOrderAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct TriggerOrderIxArgs {
    pub order_id: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct TriggerOrderIxData<'me>(pub &'me TriggerOrderIxArgs);
pub const TRIGGER_ORDER_IX_DISCM: [u8; 8] = [63, 112, 51, 233, 232, 47, 240, 199];
impl<'me> From<&'me TriggerOrderIxArgs> for TriggerOrderIxData<'me> {
    fn from(args: &'me TriggerOrderIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for TriggerOrderIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&TRIGGER_ORDER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn trigger_order_ix<K: Into<TriggerOrderKeys>, A: Into<TriggerOrderIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: TriggerOrderKeys = accounts.into();
    let metas: [AccountMeta; TRIGGER_ORDER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: TriggerOrderIxArgs = args.into();
    let data: TriggerOrderIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn trigger_order_invoke<'a, A: Into<TriggerOrderIxArgs>>(
    accounts: &TriggerOrderAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = trigger_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; TRIGGER_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn trigger_order_invoke_signed<'a, A: Into<TriggerOrderIxArgs>>(
    accounts: &TriggerOrderAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = trigger_order_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; TRIGGER_ORDER_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct ForceCancelOrdersAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub user: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct ForceCancelOrdersKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&ForceCancelOrdersAccounts<'me, '_, '_, '_, '_>> for ForceCancelOrdersKeys {
    fn from(accounts: &ForceCancelOrdersAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&ForceCancelOrdersKeys> for [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] {
    fn from(keys: &ForceCancelOrdersKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&ForceCancelOrdersAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ForceCancelOrdersAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ForceCancelOrdersIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ForceCancelOrdersIxData<'me>(pub &'me ForceCancelOrdersIxArgs);
pub const FORCE_CANCEL_ORDERS_IX_DISCM: [u8; 8] = [64, 181, 196, 63, 222, 72, 64, 232];
impl<'me> From<&'me ForceCancelOrdersIxArgs> for ForceCancelOrdersIxData<'me> {
    fn from(args: &'me ForceCancelOrdersIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ForceCancelOrdersIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&FORCE_CANCEL_ORDERS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn force_cancel_orders_ix<K: Into<ForceCancelOrdersKeys>, A: Into<ForceCancelOrdersIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ForceCancelOrdersKeys = accounts.into();
    let metas: [AccountMeta; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ForceCancelOrdersIxArgs = args.into();
    let data: ForceCancelOrdersIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn force_cancel_orders_invoke<'a, A: Into<ForceCancelOrdersIxArgs>>(
    accounts: &ForceCancelOrdersAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = force_cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn force_cancel_orders_invoke_signed<'a, A: Into<ForceCancelOrdersIxArgs>>(
    accounts: &ForceCancelOrdersAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = force_cancel_orders_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FORCE_CANCEL_ORDERS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_IDLE_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserIdleAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub user: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserIdleKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&UpdateUserIdleAccounts<'me, '_, '_, '_, '_>> for UpdateUserIdleKeys {
    fn from(accounts: &UpdateUserIdleAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&UpdateUserIdleKeys> for [AccountMeta; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateUserIdleKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&UpdateUserIdleAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserIdleAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserIdleIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserIdleIxData<'me>(pub &'me UpdateUserIdleIxArgs);
pub const UPDATE_USER_IDLE_IX_DISCM: [u8; 8] = [253, 133, 67, 22, 103, 161, 20, 100];
impl<'me> From<&'me UpdateUserIdleIxArgs> for UpdateUserIdleIxData<'me> {
    fn from(args: &'me UpdateUserIdleIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserIdleIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_IDLE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_idle_ix<K: Into<UpdateUserIdleKeys>, A: Into<UpdateUserIdleIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserIdleKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserIdleIxArgs = args.into();
    let data: UpdateUserIdleIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_idle_invoke<'a, A: Into<UpdateUserIdleIxArgs>>(
    accounts: &UpdateUserIdleAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_idle_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_idle_invoke_signed<'a, A: Into<UpdateUserIdleIxArgs>>(
    accounts: &UpdateUserIdleAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_idle_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_IDLE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserOpenOrdersCountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub filler: &'me AccountInfo<'a2>,
    pub user: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserOpenOrdersCountKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub filler: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&UpdateUserOpenOrdersCountAccounts<'me, '_, '_, '_, '_>>
    for UpdateUserOpenOrdersCountKeys
{
    fn from(accounts: &UpdateUserOpenOrdersCountAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            filler: *accounts.filler.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&UpdateUserOpenOrdersCountKeys>
    for [AccountMeta; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserOpenOrdersCountKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.filler, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&UpdateUserOpenOrdersCountAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateUserOpenOrdersCountAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.filler.clone(),
            accounts.user.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserOpenOrdersCountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserOpenOrdersCountIxData<'me>(pub &'me UpdateUserOpenOrdersCountIxArgs);
pub const UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM: [u8; 8] = [104, 39, 65, 210, 250, 163, 100, 134];
impl<'me> From<&'me UpdateUserOpenOrdersCountIxArgs> for UpdateUserOpenOrdersCountIxData<'me> {
    fn from(args: &'me UpdateUserOpenOrdersCountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserOpenOrdersCountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_OPEN_ORDERS_COUNT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_open_orders_count_ix<
    K: Into<UpdateUserOpenOrdersCountKeys>,
    A: Into<UpdateUserOpenOrdersCountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserOpenOrdersCountKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateUserOpenOrdersCountIxArgs = args.into();
    let data: UpdateUserOpenOrdersCountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_open_orders_count_invoke<'a, A: Into<UpdateUserOpenOrdersCountIxArgs>>(
    accounts: &UpdateUserOpenOrdersCountAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_open_orders_count_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_open_orders_count_invoke_signed<'a, A: Into<UpdateUserOpenOrdersCountIxArgs>>(
    accounts: &UpdateUserOpenOrdersCountAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_open_orders_count_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_OPEN_ORDERS_COUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SETTLE_PNL_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct SettlePnlAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
    pub authority: &'me AccountInfo<'a2>,
    pub spot_market_vault: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct SettlePnlKeys {
    pub state: Pubkey,
    pub user: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
}
impl<'me> From<&SettlePnlAccounts<'me, '_, '_, '_, '_>> for SettlePnlKeys {
    fn from(accounts: &SettlePnlAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
        }
    }
}
impl From<&SettlePnlKeys> for [AccountMeta; SETTLE_PNL_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettlePnlKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new_readonly(keys.spot_market_vault, false),
        ]
    }
}
impl<'a> From<&SettlePnlAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SETTLE_PNL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettlePnlAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.user.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SettlePnlIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct SettlePnlIxData<'me>(pub &'me SettlePnlIxArgs);
pub const SETTLE_PNL_IX_DISCM: [u8; 8] = [43, 61, 234, 45, 15, 95, 152, 153];
impl<'me> From<&'me SettlePnlIxArgs> for SettlePnlIxData<'me> {
    fn from(args: &'me SettlePnlIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SettlePnlIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_PNL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn settle_pnl_ix<K: Into<SettlePnlKeys>, A: Into<SettlePnlIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettlePnlKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_PNL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettlePnlIxArgs = args.into();
    let data: SettlePnlIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_pnl_invoke<'a, A: Into<SettlePnlIxArgs>>(
    accounts: &SettlePnlAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = settle_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_PNL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_pnl_invoke_signed<'a, A: Into<SettlePnlIxArgs>>(
    accounts: &SettlePnlAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_PNL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct SettleFundingPaymentAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleFundingPaymentKeys {
    pub state: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&SettleFundingPaymentAccounts<'me, '_, '_>> for SettleFundingPaymentKeys {
    fn from(accounts: &SettleFundingPaymentAccounts<'me, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&SettleFundingPaymentKeys> for [AccountMeta; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettleFundingPaymentKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&SettleFundingPaymentAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleFundingPaymentAccounts<'_, 'a, 'a>) -> Self {
        [accounts.state.clone(), accounts.user.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SettleFundingPaymentIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SettleFundingPaymentIxData<'me>(pub &'me SettleFundingPaymentIxArgs);
pub const SETTLE_FUNDING_PAYMENT_IX_DISCM: [u8; 8] = [222, 90, 202, 94, 28, 45, 115, 183];
impl<'me> From<&'me SettleFundingPaymentIxArgs> for SettleFundingPaymentIxData<'me> {
    fn from(args: &'me SettleFundingPaymentIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SettleFundingPaymentIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_FUNDING_PAYMENT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn settle_funding_payment_ix<
    K: Into<SettleFundingPaymentKeys>,
    A: Into<SettleFundingPaymentIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleFundingPaymentKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleFundingPaymentIxArgs = args.into();
    let data: SettleFundingPaymentIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_funding_payment_invoke<'a, A: Into<SettleFundingPaymentIxArgs>>(
    accounts: &SettleFundingPaymentAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = settle_funding_payment_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_funding_payment_invoke_signed<'a, A: Into<SettleFundingPaymentIxArgs>>(
    accounts: &SettleFundingPaymentAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_funding_payment_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_FUNDING_PAYMENT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SETTLE_LP_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct SettleLpAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub user: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleLpKeys {
    pub state: Pubkey,
    pub user: Pubkey,
}
impl<'me> From<&SettleLpAccounts<'me, '_, '_>> for SettleLpKeys {
    fn from(accounts: &SettleLpAccounts<'me, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            user: *accounts.user.key,
        }
    }
}
impl From<&SettleLpKeys> for [AccountMeta; SETTLE_LP_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettleLpKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.user, false),
        ]
    }
}
impl<'a> From<&SettleLpAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; SETTLE_LP_IX_ACCOUNTS_LEN] {
    fn from(accounts: &SettleLpAccounts<'_, 'a, 'a>) -> Self {
        [accounts.state.clone(), accounts.user.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SettleLpIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleLpIxData<'me>(pub &'me SettleLpIxArgs);
pub const SETTLE_LP_IX_DISCM: [u8; 8] = [155, 231, 116, 113, 97, 229, 139, 141];
impl<'me> From<&'me SettleLpIxArgs> for SettleLpIxData<'me> {
    fn from(args: &'me SettleLpIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SettleLpIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_LP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn settle_lp_ix<K: Into<SettleLpKeys>, A: Into<SettleLpIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleLpKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_LP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleLpIxArgs = args.into();
    let data: SettleLpIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_lp_invoke<'a, A: Into<SettleLpIxArgs>>(
    accounts: &SettleLpAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = settle_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_lp_invoke_signed<'a, A: Into<SettleLpIxArgs>>(
    accounts: &SettleLpAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&SettleExpiredMarketAccounts<'me, '_, '_>> for SettleExpiredMarketKeys {
    fn from(accounts: &SettleExpiredMarketAccounts<'me, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&SettleExpiredMarketKeys> for [AccountMeta; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &SettleExpiredMarketKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&SettleExpiredMarketAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleExpiredMarketAccounts<'_, 'a, 'a>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SettleExpiredMarketIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketIxData<'me>(pub &'me SettleExpiredMarketIxArgs);
pub const SETTLE_EXPIRED_MARKET_IX_DISCM: [u8; 8] = [120, 89, 11, 25, 122, 77, 72, 193];
impl<'me> From<&'me SettleExpiredMarketIxArgs> for SettleExpiredMarketIxData<'me> {
    fn from(args: &'me SettleExpiredMarketIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SettleExpiredMarketIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_EXPIRED_MARKET_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn settle_expired_market_ix<
    K: Into<SettleExpiredMarketKeys>,
    A: Into<SettleExpiredMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleExpiredMarketKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleExpiredMarketIxArgs = args.into();
    let data: SettleExpiredMarketIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_expired_market_invoke<'a, A: Into<SettleExpiredMarketIxArgs>>(
    accounts: &SettleExpiredMarketAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = settle_expired_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_expired_market_invoke_signed<'a, A: Into<SettleExpiredMarketIxArgs>>(
    accounts: &SettleExpiredMarketAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_expired_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_EXPIRED_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const LIQUIDATE_PERP_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub liquidator: &'me AccountInfo<'a2>,
    pub liquidator_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl<'me> From<&LiquidatePerpAccounts<'me, '_, '_, '_, '_, '_, '_>> for LiquidatePerpKeys {
    fn from(accounts: &LiquidatePerpAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidatePerpKeys> for [AccountMeta; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] {
    fn from(keys: &LiquidatePerpKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl<'a> From<&LiquidatePerpAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidatePerpAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct LiquidatePerpIxArgs {
    pub market_index: u16,
    pub liquidator_max_base_asset_amount: u64,
    pub limit_price: Option<u64>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpIxData<'me>(pub &'me LiquidatePerpIxArgs);
pub const LIQUIDATE_PERP_IX_DISCM: [u8; 8] = [75, 35, 119, 247, 191, 18, 139, 2];
impl<'me> From<&'me LiquidatePerpIxArgs> for LiquidatePerpIxData<'me> {
    fn from(args: &'me LiquidatePerpIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LiquidatePerpIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_PERP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn liquidate_perp_ix<K: Into<LiquidatePerpKeys>, A: Into<LiquidatePerpIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidatePerpKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidatePerpIxArgs = args.into();
    let data: LiquidatePerpIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_perp_invoke<'a, A: Into<LiquidatePerpIxArgs>>(
    accounts: &LiquidatePerpAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_perp_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_perp_invoke_signed<'a, A: Into<LiquidatePerpIxArgs>>(
    accounts: &LiquidatePerpAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_perp_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_PERP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const LIQUIDATE_SPOT_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateSpotAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub liquidator: &'me AccountInfo<'a2>,
    pub liquidator_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidateSpotKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl<'me> From<&LiquidateSpotAccounts<'me, '_, '_, '_, '_, '_, '_>> for LiquidateSpotKeys {
    fn from(accounts: &LiquidateSpotAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidateSpotKeys> for [AccountMeta; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] {
    fn from(keys: &LiquidateSpotKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl<'a> From<&LiquidateSpotAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidateSpotAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct LiquidateSpotIxArgs {
    pub asset_market_index: u16,
    pub liability_market_index: u16,
    pub liquidator_max_liability_transfer: u128,
    pub limit_price: Option<u64>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidateSpotIxData<'me>(pub &'me LiquidateSpotIxArgs);
pub const LIQUIDATE_SPOT_IX_DISCM: [u8; 8] = [107, 0, 128, 41, 35, 229, 251, 18];
impl<'me> From<&'me LiquidateSpotIxArgs> for LiquidateSpotIxData<'me> {
    fn from(args: &'me LiquidateSpotIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LiquidateSpotIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_SPOT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn liquidate_spot_ix<K: Into<LiquidateSpotKeys>, A: Into<LiquidateSpotIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidateSpotKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidateSpotIxArgs = args.into();
    let data: LiquidateSpotIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_spot_invoke<'a, A: Into<LiquidateSpotIxArgs>>(
    accounts: &LiquidateSpotAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_spot_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_spot_invoke_signed<'a, A: Into<LiquidateSpotIxArgs>>(
    accounts: &LiquidateSpotAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_spot_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_SPOT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct LiquidateBorrowForPerpPnlAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub liquidator: &'me AccountInfo<'a2>,
    pub liquidator_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidateBorrowForPerpPnlKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl<'me> From<&LiquidateBorrowForPerpPnlAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for LiquidateBorrowForPerpPnlKeys
{
    fn from(accounts: &LiquidateBorrowForPerpPnlAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidateBorrowForPerpPnlKeys>
    for [AccountMeta; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &LiquidateBorrowForPerpPnlKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl<'a> From<&LiquidateBorrowForPerpPnlAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidateBorrowForPerpPnlAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct LiquidateBorrowForPerpPnlIxArgs {
    pub perp_market_index: u16,
    pub spot_market_index: u16,
    pub liquidator_max_liability_transfer: u128,
    pub limit_price: Option<u64>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidateBorrowForPerpPnlIxData<'me>(pub &'me LiquidateBorrowForPerpPnlIxArgs);
pub const LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM: [u8; 8] = [169, 17, 32, 90, 207, 148, 209, 27];
impl<'me> From<&'me LiquidateBorrowForPerpPnlIxArgs> for LiquidateBorrowForPerpPnlIxData<'me> {
    fn from(args: &'me LiquidateBorrowForPerpPnlIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LiquidateBorrowForPerpPnlIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_BORROW_FOR_PERP_PNL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn liquidate_borrow_for_perp_pnl_ix<
    K: Into<LiquidateBorrowForPerpPnlKeys>,
    A: Into<LiquidateBorrowForPerpPnlIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidateBorrowForPerpPnlKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidateBorrowForPerpPnlIxArgs = args.into();
    let data: LiquidateBorrowForPerpPnlIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_borrow_for_perp_pnl_invoke<'a, A: Into<LiquidateBorrowForPerpPnlIxArgs>>(
    accounts: &LiquidateBorrowForPerpPnlAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_borrow_for_perp_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_borrow_for_perp_pnl_invoke_signed<'a, A: Into<LiquidateBorrowForPerpPnlIxArgs>>(
    accounts: &LiquidateBorrowForPerpPnlAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_borrow_for_perp_pnl_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_BORROW_FOR_PERP_PNL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpPnlForDepositAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub liquidator: &'me AccountInfo<'a2>,
    pub liquidator_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpPnlForDepositKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
}
impl<'me> From<&LiquidatePerpPnlForDepositAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for LiquidatePerpPnlForDepositKeys
{
    fn from(accounts: &LiquidatePerpPnlForDepositAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
        }
    }
}
impl From<&LiquidatePerpPnlForDepositKeys>
    for [AccountMeta; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(keys: &LiquidatePerpPnlForDepositKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
        ]
    }
}
impl<'a> From<&LiquidatePerpPnlForDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &LiquidatePerpPnlForDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct LiquidatePerpPnlForDepositIxArgs {
    pub perp_market_index: u16,
    pub spot_market_index: u16,
    pub liquidator_max_pnl_transfer: u128,
    pub limit_price: Option<u64>,
}
#[derive(Copy, Clone, Debug)]
pub struct LiquidatePerpPnlForDepositIxData<'me>(pub &'me LiquidatePerpPnlForDepositIxArgs);
pub const LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM: [u8; 8] = [237, 75, 198, 235, 233, 186, 75, 35];
impl<'me> From<&'me LiquidatePerpPnlForDepositIxArgs> for LiquidatePerpPnlForDepositIxData<'me> {
    fn from(args: &'me LiquidatePerpPnlForDepositIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for LiquidatePerpPnlForDepositIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn liquidate_perp_pnl_for_deposit_ix<
    K: Into<LiquidatePerpPnlForDepositKeys>,
    A: Into<LiquidatePerpPnlForDepositIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: LiquidatePerpPnlForDepositKeys = accounts.into();
    let metas: [AccountMeta; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: LiquidatePerpPnlForDepositIxArgs = args.into();
    let data: LiquidatePerpPnlForDepositIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn liquidate_perp_pnl_for_deposit_invoke<'a, A: Into<LiquidatePerpPnlForDepositIxArgs>>(
    accounts: &LiquidatePerpPnlForDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = liquidate_perp_pnl_for_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn liquidate_perp_pnl_for_deposit_invoke_signed<
    'a,
    A: Into<LiquidatePerpPnlForDepositIxArgs>,
>(
    accounts: &LiquidatePerpPnlForDepositAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = liquidate_perp_pnl_for_deposit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; LIQUIDATE_PERP_PNL_FOR_DEPOSIT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpPnlDeficitAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub spot_market_vault: &'me AccountInfo<'a2>,
    pub insurance_fund_vault: &'me AccountInfo<'a3>,
    pub drift_signer: &'me AccountInfo<'a4>,
    pub token_program: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpPnlDeficitKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&ResolvePerpPnlDeficitAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for ResolvePerpPnlDeficitKeys
{
    fn from(accounts: &ResolvePerpPnlDeficitAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ResolvePerpPnlDeficitKeys> for [AccountMeta; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] {
    fn from(keys: &ResolvePerpPnlDeficitKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&ResolvePerpPnlDeficitAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ResolvePerpPnlDeficitAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ResolvePerpPnlDeficitIxArgs {
    pub spot_market_index: u16,
    pub perp_market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpPnlDeficitIxData<'me>(pub &'me ResolvePerpPnlDeficitIxArgs);
pub const RESOLVE_PERP_PNL_DEFICIT_IX_DISCM: [u8; 8] = [168, 204, 68, 150, 159, 126, 95, 148];
impl<'me> From<&'me ResolvePerpPnlDeficitIxArgs> for ResolvePerpPnlDeficitIxData<'me> {
    fn from(args: &'me ResolvePerpPnlDeficitIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ResolvePerpPnlDeficitIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&RESOLVE_PERP_PNL_DEFICIT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn resolve_perp_pnl_deficit_ix<
    K: Into<ResolvePerpPnlDeficitKeys>,
    A: Into<ResolvePerpPnlDeficitIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResolvePerpPnlDeficitKeys = accounts.into();
    let metas: [AccountMeta; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResolvePerpPnlDeficitIxArgs = args.into();
    let data: ResolvePerpPnlDeficitIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn resolve_perp_pnl_deficit_invoke<'a, A: Into<ResolvePerpPnlDeficitIxArgs>>(
    accounts: &ResolvePerpPnlDeficitAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = resolve_perp_pnl_deficit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn resolve_perp_pnl_deficit_invoke_signed<'a, A: Into<ResolvePerpPnlDeficitIxArgs>>(
    accounts: &ResolvePerpPnlDeficitAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = resolve_perp_pnl_deficit_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESOLVE_PERP_PNL_DEFICIT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpBankruptcyAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub liquidator: &'me AccountInfo<'a2>,
    pub liquidator_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
    pub spot_market_vault: &'me AccountInfo<'a6>,
    pub insurance_fund_vault: &'me AccountInfo<'a7>,
    pub drift_signer: &'me AccountInfo<'a8>,
    pub token_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpBankruptcyKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&ResolvePerpBankruptcyAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for ResolvePerpBankruptcyKeys
{
    fn from(
        accounts: &ResolvePerpBankruptcyAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ResolvePerpBankruptcyKeys> for [AccountMeta; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ResolvePerpBankruptcyKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&ResolvePerpBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &ResolvePerpBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ResolvePerpBankruptcyIxArgs {
    pub quote_spot_market_index: u16,
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct ResolvePerpBankruptcyIxData<'me>(pub &'me ResolvePerpBankruptcyIxArgs);
pub const RESOLVE_PERP_BANKRUPTCY_IX_DISCM: [u8; 8] = [224, 16, 176, 214, 162, 213, 183, 222];
impl<'me> From<&'me ResolvePerpBankruptcyIxArgs> for ResolvePerpBankruptcyIxData<'me> {
    fn from(args: &'me ResolvePerpBankruptcyIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ResolvePerpBankruptcyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&RESOLVE_PERP_BANKRUPTCY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn resolve_perp_bankruptcy_ix<
    K: Into<ResolvePerpBankruptcyKeys>,
    A: Into<ResolvePerpBankruptcyIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResolvePerpBankruptcyKeys = accounts.into();
    let metas: [AccountMeta; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResolvePerpBankruptcyIxArgs = args.into();
    let data: ResolvePerpBankruptcyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn resolve_perp_bankruptcy_invoke<'a, A: Into<ResolvePerpBankruptcyIxArgs>>(
    accounts: &ResolvePerpBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = resolve_perp_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn resolve_perp_bankruptcy_invoke_signed<'a, A: Into<ResolvePerpBankruptcyIxArgs>>(
    accounts: &ResolvePerpBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = resolve_perp_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESOLVE_PERP_BANKRUPTCY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct ResolveSpotBankruptcyAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
    pub liquidator: &'me AccountInfo<'a2>,
    pub liquidator_stats: &'me AccountInfo<'a3>,
    pub user: &'me AccountInfo<'a4>,
    pub user_stats: &'me AccountInfo<'a5>,
    pub spot_market_vault: &'me AccountInfo<'a6>,
    pub insurance_fund_vault: &'me AccountInfo<'a7>,
    pub drift_signer: &'me AccountInfo<'a8>,
    pub token_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct ResolveSpotBankruptcyKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
    pub liquidator: Pubkey,
    pub liquidator_stats: Pubkey,
    pub user: Pubkey,
    pub user_stats: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&ResolveSpotBankruptcyAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for ResolveSpotBankruptcyKeys
{
    fn from(
        accounts: &ResolveSpotBankruptcyAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            liquidator: *accounts.liquidator.key,
            liquidator_stats: *accounts.liquidator_stats.key,
            user: *accounts.user.key,
            user_stats: *accounts.user_stats.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ResolveSpotBankruptcyKeys> for [AccountMeta; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ResolveSpotBankruptcyKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.liquidator, false),
            AccountMeta::new(keys.liquidator_stats, false),
            AccountMeta::new(keys.user, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&ResolveSpotBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &ResolveSpotBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.liquidator.clone(),
            accounts.liquidator_stats.clone(),
            accounts.user.clone(),
            accounts.user_stats.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ResolveSpotBankruptcyIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct ResolveSpotBankruptcyIxData<'me>(pub &'me ResolveSpotBankruptcyIxArgs);
pub const RESOLVE_SPOT_BANKRUPTCY_IX_DISCM: [u8; 8] = [124, 194, 240, 254, 198, 213, 52, 122];
impl<'me> From<&'me ResolveSpotBankruptcyIxArgs> for ResolveSpotBankruptcyIxData<'me> {
    fn from(args: &'me ResolveSpotBankruptcyIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ResolveSpotBankruptcyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&RESOLVE_SPOT_BANKRUPTCY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn resolve_spot_bankruptcy_ix<
    K: Into<ResolveSpotBankruptcyKeys>,
    A: Into<ResolveSpotBankruptcyIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResolveSpotBankruptcyKeys = accounts.into();
    let metas: [AccountMeta; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResolveSpotBankruptcyIxArgs = args.into();
    let data: ResolveSpotBankruptcyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn resolve_spot_bankruptcy_invoke<'a, A: Into<ResolveSpotBankruptcyIxArgs>>(
    accounts: &ResolveSpotBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = resolve_spot_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn resolve_spot_bankruptcy_invoke_signed<'a, A: Into<ResolveSpotBankruptcyIxArgs>>(
    accounts: &ResolveSpotBankruptcyAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = resolve_spot_bankruptcy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESOLVE_SPOT_BANKRUPTCY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct SettleRevenueToInsuranceFundAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub spot_market: &'me AccountInfo<'a1>,
    pub spot_market_vault: &'me AccountInfo<'a2>,
    pub drift_signer: &'me AccountInfo<'a3>,
    pub insurance_fund_vault: &'me AccountInfo<'a4>,
    pub token_program: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleRevenueToInsuranceFundKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&SettleRevenueToInsuranceFundAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for SettleRevenueToInsuranceFundKeys
{
    fn from(accounts: &SettleRevenueToInsuranceFundAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            drift_signer: *accounts.drift_signer.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&SettleRevenueToInsuranceFundKeys>
    for [AccountMeta; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SettleRevenueToInsuranceFundKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&SettleRevenueToInsuranceFundAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleRevenueToInsuranceFundAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.spot_market_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SettleRevenueToInsuranceFundIxArgs {
    pub spot_market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleRevenueToInsuranceFundIxData<'me>(pub &'me SettleRevenueToInsuranceFundIxArgs);
pub const SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM: [u8; 8] =
    [200, 120, 93, 136, 69, 38, 199, 159];
impl<'me> From<&'me SettleRevenueToInsuranceFundIxArgs>
    for SettleRevenueToInsuranceFundIxData<'me>
{
    fn from(args: &'me SettleRevenueToInsuranceFundIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SettleRevenueToInsuranceFundIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_REVENUE_TO_INSURANCE_FUND_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn settle_revenue_to_insurance_fund_ix<
    K: Into<SettleRevenueToInsuranceFundKeys>,
    A: Into<SettleRevenueToInsuranceFundIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleRevenueToInsuranceFundKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SettleRevenueToInsuranceFundIxArgs = args.into();
    let data: SettleRevenueToInsuranceFundIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_revenue_to_insurance_fund_invoke<'a, A: Into<SettleRevenueToInsuranceFundIxArgs>>(
    accounts: &SettleRevenueToInsuranceFundAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = settle_revenue_to_insurance_fund_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_revenue_to_insurance_fund_invoke_signed<
    'a,
    A: Into<SettleRevenueToInsuranceFundIxArgs>,
>(
    accounts: &SettleRevenueToInsuranceFundAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_revenue_to_insurance_fund_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SETTLE_REVENUE_TO_INSURANCE_FUND_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateFundingRateAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub perp_market: &'me AccountInfo<'a1>,
    pub oracle: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateFundingRateKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}
impl<'me> From<&UpdateFundingRateAccounts<'me, '_, '_, '_>> for UpdateFundingRateKeys {
    fn from(accounts: &UpdateFundingRateAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateFundingRateKeys> for [AccountMeta; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateFundingRateKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl<'a> From<&UpdateFundingRateAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateFundingRateAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateFundingRateIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateFundingRateIxData<'me>(pub &'me UpdateFundingRateIxArgs);
pub const UPDATE_FUNDING_RATE_IX_DISCM: [u8; 8] = [201, 178, 116, 212, 166, 144, 72, 238];
impl<'me> From<&'me UpdateFundingRateIxArgs> for UpdateFundingRateIxData<'me> {
    fn from(args: &'me UpdateFundingRateIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateFundingRateIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_FUNDING_RATE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_funding_rate_ix<K: Into<UpdateFundingRateKeys>, A: Into<UpdateFundingRateIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateFundingRateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateFundingRateIxArgs = args.into();
    let data: UpdateFundingRateIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_funding_rate_invoke<'a, A: Into<UpdateFundingRateIxArgs>>(
    accounts: &UpdateFundingRateAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_funding_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_funding_rate_invoke_signed<'a, A: Into<UpdateFundingRateIxArgs>>(
    accounts: &UpdateFundingRateAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_funding_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_FUNDING_RATE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketCumulativeInterestAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub spot_market: &'me AccountInfo<'a1>,
    pub oracle: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketCumulativeInterestKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
}
impl<'me> From<&UpdateSpotMarketCumulativeInterestAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketCumulativeInterestKeys
{
    fn from(accounts: &UpdateSpotMarketCumulativeInterestAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateSpotMarketCumulativeInterestKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketCumulativeInterestKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketCumulativeInterestAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketCumulativeInterestIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketCumulativeInterestIxData<'me>(
    pub &'me UpdateSpotMarketCumulativeInterestIxArgs,
);
pub const UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM: [u8; 8] =
    [39, 166, 139, 243, 158, 165, 155, 225];
impl<'me> From<&'me UpdateSpotMarketCumulativeInterestIxArgs>
    for UpdateSpotMarketCumulativeInterestIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketCumulativeInterestIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketCumulativeInterestIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_cumulative_interest_ix<
    K: Into<UpdateSpotMarketCumulativeInterestKeys>,
    A: Into<UpdateSpotMarketCumulativeInterestIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketCumulativeInterestKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketCumulativeInterestIxArgs = args.into();
    let data: UpdateSpotMarketCumulativeInterestIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_cumulative_interest_invoke<
    'a,
    A: Into<UpdateSpotMarketCumulativeInterestIxArgs>,
>(
    accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_cumulative_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_cumulative_interest_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketCumulativeInterestIxArgs>,
>(
    accounts: &UpdateSpotMarketCumulativeInterestAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_cumulative_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_CUMULATIVE_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_AMMS_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmsAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmsKeys {
    pub state: Pubkey,
    pub authority: Pubkey,
}
impl<'me> From<&UpdateAmmsAccounts<'me, '_, '_>> for UpdateAmmsKeys {
    fn from(accounts: &UpdateAmmsAccounts<'me, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<&UpdateAmmsKeys> for [AccountMeta; UPDATE_AMMS_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateAmmsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
        ]
    }
}
impl<'a> From<&UpdateAmmsAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; UPDATE_AMMS_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UpdateAmmsAccounts<'_, 'a, 'a>) -> Self {
        [accounts.state.clone(), accounts.authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateAmmsIxArgs {
    pub market_indexes: [u16; 5],
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmsIxData<'me>(pub &'me UpdateAmmsIxArgs);
pub const UPDATE_AMMS_IX_DISCM: [u8; 8] = [201, 106, 217, 253, 4, 175, 228, 97];
impl<'me> From<&'me UpdateAmmsIxArgs> for UpdateAmmsIxData<'me> {
    fn from(args: &'me UpdateAmmsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateAmmsIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AMMS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_amms_ix<K: Into<UpdateAmmsKeys>, A: Into<UpdateAmmsIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateAmmsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_AMMS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateAmmsIxArgs = args.into();
    let data: UpdateAmmsIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_amms_invoke<'a, A: Into<UpdateAmmsIxArgs>>(
    accounts: &UpdateAmmsAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_amms_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_AMMS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_amms_invoke_signed<'a, A: Into<UpdateAmmsIxArgs>>(
    accounts: &UpdateAmmsAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_amms_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_AMMS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketExpiryAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketExpiryKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketExpiryAccounts<'me, '_, '_, '_>> for UpdateSpotMarketExpiryKeys {
    fn from(accounts: &UpdateSpotMarketExpiryAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketExpiryKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketExpiryKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketExpiryAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketExpiryAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketExpiryIxArgs {
    pub expiry_ts: i64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketExpiryIxData<'me>(pub &'me UpdateSpotMarketExpiryIxArgs);
pub const UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM: [u8; 8] = [208, 11, 211, 159, 226, 24, 11, 247];
impl<'me> From<&'me UpdateSpotMarketExpiryIxArgs> for UpdateSpotMarketExpiryIxData<'me> {
    fn from(args: &'me UpdateSpotMarketExpiryIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketExpiryIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_EXPIRY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_expiry_ix<
    K: Into<UpdateSpotMarketExpiryKeys>,
    A: Into<UpdateSpotMarketExpiryIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketExpiryKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketExpiryIxArgs = args.into();
    let data: UpdateSpotMarketExpiryIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_expiry_invoke<'a, A: Into<UpdateSpotMarketExpiryIxArgs>>(
    accounts: &UpdateSpotMarketExpiryAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_expiry_invoke_signed<'a, A: Into<UpdateSpotMarketExpiryIxArgs>>(
    accounts: &UpdateSpotMarketExpiryAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserQuoteAssetInsuranceStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub spot_market: &'me AccountInfo<'a1>,
    pub insurance_fund_stake: &'me AccountInfo<'a2>,
    pub user_stats: &'me AccountInfo<'a3>,
    pub authority: &'me AccountInfo<'a4>,
    pub insurance_fund_vault: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserQuoteAssetInsuranceStakeKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}
impl<'me> From<&UpdateUserQuoteAssetInsuranceStakeAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for UpdateUserQuoteAssetInsuranceStakeKeys
{
    fn from(
        accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'me, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
        }
    }
}
impl From<&UpdateUserQuoteAssetInsuranceStakeKeys>
    for [AccountMeta; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateUserQuoteAssetInsuranceStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
        ]
    }
}
impl<'a> From<&UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateUserQuoteAssetInsuranceStakeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdateUserQuoteAssetInsuranceStakeIxData<'me>(
    pub &'me UpdateUserQuoteAssetInsuranceStakeIxArgs,
);
pub const UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM: [u8; 8] =
    [251, 101, 156, 7, 2, 63, 30, 23];
impl<'me> From<&'me UpdateUserQuoteAssetInsuranceStakeIxArgs>
    for UpdateUserQuoteAssetInsuranceStakeIxData<'me>
{
    fn from(args: &'me UpdateUserQuoteAssetInsuranceStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateUserQuoteAssetInsuranceStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_user_quote_asset_insurance_stake_ix<
    K: Into<UpdateUserQuoteAssetInsuranceStakeKeys>,
    A: Into<UpdateUserQuoteAssetInsuranceStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateUserQuoteAssetInsuranceStakeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateUserQuoteAssetInsuranceStakeIxArgs = args.into();
    let data: UpdateUserQuoteAssetInsuranceStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_user_quote_asset_insurance_stake_invoke<
    'a,
    A: Into<UpdateUserQuoteAssetInsuranceStakeIxArgs>,
>(
    accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_user_quote_asset_insurance_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_user_quote_asset_insurance_stake_invoke_signed<
    'a,
    A: Into<UpdateUserQuoteAssetInsuranceStakeIxArgs>,
>(
    accounts: &UpdateUserQuoteAssetInsuranceStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_user_quote_asset_insurance_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_USER_QUOTE_ASSET_INSURANCE_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 8usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeInsuranceFundStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
> {
    pub spot_market: &'me AccountInfo<'a0>,
    pub insurance_fund_stake: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub state: &'me AccountInfo<'a3>,
    pub authority: &'me AccountInfo<'a4>,
    pub payer: &'me AccountInfo<'a5>,
    pub rent: &'me AccountInfo<'a6>,
    pub system_program: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeInsuranceFundStakeKeys {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub state: Pubkey,
    pub authority: Pubkey,
    pub payer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&InitializeInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for InitializeInsuranceFundStakeKeys
{
    fn from(
        accounts: &InitializeInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            state: *accounts.state.key,
            authority: *accounts.authority.key,
            payer: *accounts.payer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeInsuranceFundStakeKeys>
    for [AccountMeta; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &InitializeInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.payer, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&InitializeInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &InitializeInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.state.clone(),
            accounts.authority.clone(),
            accounts.payer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeInsuranceFundStakeIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeInsuranceFundStakeIxData<'me>(pub &'me InitializeInsuranceFundStakeIxArgs);
pub const INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] = [187, 179, 243, 70, 248, 90, 92, 147];
impl<'me> From<&'me InitializeInsuranceFundStakeIxArgs>
    for InitializeInsuranceFundStakeIxData<'me>
{
    fn from(args: &'me InitializeInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeInsuranceFundStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_insurance_fund_stake_ix<
    K: Into<InitializeInsuranceFundStakeKeys>,
    A: Into<InitializeInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeInsuranceFundStakeIxArgs = args.into();
    let data: InitializeInsuranceFundStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_insurance_fund_stake_invoke<'a, A: Into<InitializeInsuranceFundStakeIxArgs>>(
    accounts: &InitializeInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_insurance_fund_stake_invoke_signed<
    'a,
    A: Into<InitializeInsuranceFundStakeIxArgs>,
>(
    accounts: &InitializeInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct AddInsuranceFundStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub spot_market: &'me AccountInfo<'a1>,
    pub insurance_fund_stake: &'me AccountInfo<'a2>,
    pub user_stats: &'me AccountInfo<'a3>,
    pub authority: &'me AccountInfo<'a4>,
    pub spot_market_vault: &'me AccountInfo<'a5>,
    pub insurance_fund_vault: &'me AccountInfo<'a6>,
    pub drift_signer: &'me AccountInfo<'a7>,
    pub user_token_account: &'me AccountInfo<'a8>,
    pub token_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct AddInsuranceFundStakeKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&AddInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for AddInsuranceFundStakeKeys
{
    fn from(
        accounts: &AddInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&AddInsuranceFundStakeKeys> for [AccountMeta; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: &AddInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&AddInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &AddInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct AddInsuranceFundStakeIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct AddInsuranceFundStakeIxData<'me>(pub &'me AddInsuranceFundStakeIxArgs);
pub const ADD_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] = [251, 144, 115, 11, 222, 47, 62, 236];
impl<'me> From<&'me AddInsuranceFundStakeIxArgs> for AddInsuranceFundStakeIxData<'me> {
    fn from(args: &'me AddInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AddInsuranceFundStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADD_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn add_insurance_fund_stake_ix<
    K: Into<AddInsuranceFundStakeKeys>,
    A: Into<AddInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AddInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AddInsuranceFundStakeIxArgs = args.into();
    let data: AddInsuranceFundStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn add_insurance_fund_stake_invoke<'a, A: Into<AddInsuranceFundStakeIxArgs>>(
    accounts: &AddInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = add_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn add_insurance_fund_stake_invoke_signed<'a, A: Into<AddInsuranceFundStakeIxArgs>>(
    accounts: &AddInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = add_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADD_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 5usize;
#[derive(Copy, Clone, Debug)]
pub struct RequestRemoveInsuranceFundStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
> {
    pub spot_market: &'me AccountInfo<'a0>,
    pub insurance_fund_stake: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub insurance_fund_vault: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct RequestRemoveInsuranceFundStakeKeys {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}
impl<'me> From<&RequestRemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_>>
    for RequestRemoveInsuranceFundStakeKeys
{
    fn from(accounts: &RequestRemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
        }
    }
}
impl From<&RequestRemoveInsuranceFundStakeKeys>
    for [AccountMeta; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RequestRemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
        ]
    }
}
impl<'a> From<&RequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RequestRemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct RequestRemoveInsuranceFundStakeIxData<'me>(
    pub &'me RequestRemoveInsuranceFundStakeIxArgs,
);
pub const REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] =
    [142, 70, 204, 92, 73, 106, 180, 52];
impl<'me> From<&'me RequestRemoveInsuranceFundStakeIxArgs>
    for RequestRemoveInsuranceFundStakeIxData<'me>
{
    fn from(args: &'me RequestRemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RequestRemoveInsuranceFundStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn request_remove_insurance_fund_stake_ix<
    K: Into<RequestRemoveInsuranceFundStakeKeys>,
    A: Into<RequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RequestRemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RequestRemoveInsuranceFundStakeIxArgs = args.into();
    let data: RequestRemoveInsuranceFundStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_remove_insurance_fund_stake_invoke<
    'a,
    A: Into<RequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn request_remove_insurance_fund_stake_invoke_signed<
    'a,
    A: Into<RequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &RequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 5usize;
#[derive(Copy, Clone, Debug)]
pub struct CancelRequestRemoveInsuranceFundStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
> {
    pub spot_market: &'me AccountInfo<'a0>,
    pub insurance_fund_stake: &'me AccountInfo<'a1>,
    pub user_stats: &'me AccountInfo<'a2>,
    pub authority: &'me AccountInfo<'a3>,
    pub insurance_fund_vault: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelRequestRemoveInsuranceFundStakeKeys {
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
}
impl<'me> From<&CancelRequestRemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_>>
    for CancelRequestRemoveInsuranceFundStakeKeys
{
    fn from(
        accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
        }
    }
}
impl From<&CancelRequestRemoveInsuranceFundStakeKeys>
    for [AccountMeta; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CancelRequestRemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
        ]
    }
}
impl<'a> From<&CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CancelRequestRemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct CancelRequestRemoveInsuranceFundStakeIxData<'me>(
    pub &'me CancelRequestRemoveInsuranceFundStakeIxArgs,
);
pub const CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] =
    [97, 235, 78, 62, 212, 42, 241, 127];
impl<'me> From<&'me CancelRequestRemoveInsuranceFundStakeIxArgs>
    for CancelRequestRemoveInsuranceFundStakeIxData<'me>
{
    fn from(args: &'me CancelRequestRemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CancelRequestRemoveInsuranceFundStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn cancel_request_remove_insurance_fund_stake_ix<
    K: Into<CancelRequestRemoveInsuranceFundStakeKeys>,
    A: Into<CancelRequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CancelRequestRemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: CancelRequestRemoveInsuranceFundStakeIxArgs = args.into();
    let data: CancelRequestRemoveInsuranceFundStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn cancel_request_remove_insurance_fund_stake_invoke<
    'a,
    A: Into<CancelRequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = cancel_request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn cancel_request_remove_insurance_fund_stake_invoke_signed<
    'a,
    A: Into<CancelRequestRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &CancelRequestRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = cancel_request_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        CANCEL_REQUEST_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 9usize;
#[derive(Copy, Clone, Debug)]
pub struct RemoveInsuranceFundStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub spot_market: &'me AccountInfo<'a1>,
    pub insurance_fund_stake: &'me AccountInfo<'a2>,
    pub user_stats: &'me AccountInfo<'a3>,
    pub authority: &'me AccountInfo<'a4>,
    pub insurance_fund_vault: &'me AccountInfo<'a5>,
    pub drift_signer: &'me AccountInfo<'a6>,
    pub user_token_account: &'me AccountInfo<'a7>,
    pub token_program: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveInsuranceFundStakeKeys {
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_stake: Pubkey,
    pub user_stats: Pubkey,
    pub authority: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub user_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&RemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for RemoveInsuranceFundStakeKeys
{
    fn from(
        accounts: &RemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_stake: *accounts.insurance_fund_stake.key,
            user_stats: *accounts.user_stats.key,
            authority: *accounts.authority.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            user_token_account: *accounts.user_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&RemoveInsuranceFundStakeKeys>
    for [AccountMeta; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_stake, false),
            AccountMeta::new(keys.user_stats, false),
            AccountMeta::new_readonly(keys.authority, true),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.user_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&RemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &RemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_stake.clone(),
            accounts.user_stats.clone(),
            accounts.authority.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.user_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveInsuranceFundStakeIxData<'me>(pub &'me RemoveInsuranceFundStakeIxArgs);
pub const REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] = [128, 166, 142, 9, 254, 187, 143, 174];
impl<'me> From<&'me RemoveInsuranceFundStakeIxArgs> for RemoveInsuranceFundStakeIxData<'me> {
    fn from(args: &'me RemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveInsuranceFundStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn remove_insurance_fund_stake_ix<
    K: Into<RemoveInsuranceFundStakeKeys>,
    A: Into<RemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveInsuranceFundStakeIxArgs = args.into();
    let data: RemoveInsuranceFundStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_insurance_fund_stake_invoke<'a, A: Into<RemoveInsuranceFundStakeIxArgs>>(
    accounts: &RemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_insurance_fund_stake_invoke_signed<'a, A: Into<RemoveInsuranceFundStakeIxArgs>>(
    accounts: &RemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 7usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub quote_asset_mint: &'me AccountInfo<'a2>,
    pub drift_signer: &'me AccountInfo<'a3>,
    pub rent: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
    pub token_program: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub quote_asset_mint: Pubkey,
    pub drift_signer: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&InitializeAccounts<'me, '_, '_, '_, '_, '_, '_, '_>> for InitializeKeys {
    fn from(accounts: &InitializeAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            quote_asset_mint: *accounts.quote_asset_mint.key,
            drift_signer: *accounts.drift_signer.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeKeys) -> Self {
        [
            AccountMeta::new(keys.admin, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.quote_asset_mint, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&InitializeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.quote_asset_mint.clone(),
            accounts.drift_signer.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct InitializeIxData<'me>(pub &'me InitializeIxArgs);
pub const INITIALIZE_IX_DISCM: [u8; 8] = [175, 175, 109, 31, 13, 152, 155, 237];
impl<'me> From<&'me InitializeIxArgs> for InitializeIxData<'me> {
    fn from(args: &'me InitializeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_ix<K: Into<InitializeKeys>, A: Into<InitializeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeIxArgs = args.into();
    let data: InitializeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_invoke<'a, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_invoke_signed<'a, A: Into<InitializeIxArgs>>(
    accounts: &InitializeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN: usize = 11usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeSpotMarketAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
> {
    pub spot_market: &'me AccountInfo<'a0>,
    pub spot_market_mint: &'me AccountInfo<'a1>,
    pub spot_market_vault: &'me AccountInfo<'a2>,
    pub insurance_fund_vault: &'me AccountInfo<'a3>,
    pub drift_signer: &'me AccountInfo<'a4>,
    pub state: &'me AccountInfo<'a5>,
    pub oracle: &'me AccountInfo<'a6>,
    pub admin: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
    pub token_program: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeSpotMarketKeys {
    pub spot_market: Pubkey,
    pub spot_market_mint: Pubkey,
    pub spot_market_vault: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub state: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&InitializeSpotMarketAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for InitializeSpotMarketKeys
{
    fn from(
        accounts: &InitializeSpotMarketAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            spot_market: *accounts.spot_market.key,
            spot_market_mint: *accounts.spot_market_mint.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            state: *accounts.state.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&InitializeSpotMarketKeys> for [AccountMeta; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializeSpotMarketKeys) -> Self {
        [
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new_readonly(keys.spot_market_mint, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&InitializeSpotMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &InitializeSpotMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.spot_market.clone(),
            accounts.spot_market_mint.clone(),
            accounts.spot_market_vault.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.state.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeSpotMarketIxArgs {
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
    pub oracle_source: OracleSource,
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
    pub liquidator_fee: u32,
    pub active_status: bool,
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeSpotMarketIxData<'me>(pub &'me InitializeSpotMarketIxArgs);
pub const INITIALIZE_SPOT_MARKET_IX_DISCM: [u8; 8] = [234, 196, 128, 44, 94, 15, 48, 201];
impl<'me> From<&'me InitializeSpotMarketIxArgs> for InitializeSpotMarketIxData<'me> {
    fn from(args: &'me InitializeSpotMarketIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeSpotMarketIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_SPOT_MARKET_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_spot_market_ix<
    K: Into<InitializeSpotMarketKeys>,
    A: Into<InitializeSpotMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeSpotMarketKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeSpotMarketIxArgs = args.into();
    let data: InitializeSpotMarketIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_spot_market_invoke<'a, A: Into<InitializeSpotMarketIxArgs>>(
    accounts: &InitializeSpotMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_spot_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_spot_market_invoke_signed<'a, A: Into<InitializeSpotMarketIxArgs>>(
    accounts: &InitializeSpotMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_spot_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_SPOT_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN: usize = 11usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializeSerumFulfillmentConfigAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
    'a10: 'me,
> {
    pub base_spot_market: &'me AccountInfo<'a0>,
    pub quote_spot_market: &'me AccountInfo<'a1>,
    pub state: &'me AccountInfo<'a2>,
    pub serum_program: &'me AccountInfo<'a3>,
    pub serum_market: &'me AccountInfo<'a4>,
    pub serum_open_orders: &'me AccountInfo<'a5>,
    pub drift_signer: &'me AccountInfo<'a6>,
    pub serum_fulfillment_config: &'me AccountInfo<'a7>,
    pub admin: &'me AccountInfo<'a8>,
    pub rent: &'me AccountInfo<'a9>,
    pub system_program: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeSerumFulfillmentConfigKeys {
    pub base_spot_market: Pubkey,
    pub quote_spot_market: Pubkey,
    pub state: Pubkey,
    pub serum_program: Pubkey,
    pub serum_market: Pubkey,
    pub serum_open_orders: Pubkey,
    pub drift_signer: Pubkey,
    pub serum_fulfillment_config: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me>
    From<&InitializeSerumFulfillmentConfigAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for InitializeSerumFulfillmentConfigKeys
{
    fn from(
        accounts: &InitializeSerumFulfillmentConfigAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            base_spot_market: *accounts.base_spot_market.key,
            quote_spot_market: *accounts.quote_spot_market.key,
            state: *accounts.state.key,
            serum_program: *accounts.serum_program.key,
            serum_market: *accounts.serum_market.key,
            serum_open_orders: *accounts.serum_open_orders.key,
            drift_signer: *accounts.drift_signer.key,
            serum_fulfillment_config: *accounts.serum_fulfillment_config.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializeSerumFulfillmentConfigKeys>
    for [AccountMeta; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(keys: &InitializeSerumFulfillmentConfigKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.base_spot_market, false),
            AccountMeta::new_readonly(keys.quote_spot_market, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.serum_program, false),
            AccountMeta::new_readonly(keys.serum_market, false),
            AccountMeta::new(keys.serum_open_orders, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.serum_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a>
    From<&InitializeSerumFulfillmentConfigAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &InitializeSerumFulfillmentConfigAccounts<
            '_,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.base_spot_market.clone(),
            accounts.quote_spot_market.clone(),
            accounts.state.clone(),
            accounts.serum_program.clone(),
            accounts.serum_market.clone(),
            accounts.serum_open_orders.clone(),
            accounts.drift_signer.clone(),
            accounts.serum_fulfillment_config.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializeSerumFulfillmentConfigIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeSerumFulfillmentConfigIxData<'me>(
    pub &'me InitializeSerumFulfillmentConfigIxArgs,
);
pub const INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM: [u8; 8] =
    [193, 211, 132, 172, 70, 171, 7, 94];
impl<'me> From<&'me InitializeSerumFulfillmentConfigIxArgs>
    for InitializeSerumFulfillmentConfigIxData<'me>
{
    fn from(args: &'me InitializeSerumFulfillmentConfigIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializeSerumFulfillmentConfigIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_serum_fulfillment_config_ix<
    K: Into<InitializeSerumFulfillmentConfigKeys>,
    A: Into<InitializeSerumFulfillmentConfigIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializeSerumFulfillmentConfigKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializeSerumFulfillmentConfigIxArgs = args.into();
    let data: InitializeSerumFulfillmentConfigIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_serum_fulfillment_config_invoke<
    'a,
    A: Into<InitializeSerumFulfillmentConfigIxArgs>,
>(
    accounts: &InitializeSerumFulfillmentConfigAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = initialize_serum_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_serum_fulfillment_config_invoke_signed<
    'a,
    A: Into<InitializeSerumFulfillmentConfigIxArgs>,
>(
    accounts: &InitializeSerumFulfillmentConfigAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_serum_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_SERUM_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumFulfillmentConfigStatusAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub serum_fulfillment_config: &'me AccountInfo<'a1>,
    pub admin: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumFulfillmentConfigStatusKeys {
    pub state: Pubkey,
    pub serum_fulfillment_config: Pubkey,
    pub admin: Pubkey,
}
impl<'me> From<&UpdateSerumFulfillmentConfigStatusAccounts<'me, '_, '_, '_>>
    for UpdateSerumFulfillmentConfigStatusKeys
{
    fn from(accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            serum_fulfillment_config: *accounts.serum_fulfillment_config.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&UpdateSerumFulfillmentConfigStatusKeys>
    for [AccountMeta; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSerumFulfillmentConfigStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.serum_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
        ]
    }
}
impl<'a> From<&UpdateSerumFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.serum_fulfillment_config.clone(),
            accounts.admin.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSerumFulfillmentConfigStatusIxArgs {
    pub status: SpotFulfillmentConfigStatus,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumFulfillmentConfigStatusIxData<'me>(
    pub &'me UpdateSerumFulfillmentConfigStatusIxArgs,
);
pub const UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM: [u8; 8] =
    [171, 109, 240, 251, 95, 1, 149, 89];
impl<'me> From<&'me UpdateSerumFulfillmentConfigStatusIxArgs>
    for UpdateSerumFulfillmentConfigStatusIxData<'me>
{
    fn from(args: &'me UpdateSerumFulfillmentConfigStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSerumFulfillmentConfigStatusIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_serum_fulfillment_config_status_ix<
    K: Into<UpdateSerumFulfillmentConfigStatusKeys>,
    A: Into<UpdateSerumFulfillmentConfigStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSerumFulfillmentConfigStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSerumFulfillmentConfigStatusIxArgs = args.into();
    let data: UpdateSerumFulfillmentConfigStatusIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_serum_fulfillment_config_status_invoke<
    'a,
    A: Into<UpdateSerumFulfillmentConfigStatusIxArgs>,
>(
    accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_serum_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_serum_fulfillment_config_status_invoke_signed<
    'a,
    A: Into<UpdateSerumFulfillmentConfigStatusIxArgs>,
>(
    accounts: &UpdateSerumFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_serum_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SERUM_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN: usize = 10usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializePhoenixFulfillmentConfigAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
    'a8: 'me,
    'a9: 'me,
> {
    pub base_spot_market: &'me AccountInfo<'a0>,
    pub quote_spot_market: &'me AccountInfo<'a1>,
    pub state: &'me AccountInfo<'a2>,
    pub phoenix_program: &'me AccountInfo<'a3>,
    pub phoenix_market: &'me AccountInfo<'a4>,
    pub drift_signer: &'me AccountInfo<'a5>,
    pub phoenix_fulfillment_config: &'me AccountInfo<'a6>,
    pub admin: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializePhoenixFulfillmentConfigKeys {
    pub base_spot_market: Pubkey,
    pub quote_spot_market: Pubkey,
    pub state: Pubkey,
    pub phoenix_program: Pubkey,
    pub phoenix_market: Pubkey,
    pub drift_signer: Pubkey,
    pub phoenix_fulfillment_config: Pubkey,
    pub admin: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me>
    From<&InitializePhoenixFulfillmentConfigAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for InitializePhoenixFulfillmentConfigKeys
{
    fn from(
        accounts: &InitializePhoenixFulfillmentConfigAccounts<
            'me,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            base_spot_market: *accounts.base_spot_market.key,
            quote_spot_market: *accounts.quote_spot_market.key,
            state: *accounts.state.key,
            phoenix_program: *accounts.phoenix_program.key,
            phoenix_market: *accounts.phoenix_market.key,
            drift_signer: *accounts.drift_signer.key,
            phoenix_fulfillment_config: *accounts.phoenix_fulfillment_config.key,
            admin: *accounts.admin.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializePhoenixFulfillmentConfigKeys>
    for [AccountMeta; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(keys: &InitializePhoenixFulfillmentConfigKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.base_spot_market, false),
            AccountMeta::new_readonly(keys.quote_spot_market, false),
            AccountMeta::new(keys.state, false),
            AccountMeta::new_readonly(keys.phoenix_program, false),
            AccountMeta::new_readonly(keys.phoenix_market, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.phoenix_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a>
    From<&InitializePhoenixFulfillmentConfigAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &InitializePhoenixFulfillmentConfigAccounts<
            '_,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.base_spot_market.clone(),
            accounts.quote_spot_market.clone(),
            accounts.state.clone(),
            accounts.phoenix_program.clone(),
            accounts.phoenix_market.clone(),
            accounts.drift_signer.clone(),
            accounts.phoenix_fulfillment_config.clone(),
            accounts.admin.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializePhoenixFulfillmentConfigIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializePhoenixFulfillmentConfigIxData<'me>(
    pub &'me InitializePhoenixFulfillmentConfigIxArgs,
);
pub const INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM: [u8; 8] =
    [135, 132, 110, 107, 185, 160, 169, 154];
impl<'me> From<&'me InitializePhoenixFulfillmentConfigIxArgs>
    for InitializePhoenixFulfillmentConfigIxData<'me>
{
    fn from(args: &'me InitializePhoenixFulfillmentConfigIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializePhoenixFulfillmentConfigIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_phoenix_fulfillment_config_ix<
    K: Into<InitializePhoenixFulfillmentConfigKeys>,
    A: Into<InitializePhoenixFulfillmentConfigIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializePhoenixFulfillmentConfigKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: InitializePhoenixFulfillmentConfigIxArgs = args.into();
    let data: InitializePhoenixFulfillmentConfigIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_phoenix_fulfillment_config_invoke<
    'a,
    A: Into<InitializePhoenixFulfillmentConfigIxArgs>,
>(
    accounts: &InitializePhoenixFulfillmentConfigAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
) -> ProgramResult {
    let ix = initialize_phoenix_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_phoenix_fulfillment_config_invoke_signed<
    'a,
    A: Into<InitializePhoenixFulfillmentConfigIxArgs>,
>(
    accounts: &InitializePhoenixFulfillmentConfigAccounts<
        '_,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_phoenix_fulfillment_config_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_PHOENIX_FULFILLMENT_CONFIG_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct PhoenixFulfillmentConfigStatusAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub phoenix_fulfillment_config: &'me AccountInfo<'a1>,
    pub admin: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct PhoenixFulfillmentConfigStatusKeys {
    pub state: Pubkey,
    pub phoenix_fulfillment_config: Pubkey,
    pub admin: Pubkey,
}
impl<'me> From<&PhoenixFulfillmentConfigStatusAccounts<'me, '_, '_, '_>>
    for PhoenixFulfillmentConfigStatusKeys
{
    fn from(accounts: &PhoenixFulfillmentConfigStatusAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            phoenix_fulfillment_config: *accounts.phoenix_fulfillment_config.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&PhoenixFulfillmentConfigStatusKeys>
    for [AccountMeta; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &PhoenixFulfillmentConfigStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.phoenix_fulfillment_config, false),
            AccountMeta::new(keys.admin, true),
        ]
    }
}
impl<'a> From<&PhoenixFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.phoenix_fulfillment_config.clone(),
            accounts.admin.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PhoenixFulfillmentConfigStatusIxArgs {
    pub status: SpotFulfillmentConfigStatus,
}
#[derive(Copy, Clone, Debug)]
pub struct PhoenixFulfillmentConfigStatusIxData<'me>(pub &'me PhoenixFulfillmentConfigStatusIxArgs);
pub const PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM: [u8; 8] = [96, 31, 113, 32, 12, 203, 7, 154];
impl<'me> From<&'me PhoenixFulfillmentConfigStatusIxArgs>
    for PhoenixFulfillmentConfigStatusIxData<'me>
{
    fn from(args: &'me PhoenixFulfillmentConfigStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PhoenixFulfillmentConfigStatusIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn phoenix_fulfillment_config_status_ix<
    K: Into<PhoenixFulfillmentConfigStatusKeys>,
    A: Into<PhoenixFulfillmentConfigStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PhoenixFulfillmentConfigStatusKeys = accounts.into();
    let metas: [AccountMeta; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PhoenixFulfillmentConfigStatusIxArgs = args.into();
    let data: PhoenixFulfillmentConfigStatusIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn phoenix_fulfillment_config_status_invoke<
    'a,
    A: Into<PhoenixFulfillmentConfigStatusIxArgs>,
>(
    accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = phoenix_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn phoenix_fulfillment_config_status_invoke_signed<
    'a,
    A: Into<PhoenixFulfillmentConfigStatusIxArgs>,
>(
    accounts: &PhoenixFulfillmentConfigStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = phoenix_fulfillment_config_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PHOENIX_FULFILLMENT_CONFIG_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumVaultAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub admin: &'me AccountInfo<'a1>,
    pub srm_vault: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumVaultKeys {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub srm_vault: Pubkey,
}
impl<'me> From<&UpdateSerumVaultAccounts<'me, '_, '_, '_>> for UpdateSerumVaultKeys {
    fn from(accounts: &UpdateSerumVaultAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            srm_vault: *accounts.srm_vault.key,
        }
    }
}
impl From<&UpdateSerumVaultKeys> for [AccountMeta; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateSerumVaultKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.admin, true),
            AccountMeta::new_readonly(keys.srm_vault, false),
        ]
    }
}
impl<'a> From<&UpdateSerumVaultAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSerumVaultAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.srm_vault.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSerumVaultIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSerumVaultIxData<'me>(pub &'me UpdateSerumVaultIxArgs);
pub const UPDATE_SERUM_VAULT_IX_DISCM: [u8; 8] = [219, 8, 246, 96, 169, 121, 91, 110];
impl<'me> From<&'me UpdateSerumVaultIxArgs> for UpdateSerumVaultIxData<'me> {
    fn from(args: &'me UpdateSerumVaultIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSerumVaultIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SERUM_VAULT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_serum_vault_ix<K: Into<UpdateSerumVaultKeys>, A: Into<UpdateSerumVaultIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSerumVaultKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSerumVaultIxArgs = args.into();
    let data: UpdateSerumVaultIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_serum_vault_invoke<'a, A: Into<UpdateSerumVaultIxArgs>>(
    accounts: &UpdateSerumVaultAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_serum_vault_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_serum_vault_invoke_signed<'a, A: Into<UpdateSerumVaultIxArgs>>(
    accounts: &UpdateSerumVaultAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_serum_vault_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SERUM_VAULT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct InitializePerpMarketAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
    pub oracle: &'me AccountInfo<'a3>,
    pub rent: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializePerpMarketKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub rent: Pubkey,
    pub system_program: Pubkey,
}
impl<'me> From<&InitializePerpMarketAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for InitializePerpMarketKeys
{
    fn from(accounts: &InitializePerpMarketAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            rent: *accounts.rent.key,
            system_program: *accounts.system_program.key,
        }
    }
}
impl From<&InitializePerpMarketKeys> for [AccountMeta; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] {
    fn from(keys: &InitializePerpMarketKeys) -> Self {
        [
            AccountMeta::new(keys.admin, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.system_program, false),
        ]
    }
}
impl<'a> From<&InitializePerpMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &InitializePerpMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.rent.clone(),
            accounts.system_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct InitializePerpMarketIxArgs {
    pub market_index: u16,
    pub amm_base_asset_reserve: u128,
    pub amm_quote_asset_reserve: u128,
    pub amm_periodicity: i64,
    pub amm_peg_multiplier: u128,
    pub oracle_source: OracleSource,
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
    pub liquidator_fee: u32,
    pub active_status: bool,
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct InitializePerpMarketIxData<'me>(pub &'me InitializePerpMarketIxArgs);
pub const INITIALIZE_PERP_MARKET_IX_DISCM: [u8; 8] = [132, 9, 229, 118, 117, 118, 117, 62];
impl<'me> From<&'me InitializePerpMarketIxArgs> for InitializePerpMarketIxData<'me> {
    fn from(args: &'me InitializePerpMarketIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for InitializePerpMarketIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_PERP_MARKET_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn initialize_perp_market_ix<
    K: Into<InitializePerpMarketKeys>,
    A: Into<InitializePerpMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: InitializePerpMarketKeys = accounts.into();
    let metas: [AccountMeta; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: InitializePerpMarketIxArgs = args.into();
    let data: InitializePerpMarketIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_perp_market_invoke<'a, A: Into<InitializePerpMarketIxArgs>>(
    accounts: &InitializePerpMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = initialize_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn initialize_perp_market_invoke_signed<'a, A: Into<InitializePerpMarketIxArgs>>(
    accounts: &InitializePerpMarketAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = initialize_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; INITIALIZE_PERP_MARKET_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct DeleteInitializedPerpMarketAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeleteInitializedPerpMarketKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&DeleteInitializedPerpMarketAccounts<'me, '_, '_, '_>>
    for DeleteInitializedPerpMarketKeys
{
    fn from(accounts: &DeleteInitializedPerpMarketAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&DeleteInitializedPerpMarketKeys>
    for [AccountMeta; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeleteInitializedPerpMarketKeys) -> Self {
        [
            AccountMeta::new(keys.admin, true),
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&DeleteInitializedPerpMarketAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeleteInitializedPerpMarketAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeleteInitializedPerpMarketIxArgs {
    pub market_index: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct DeleteInitializedPerpMarketIxData<'me>(pub &'me DeleteInitializedPerpMarketIxArgs);
pub const DELETE_INITIALIZED_PERP_MARKET_IX_DISCM: [u8; 8] = [91, 154, 24, 87, 106, 59, 190, 66];
impl<'me> From<&'me DeleteInitializedPerpMarketIxArgs> for DeleteInitializedPerpMarketIxData<'me> {
    fn from(args: &'me DeleteInitializedPerpMarketIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeleteInitializedPerpMarketIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DELETE_INITIALIZED_PERP_MARKET_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn delete_initialized_perp_market_ix<
    K: Into<DeleteInitializedPerpMarketKeys>,
    A: Into<DeleteInitializedPerpMarketIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeleteInitializedPerpMarketKeys = accounts.into();
    let metas: [AccountMeta; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeleteInitializedPerpMarketIxArgs = args.into();
    let data: DeleteInitializedPerpMarketIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn delete_initialized_perp_market_invoke<'a, A: Into<DeleteInitializedPerpMarketIxArgs>>(
    accounts: &DeleteInitializedPerpMarketAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = delete_initialized_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn delete_initialized_perp_market_invoke_signed<
    'a,
    A: Into<DeleteInitializedPerpMarketIxArgs>,
>(
    accounts: &DeleteInitializedPerpMarketAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = delete_initialized_perp_market_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DELETE_INITIALIZED_PERP_MARKET_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MOVE_AMM_PRICE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct MoveAmmPriceAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct MoveAmmPriceKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&MoveAmmPriceAccounts<'me, '_, '_, '_>> for MoveAmmPriceKeys {
    fn from(accounts: &MoveAmmPriceAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&MoveAmmPriceKeys> for [AccountMeta; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] {
    fn from(keys: &MoveAmmPriceKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&MoveAmmPriceAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &MoveAmmPriceAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct MoveAmmPriceIxArgs {
    pub base_asset_reserve: u128,
    pub quote_asset_reserve: u128,
    pub sqrt_k: u128,
}
#[derive(Copy, Clone, Debug)]
pub struct MoveAmmPriceIxData<'me>(pub &'me MoveAmmPriceIxArgs);
pub const MOVE_AMM_PRICE_IX_DISCM: [u8; 8] = [235, 109, 2, 82, 219, 118, 6, 159];
impl<'me> From<&'me MoveAmmPriceIxArgs> for MoveAmmPriceIxData<'me> {
    fn from(args: &'me MoveAmmPriceIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MoveAmmPriceIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&MOVE_AMM_PRICE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn move_amm_price_ix<K: Into<MoveAmmPriceKeys>, A: Into<MoveAmmPriceIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MoveAmmPriceKeys = accounts.into();
    let metas: [AccountMeta; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: MoveAmmPriceIxArgs = args.into();
    let data: MoveAmmPriceIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn move_amm_price_invoke<'a, A: Into<MoveAmmPriceIxArgs>>(
    accounts: &MoveAmmPriceAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = move_amm_price_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn move_amm_price_invoke_signed<'a, A: Into<MoveAmmPriceIxArgs>>(
    accounts: &MoveAmmPriceAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = move_amm_price_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; MOVE_AMM_PRICE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketExpiryAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketExpiryKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketExpiryAccounts<'me, '_, '_, '_>> for UpdatePerpMarketExpiryKeys {
    fn from(accounts: &UpdatePerpMarketExpiryAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketExpiryKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketExpiryKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketExpiryAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketExpiryAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketExpiryIxArgs {
    pub expiry_ts: i64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketExpiryIxData<'me>(pub &'me UpdatePerpMarketExpiryIxArgs);
pub const UPDATE_PERP_MARKET_EXPIRY_IX_DISCM: [u8; 8] = [44, 221, 227, 151, 131, 140, 22, 110];
impl<'me> From<&'me UpdatePerpMarketExpiryIxArgs> for UpdatePerpMarketExpiryIxData<'me> {
    fn from(args: &'me UpdatePerpMarketExpiryIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketExpiryIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_EXPIRY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_expiry_ix<
    K: Into<UpdatePerpMarketExpiryKeys>,
    A: Into<UpdatePerpMarketExpiryIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketExpiryKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketExpiryIxArgs = args.into();
    let data: UpdatePerpMarketExpiryIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_expiry_invoke<'a, A: Into<UpdatePerpMarketExpiryIxArgs>>(
    accounts: &UpdatePerpMarketExpiryAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_expiry_invoke_signed<'a, A: Into<UpdatePerpMarketExpiryIxArgs>>(
    accounts: &UpdatePerpMarketExpiryAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_expiry_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_EXPIRY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketPoolsToRevenuePoolAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub admin: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
    pub perp_market: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketPoolsToRevenuePoolKeys {
    pub state: Pubkey,
    pub admin: Pubkey,
    pub spot_market: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&SettleExpiredMarketPoolsToRevenuePoolAccounts<'me, '_, '_, '_, '_>>
    for SettleExpiredMarketPoolsToRevenuePoolKeys
{
    fn from(accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            admin: *accounts.admin.key,
            spot_market: *accounts.spot_market.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&SettleExpiredMarketPoolsToRevenuePoolKeys>
    for [AccountMeta; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SettleExpiredMarketPoolsToRevenuePoolKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.admin.clone(),
            accounts.spot_market.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SettleExpiredMarketPoolsToRevenuePoolIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SettleExpiredMarketPoolsToRevenuePoolIxData<'me>(
    pub &'me SettleExpiredMarketPoolsToRevenuePoolIxArgs,
);
pub const SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM: [u8; 8] =
    [55, 19, 238, 169, 227, 90, 200, 184];
impl<'me> From<&'me SettleExpiredMarketPoolsToRevenuePoolIxArgs>
    for SettleExpiredMarketPoolsToRevenuePoolIxData<'me>
{
    fn from(args: &'me SettleExpiredMarketPoolsToRevenuePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SettleExpiredMarketPoolsToRevenuePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn settle_expired_market_pools_to_revenue_pool_ix<
    K: Into<SettleExpiredMarketPoolsToRevenuePoolKeys>,
    A: Into<SettleExpiredMarketPoolsToRevenuePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SettleExpiredMarketPoolsToRevenuePoolKeys = accounts.into();
    let metas: [AccountMeta; SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: SettleExpiredMarketPoolsToRevenuePoolIxArgs = args.into();
    let data: SettleExpiredMarketPoolsToRevenuePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn settle_expired_market_pools_to_revenue_pool_invoke<
    'a,
    A: Into<SettleExpiredMarketPoolsToRevenuePoolIxArgs>,
>(
    accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = settle_expired_market_pools_to_revenue_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn settle_expired_market_pools_to_revenue_pool_invoke_signed<
    'a,
    A: Into<SettleExpiredMarketPoolsToRevenuePoolIxArgs>,
>(
    accounts: &SettleExpiredMarketPoolsToRevenuePoolAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = settle_expired_market_pools_to_revenue_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        SETTLE_EXPIRED_MARKET_POOLS_TO_REVENUE_POOL_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN: usize = 8usize;
#[derive(Copy, Clone, Debug)]
pub struct DepositIntoPerpMarketFeePoolAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
    'a7: 'me,
> {
    pub state: &'me AccountInfo<'a0>,
    pub perp_market: &'me AccountInfo<'a1>,
    pub admin: &'me AccountInfo<'a2>,
    pub source_vault: &'me AccountInfo<'a3>,
    pub drift_signer: &'me AccountInfo<'a4>,
    pub quote_spot_market: &'me AccountInfo<'a5>,
    pub spot_market_vault: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositIntoPerpMarketFeePoolKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub admin: Pubkey,
    pub source_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub quote_spot_market: Pubkey,
    pub spot_market_vault: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&DepositIntoPerpMarketFeePoolAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for DepositIntoPerpMarketFeePoolKeys
{
    fn from(
        accounts: &DepositIntoPerpMarketFeePoolAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            admin: *accounts.admin.key,
            source_vault: *accounts.source_vault.key,
            drift_signer: *accounts.drift_signer.key,
            quote_spot_market: *accounts.quote_spot_market.key,
            spot_market_vault: *accounts.spot_market_vault.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&DepositIntoPerpMarketFeePoolKeys>
    for [AccountMeta; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DepositIntoPerpMarketFeePoolKeys) -> Self {
        [
            AccountMeta::new(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.source_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.quote_spot_market, false),
            AccountMeta::new(keys.spot_market_vault, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&DepositIntoPerpMarketFeePoolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.admin.clone(),
            accounts.source_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.quote_spot_market.clone(),
            accounts.spot_market_vault.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DepositIntoPerpMarketFeePoolIxArgs {
    pub amount: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct DepositIntoPerpMarketFeePoolIxData<'me>(pub &'me DepositIntoPerpMarketFeePoolIxArgs);
pub const DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM: [u8; 8] = [34, 58, 57, 68, 97, 80, 244, 6];
impl<'me> From<&'me DepositIntoPerpMarketFeePoolIxArgs>
    for DepositIntoPerpMarketFeePoolIxData<'me>
{
    fn from(args: &'me DepositIntoPerpMarketFeePoolIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DepositIntoPerpMarketFeePoolIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn deposit_into_perp_market_fee_pool_ix<
    K: Into<DepositIntoPerpMarketFeePoolKeys>,
    A: Into<DepositIntoPerpMarketFeePoolIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DepositIntoPerpMarketFeePoolKeys = accounts.into();
    let metas: [AccountMeta; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DepositIntoPerpMarketFeePoolIxArgs = args.into();
    let data: DepositIntoPerpMarketFeePoolIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deposit_into_perp_market_fee_pool_invoke<'a, A: Into<DepositIntoPerpMarketFeePoolIxArgs>>(
    accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deposit_into_perp_market_fee_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deposit_into_perp_market_fee_pool_invoke_signed<
    'a,
    A: Into<DepositIntoPerpMarketFeePoolIxArgs>,
>(
    accounts: &DepositIntoPerpMarketFeePoolAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deposit_into_perp_market_fee_pool_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPOSIT_INTO_PERP_MARKET_FEE_POOL_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REPEG_AMM_CURVE_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct RepegAmmCurveAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub perp_market: &'me AccountInfo<'a1>,
    pub oracle: &'me AccountInfo<'a2>,
    pub admin: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct RepegAmmCurveKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl<'me> From<&RepegAmmCurveAccounts<'me, '_, '_, '_, '_>> for RepegAmmCurveKeys {
    fn from(accounts: &RepegAmmCurveAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&RepegAmmCurveKeys> for [AccountMeta; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] {
    fn from(keys: &RepegAmmCurveKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl<'a> From<&RepegAmmCurveAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RepegAmmCurveAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RepegAmmCurveIxArgs {
    pub new_peg_candidate: u128,
}
#[derive(Copy, Clone, Debug)]
pub struct RepegAmmCurveIxData<'me>(pub &'me RepegAmmCurveIxArgs);
pub const REPEG_AMM_CURVE_IX_DISCM: [u8; 8] = [3, 36, 102, 89, 180, 128, 120, 213];
impl<'me> From<&'me RepegAmmCurveIxArgs> for RepegAmmCurveIxData<'me> {
    fn from(args: &'me RepegAmmCurveIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RepegAmmCurveIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&REPEG_AMM_CURVE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn repeg_amm_curve_ix<K: Into<RepegAmmCurveKeys>, A: Into<RepegAmmCurveIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RepegAmmCurveKeys = accounts.into();
    let metas: [AccountMeta; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RepegAmmCurveIxArgs = args.into();
    let data: RepegAmmCurveIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn repeg_amm_curve_invoke<'a, A: Into<RepegAmmCurveIxArgs>>(
    accounts: &RepegAmmCurveAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = repeg_amm_curve_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn repeg_amm_curve_invoke_signed<'a, A: Into<RepegAmmCurveIxArgs>>(
    accounts: &RepegAmmCurveAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = repeg_amm_curve_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REPEG_AMM_CURVE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketAmmOracleTwapAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub perp_market: &'me AccountInfo<'a1>,
    pub oracle: &'me AccountInfo<'a2>,
    pub admin: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketAmmOracleTwapKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl<'me> From<&UpdatePerpMarketAmmOracleTwapAccounts<'me, '_, '_, '_, '_>>
    for UpdatePerpMarketAmmOracleTwapKeys
{
    fn from(accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&UpdatePerpMarketAmmOracleTwapKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketAmmOracleTwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketAmmOracleTwapIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketAmmOracleTwapIxData<'me>(pub &'me UpdatePerpMarketAmmOracleTwapIxArgs);
pub const UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM: [u8; 8] =
    [241, 74, 114, 123, 206, 153, 24, 202];
impl<'me> From<&'me UpdatePerpMarketAmmOracleTwapIxArgs>
    for UpdatePerpMarketAmmOracleTwapIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketAmmOracleTwapIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketAmmOracleTwapIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_amm_oracle_twap_ix<
    K: Into<UpdatePerpMarketAmmOracleTwapKeys>,
    A: Into<UpdatePerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketAmmOracleTwapKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketAmmOracleTwapIxArgs = args.into();
    let data: UpdatePerpMarketAmmOracleTwapIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_amm_oracle_twap_invoke<
    'a,
    A: Into<UpdatePerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_amm_oracle_twap_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &UpdatePerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct ResetPerpMarketAmmOracleTwapAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub perp_market: &'me AccountInfo<'a1>,
    pub oracle: &'me AccountInfo<'a2>,
    pub admin: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct ResetPerpMarketAmmOracleTwapKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl<'me> From<&ResetPerpMarketAmmOracleTwapAccounts<'me, '_, '_, '_, '_>>
    for ResetPerpMarketAmmOracleTwapKeys
{
    fn from(accounts: &ResetPerpMarketAmmOracleTwapAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&ResetPerpMarketAmmOracleTwapKeys>
    for [AccountMeta; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(keys: &ResetPerpMarketAmmOracleTwapKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl<'a> From<&ResetPerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ResetPerpMarketAmmOracleTwapIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ResetPerpMarketAmmOracleTwapIxData<'me>(pub &'me ResetPerpMarketAmmOracleTwapIxArgs);
pub const RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM: [u8; 8] =
    [127, 10, 55, 164, 123, 226, 47, 24];
impl<'me> From<&'me ResetPerpMarketAmmOracleTwapIxArgs>
    for ResetPerpMarketAmmOracleTwapIxData<'me>
{
    fn from(args: &'me ResetPerpMarketAmmOracleTwapIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ResetPerpMarketAmmOracleTwapIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn reset_perp_market_amm_oracle_twap_ix<
    K: Into<ResetPerpMarketAmmOracleTwapKeys>,
    A: Into<ResetPerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ResetPerpMarketAmmOracleTwapKeys = accounts.into();
    let metas: [AccountMeta; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ResetPerpMarketAmmOracleTwapIxArgs = args.into();
    let data: ResetPerpMarketAmmOracleTwapIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn reset_perp_market_amm_oracle_twap_invoke<'a, A: Into<ResetPerpMarketAmmOracleTwapIxArgs>>(
    accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = reset_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn reset_perp_market_amm_oracle_twap_invoke_signed<
    'a,
    A: Into<ResetPerpMarketAmmOracleTwapIxArgs>,
>(
    accounts: &ResetPerpMarketAmmOracleTwapAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = reset_perp_market_amm_oracle_twap_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; RESET_PERP_MARKET_AMM_ORACLE_TWAP_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_K_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateKAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
    pub oracle: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateKKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
}
impl<'me> From<&UpdateKAccounts<'me, '_, '_, '_, '_>> for UpdateKKeys {
    fn from(accounts: &UpdateKAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateKKeys> for [AccountMeta; UPDATE_K_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateKKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl<'a> From<&UpdateKAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_K_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateKAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateKIxArgs {
    pub sqrt_k: u128,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateKIxData<'me>(pub &'me UpdateKIxArgs);
pub const UPDATE_K_IX_DISCM: [u8; 8] = [72, 98, 9, 139, 129, 229, 172, 56];
impl<'me> From<&'me UpdateKIxArgs> for UpdateKIxData<'me> {
    fn from(args: &'me UpdateKIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateKIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_K_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_k_ix<K: Into<UpdateKKeys>, A: Into<UpdateKIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateKKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_K_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateKIxArgs = args.into();
    let data: UpdateKIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_k_invoke<'a, A: Into<UpdateKIxArgs>>(
    accounts: &UpdateKAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_k_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_K_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_k_invoke_signed<'a, A: Into<UpdateKIxArgs>>(
    accounts: &UpdateKAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_k_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_K_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMarginRatioAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMarginRatioKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMarginRatioAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMarginRatioKeys
{
    fn from(accounts: &UpdatePerpMarketMarginRatioAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMarginRatioKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMarginRatioKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMarginRatioAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMarginRatioAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMarginRatioIxArgs {
    pub margin_ratio_initial: u32,
    pub margin_ratio_maintenance: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMarginRatioIxData<'me>(pub &'me UpdatePerpMarketMarginRatioIxArgs);
pub const UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM: [u8; 8] =
    [130, 173, 107, 45, 119, 105, 26, 113];
impl<'me> From<&'me UpdatePerpMarketMarginRatioIxArgs> for UpdatePerpMarketMarginRatioIxData<'me> {
    fn from(args: &'me UpdatePerpMarketMarginRatioIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMarginRatioIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MARGIN_RATIO_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_margin_ratio_ix<
    K: Into<UpdatePerpMarketMarginRatioKeys>,
    A: Into<UpdatePerpMarketMarginRatioIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMarginRatioKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMarginRatioIxArgs = args.into();
    let data: UpdatePerpMarketMarginRatioIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_margin_ratio_invoke<'a, A: Into<UpdatePerpMarketMarginRatioIxArgs>>(
    accounts: &UpdatePerpMarketMarginRatioAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_margin_ratio_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketMarginRatioIxArgs>,
>(
    accounts: &UpdatePerpMarketMarginRatioAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_margin_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MARGIN_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxImbalancesAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxImbalancesKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMaxImbalancesAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMaxImbalancesKeys
{
    fn from(accounts: &UpdatePerpMarketMaxImbalancesAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxImbalancesKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxImbalancesKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMaxImbalancesAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMaxImbalancesIxArgs {
    pub unrealized_max_imbalance: u64,
    pub max_revenue_withdraw_per_period: u64,
    pub quote_max_insurance: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxImbalancesIxData<'me>(pub &'me UpdatePerpMarketMaxImbalancesIxArgs);
pub const UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM: [u8; 8] = [15, 206, 73, 133, 60, 8, 86, 89];
impl<'me> From<&'me UpdatePerpMarketMaxImbalancesIxArgs>
    for UpdatePerpMarketMaxImbalancesIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketMaxImbalancesIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMaxImbalancesIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_max_imbalances_ix<
    K: Into<UpdatePerpMarketMaxImbalancesKeys>,
    A: Into<UpdatePerpMarketMaxImbalancesIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxImbalancesKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMaxImbalancesIxArgs = args.into();
    let data: UpdatePerpMarketMaxImbalancesIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_imbalances_invoke<
    'a,
    A: Into<UpdatePerpMarketMaxImbalancesIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_imbalances_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_imbalances_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketMaxImbalancesIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxImbalancesAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_imbalances_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_IMBALANCES_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketLiquidationFeeAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketLiquidationFeeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketLiquidationFeeAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketLiquidationFeeKeys
{
    fn from(accounts: &UpdatePerpMarketLiquidationFeeAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketLiquidationFeeKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketLiquidationFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketLiquidationFeeIxArgs {
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketLiquidationFeeIxData<'me>(pub &'me UpdatePerpMarketLiquidationFeeIxArgs);
pub const UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM: [u8; 8] = [90, 137, 9, 145, 41, 8, 148, 117];
impl<'me> From<&'me UpdatePerpMarketLiquidationFeeIxArgs>
    for UpdatePerpMarketLiquidationFeeIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketLiquidationFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketLiquidationFeeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_liquidation_fee_ix<
    K: Into<UpdatePerpMarketLiquidationFeeKeys>,
    A: Into<UpdatePerpMarketLiquidationFeeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketLiquidationFeeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketLiquidationFeeIxArgs = args.into();
    let data: UpdatePerpMarketLiquidationFeeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_liquidation_fee_invoke<
    'a,
    A: Into<UpdatePerpMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_liquidation_fee_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdatePerpMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateInsuranceFundUnstakingPeriodAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateInsuranceFundUnstakingPeriodKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateInsuranceFundUnstakingPeriodAccounts<'me, '_, '_, '_>>
    for UpdateInsuranceFundUnstakingPeriodKeys
{
    fn from(accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateInsuranceFundUnstakingPeriodKeys>
    for [AccountMeta; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateInsuranceFundUnstakingPeriodKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateInsuranceFundUnstakingPeriodIxArgs {
    pub insurance_fund_unstaking_period: i64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateInsuranceFundUnstakingPeriodIxData<'me>(
    pub &'me UpdateInsuranceFundUnstakingPeriodIxArgs,
);
pub const UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM: [u8; 8] =
    [44, 69, 43, 226, 204, 223, 202, 52];
impl<'me> From<&'me UpdateInsuranceFundUnstakingPeriodIxArgs>
    for UpdateInsuranceFundUnstakingPeriodIxData<'me>
{
    fn from(args: &'me UpdateInsuranceFundUnstakingPeriodIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateInsuranceFundUnstakingPeriodIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_insurance_fund_unstaking_period_ix<
    K: Into<UpdateInsuranceFundUnstakingPeriodKeys>,
    A: Into<UpdateInsuranceFundUnstakingPeriodIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateInsuranceFundUnstakingPeriodKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateInsuranceFundUnstakingPeriodIxArgs = args.into();
    let data: UpdateInsuranceFundUnstakingPeriodIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_insurance_fund_unstaking_period_invoke<
    'a,
    A: Into<UpdateInsuranceFundUnstakingPeriodIxArgs>,
>(
    accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_insurance_fund_unstaking_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_insurance_fund_unstaking_period_invoke_signed<
    'a,
    A: Into<UpdateInsuranceFundUnstakingPeriodIxArgs>,
>(
    accounts: &UpdateInsuranceFundUnstakingPeriodAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_insurance_fund_unstaking_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_INSURANCE_FUND_UNSTAKING_PERIOD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketLiquidationFeeAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketLiquidationFeeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketLiquidationFeeAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketLiquidationFeeKeys
{
    fn from(accounts: &UpdateSpotMarketLiquidationFeeAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketLiquidationFeeKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketLiquidationFeeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketLiquidationFeeIxArgs {
    pub liquidator_fee: u32,
    pub if_liquidation_fee: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketLiquidationFeeIxData<'me>(pub &'me UpdateSpotMarketLiquidationFeeIxArgs);
pub const UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM: [u8; 8] =
    [11, 13, 255, 53, 56, 136, 104, 177];
impl<'me> From<&'me UpdateSpotMarketLiquidationFeeIxArgs>
    for UpdateSpotMarketLiquidationFeeIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketLiquidationFeeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketLiquidationFeeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_liquidation_fee_ix<
    K: Into<UpdateSpotMarketLiquidationFeeKeys>,
    A: Into<UpdateSpotMarketLiquidationFeeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketLiquidationFeeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketLiquidationFeeIxArgs = args.into();
    let data: UpdateSpotMarketLiquidationFeeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_liquidation_fee_invoke<
    'a,
    A: Into<UpdateSpotMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_liquidation_fee_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketLiquidationFeeIxArgs>,
>(
    accounts: &UpdateSpotMarketLiquidationFeeAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_liquidation_fee_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_LIQUIDATION_FEE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateWithdrawGuardThresholdAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateWithdrawGuardThresholdKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateWithdrawGuardThresholdAccounts<'me, '_, '_, '_>>
    for UpdateWithdrawGuardThresholdKeys
{
    fn from(accounts: &UpdateWithdrawGuardThresholdAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateWithdrawGuardThresholdKeys>
    for [AccountMeta; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateWithdrawGuardThresholdKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateWithdrawGuardThresholdAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateWithdrawGuardThresholdAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateWithdrawGuardThresholdIxArgs {
    pub withdraw_guard_threshold: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateWithdrawGuardThresholdIxData<'me>(pub &'me UpdateWithdrawGuardThresholdIxArgs);
pub const UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM: [u8; 8] = [56, 18, 39, 61, 155, 211, 44, 133];
impl<'me> From<&'me UpdateWithdrawGuardThresholdIxArgs>
    for UpdateWithdrawGuardThresholdIxData<'me>
{
    fn from(args: &'me UpdateWithdrawGuardThresholdIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateWithdrawGuardThresholdIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_withdraw_guard_threshold_ix<
    K: Into<UpdateWithdrawGuardThresholdKeys>,
    A: Into<UpdateWithdrawGuardThresholdIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateWithdrawGuardThresholdKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateWithdrawGuardThresholdIxArgs = args.into();
    let data: UpdateWithdrawGuardThresholdIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_withdraw_guard_threshold_invoke<'a, A: Into<UpdateWithdrawGuardThresholdIxArgs>>(
    accounts: &UpdateWithdrawGuardThresholdAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_withdraw_guard_threshold_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_withdraw_guard_threshold_invoke_signed<
    'a,
    A: Into<UpdateWithdrawGuardThresholdIxArgs>,
>(
    accounts: &UpdateWithdrawGuardThresholdAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_withdraw_guard_threshold_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_WITHDRAW_GUARD_THRESHOLD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketIfFactorAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketIfFactorKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketIfFactorAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketIfFactorKeys
{
    fn from(accounts: &UpdateSpotMarketIfFactorAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketIfFactorKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketIfFactorKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketIfFactorAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketIfFactorAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketIfFactorIxArgs {
    pub spot_market_index: u16,
    pub user_if_factor: u32,
    pub total_if_factor: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketIfFactorIxData<'me>(pub &'me UpdateSpotMarketIfFactorIxArgs);
pub const UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM: [u8; 8] = [147, 30, 224, 34, 18, 230, 105, 4];
impl<'me> From<&'me UpdateSpotMarketIfFactorIxArgs> for UpdateSpotMarketIfFactorIxData<'me> {
    fn from(args: &'me UpdateSpotMarketIfFactorIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketIfFactorIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_IF_FACTOR_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_if_factor_ix<
    K: Into<UpdateSpotMarketIfFactorKeys>,
    A: Into<UpdateSpotMarketIfFactorIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketIfFactorKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketIfFactorIxArgs = args.into();
    let data: UpdateSpotMarketIfFactorIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_if_factor_invoke<'a, A: Into<UpdateSpotMarketIfFactorIxArgs>>(
    accounts: &UpdateSpotMarketIfFactorAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_if_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_if_factor_invoke_signed<'a, A: Into<UpdateSpotMarketIfFactorIxArgs>>(
    accounts: &UpdateSpotMarketIfFactorAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_if_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_IF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketRevenueSettlePeriodAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketRevenueSettlePeriodKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketRevenueSettlePeriodAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketRevenueSettlePeriodKeys
{
    fn from(accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketRevenueSettlePeriodKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketRevenueSettlePeriodKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketRevenueSettlePeriodIxArgs {
    pub revenue_settle_period: i64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketRevenueSettlePeriodIxData<'me>(
    pub &'me UpdateSpotMarketRevenueSettlePeriodIxArgs,
);
pub const UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM: [u8; 8] =
    [81, 92, 126, 41, 250, 225, 156, 219];
impl<'me> From<&'me UpdateSpotMarketRevenueSettlePeriodIxArgs>
    for UpdateSpotMarketRevenueSettlePeriodIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketRevenueSettlePeriodIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketRevenueSettlePeriodIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_revenue_settle_period_ix<
    K: Into<UpdateSpotMarketRevenueSettlePeriodKeys>,
    A: Into<UpdateSpotMarketRevenueSettlePeriodIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketRevenueSettlePeriodKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketRevenueSettlePeriodIxArgs = args.into();
    let data: UpdateSpotMarketRevenueSettlePeriodIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_revenue_settle_period_invoke<
    'a,
    A: Into<UpdateSpotMarketRevenueSettlePeriodIxArgs>,
>(
    accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_revenue_settle_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_revenue_settle_period_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketRevenueSettlePeriodIxArgs>,
>(
    accounts: &UpdateSpotMarketRevenueSettlePeriodAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_revenue_settle_period_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_REVENUE_SETTLE_PERIOD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStatusAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStatusKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketStatusAccounts<'me, '_, '_, '_>> for UpdateSpotMarketStatusKeys {
    fn from(accounts: &UpdateSpotMarketStatusAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketStatusKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketStatusAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketStatusAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketStatusIxArgs {
    pub status: MarketStatus,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStatusIxData<'me>(pub &'me UpdateSpotMarketStatusIxArgs);
pub const UPDATE_SPOT_MARKET_STATUS_IX_DISCM: [u8; 8] = [78, 94, 16, 188, 193, 110, 231, 31];
impl<'me> From<&'me UpdateSpotMarketStatusIxArgs> for UpdateSpotMarketStatusIxData<'me> {
    fn from(args: &'me UpdateSpotMarketStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketStatusIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_STATUS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_status_ix<
    K: Into<UpdateSpotMarketStatusKeys>,
    A: Into<UpdateSpotMarketStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketStatusIxArgs = args.into();
    let data: UpdateSpotMarketStatusIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_status_invoke<'a, A: Into<UpdateSpotMarketStatusIxArgs>>(
    accounts: &UpdateSpotMarketStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_status_invoke_signed<'a, A: Into<UpdateSpotMarketStatusIxArgs>>(
    accounts: &UpdateSpotMarketStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketAssetTierAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketAssetTierKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketAssetTierAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketAssetTierKeys
{
    fn from(accounts: &UpdateSpotMarketAssetTierAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketAssetTierKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketAssetTierKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketAssetTierAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketAssetTierAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketAssetTierIxArgs {
    pub asset_tier: AssetTier,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketAssetTierIxData<'me>(pub &'me UpdateSpotMarketAssetTierIxArgs);
pub const UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM: [u8; 8] = [253, 209, 231, 14, 242, 208, 243, 130];
impl<'me> From<&'me UpdateSpotMarketAssetTierIxArgs> for UpdateSpotMarketAssetTierIxData<'me> {
    fn from(args: &'me UpdateSpotMarketAssetTierIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketAssetTierIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_ASSET_TIER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_asset_tier_ix<
    K: Into<UpdateSpotMarketAssetTierKeys>,
    A: Into<UpdateSpotMarketAssetTierIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketAssetTierKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketAssetTierIxArgs = args.into();
    let data: UpdateSpotMarketAssetTierIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_asset_tier_invoke<'a, A: Into<UpdateSpotMarketAssetTierIxArgs>>(
    accounts: &UpdateSpotMarketAssetTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_asset_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_asset_tier_invoke_signed<'a, A: Into<UpdateSpotMarketAssetTierIxArgs>>(
    accounts: &UpdateSpotMarketAssetTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_asset_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_ASSET_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMarginWeightsAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMarginWeightsKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketMarginWeightsAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketMarginWeightsKeys
{
    fn from(accounts: &UpdateSpotMarketMarginWeightsAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketMarginWeightsKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketMarginWeightsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketMarginWeightsAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketMarginWeightsIxArgs {
    pub initial_asset_weight: u32,
    pub maintenance_asset_weight: u32,
    pub initial_liability_weight: u32,
    pub maintenance_liability_weight: u32,
    pub imf_factor: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMarginWeightsIxData<'me>(pub &'me UpdateSpotMarketMarginWeightsIxArgs);
pub const UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM: [u8; 8] = [109, 33, 87, 195, 255, 36, 6, 81];
impl<'me> From<&'me UpdateSpotMarketMarginWeightsIxArgs>
    for UpdateSpotMarketMarginWeightsIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketMarginWeightsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketMarginWeightsIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_margin_weights_ix<
    K: Into<UpdateSpotMarketMarginWeightsKeys>,
    A: Into<UpdateSpotMarketMarginWeightsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketMarginWeightsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketMarginWeightsIxArgs = args.into();
    let data: UpdateSpotMarketMarginWeightsIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_margin_weights_invoke<
    'a,
    A: Into<UpdateSpotMarketMarginWeightsIxArgs>,
>(
    accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_margin_weights_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_margin_weights_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketMarginWeightsIxArgs>,
>(
    accounts: &UpdateSpotMarketMarginWeightsAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_margin_weights_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_MARGIN_WEIGHTS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketBorrowRateAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketBorrowRateKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketBorrowRateAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketBorrowRateKeys
{
    fn from(accounts: &UpdateSpotMarketBorrowRateAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketBorrowRateKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketBorrowRateKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketBorrowRateAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketBorrowRateAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketBorrowRateIxArgs {
    pub optimal_utilization: u32,
    pub optimal_borrow_rate: u32,
    pub max_borrow_rate: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketBorrowRateIxData<'me>(pub &'me UpdateSpotMarketBorrowRateIxArgs);
pub const UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM: [u8; 8] = [71, 239, 236, 153, 210, 62, 254, 76];
impl<'me> From<&'me UpdateSpotMarketBorrowRateIxArgs> for UpdateSpotMarketBorrowRateIxData<'me> {
    fn from(args: &'me UpdateSpotMarketBorrowRateIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketBorrowRateIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_BORROW_RATE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_borrow_rate_ix<
    K: Into<UpdateSpotMarketBorrowRateKeys>,
    A: Into<UpdateSpotMarketBorrowRateIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketBorrowRateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketBorrowRateIxArgs = args.into();
    let data: UpdateSpotMarketBorrowRateIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_borrow_rate_invoke<'a, A: Into<UpdateSpotMarketBorrowRateIxArgs>>(
    accounts: &UpdateSpotMarketBorrowRateAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_borrow_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_borrow_rate_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketBorrowRateIxArgs>,
>(
    accounts: &UpdateSpotMarketBorrowRateAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_borrow_rate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_BORROW_RATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMaxTokenDepositsAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMaxTokenDepositsKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketMaxTokenDepositsAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketMaxTokenDepositsKeys
{
    fn from(accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketMaxTokenDepositsKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketMaxTokenDepositsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketMaxTokenDepositsIxArgs {
    pub max_token_deposits: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMaxTokenDepositsIxData<'me>(
    pub &'me UpdateSpotMarketMaxTokenDepositsIxArgs,
);
pub const UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM: [u8; 8] =
    [56, 191, 79, 18, 26, 121, 80, 208];
impl<'me> From<&'me UpdateSpotMarketMaxTokenDepositsIxArgs>
    for UpdateSpotMarketMaxTokenDepositsIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketMaxTokenDepositsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketMaxTokenDepositsIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_max_token_deposits_ix<
    K: Into<UpdateSpotMarketMaxTokenDepositsKeys>,
    A: Into<UpdateSpotMarketMaxTokenDepositsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketMaxTokenDepositsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketMaxTokenDepositsIxArgs = args.into();
    let data: UpdateSpotMarketMaxTokenDepositsIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_max_token_deposits_invoke<
    'a,
    A: Into<UpdateSpotMarketMaxTokenDepositsIxArgs>,
>(
    accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_max_token_deposits_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_max_token_deposits_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketMaxTokenDepositsIxArgs>,
>(
    accounts: &UpdateSpotMarketMaxTokenDepositsAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_max_token_deposits_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_MAX_TOKEN_DEPOSITS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOracleAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
    pub oracle: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOracleKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub oracle: Pubkey,
}
impl<'me> From<&UpdateSpotMarketOracleAccounts<'me, '_, '_, '_, '_>>
    for UpdateSpotMarketOracleKeys
{
    fn from(accounts: &UpdateSpotMarketOracleAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            oracle: *accounts.oracle.key,
        }
    }
}
impl From<&UpdateSpotMarketOracleKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketOracleKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.oracle.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketOracleIxArgs {
    pub oracle: Pubkey,
    pub oracle_source: OracleSource,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOracleIxData<'me>(pub &'me UpdateSpotMarketOracleIxArgs);
pub const UPDATE_SPOT_MARKET_ORACLE_IX_DISCM: [u8; 8] = [114, 184, 102, 37, 246, 186, 180, 99];
impl<'me> From<&'me UpdateSpotMarketOracleIxArgs> for UpdateSpotMarketOracleIxData<'me> {
    fn from(args: &'me UpdateSpotMarketOracleIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketOracleIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_ORACLE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_oracle_ix<
    K: Into<UpdateSpotMarketOracleKeys>,
    A: Into<UpdateSpotMarketOracleIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketOracleKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketOracleIxArgs = args.into();
    let data: UpdateSpotMarketOracleIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_oracle_invoke<'a, A: Into<UpdateSpotMarketOracleIxArgs>>(
    accounts: &UpdateSpotMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_oracle_invoke_signed<'a, A: Into<UpdateSpotMarketOracleIxArgs>>(
    accounts: &UpdateSpotMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStepSizeAndTickSizeAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStepSizeAndTickSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketStepSizeAndTickSizeAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketStepSizeAndTickSizeKeys
{
    fn from(accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketStepSizeAndTickSizeKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketStepSizeAndTickSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketStepSizeAndTickSizeIxArgs {
    pub step_size: u64,
    pub tick_size: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketStepSizeAndTickSizeIxData<'me>(
    pub &'me UpdateSpotMarketStepSizeAndTickSizeIxArgs,
);
pub const UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM: [u8; 8] =
    [238, 153, 137, 80, 206, 59, 250, 61];
impl<'me> From<&'me UpdateSpotMarketStepSizeAndTickSizeIxArgs>
    for UpdateSpotMarketStepSizeAndTickSizeIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketStepSizeAndTickSizeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketStepSizeAndTickSizeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_step_size_and_tick_size_ix<
    K: Into<UpdateSpotMarketStepSizeAndTickSizeKeys>,
    A: Into<UpdateSpotMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketStepSizeAndTickSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdateSpotMarketStepSizeAndTickSizeIxArgs = args.into();
    let data: UpdateSpotMarketStepSizeAndTickSizeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_step_size_and_tick_size_invoke<
    'a,
    A: Into<UpdateSpotMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_step_size_and_tick_size_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_SPOT_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMinOrderSizeAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMinOrderSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketMinOrderSizeAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketMinOrderSizeKeys
{
    fn from(accounts: &UpdateSpotMarketMinOrderSizeAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketMinOrderSizeKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketMinOrderSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketMinOrderSizeIxArgs {
    pub order_size: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketMinOrderSizeIxData<'me>(pub &'me UpdateSpotMarketMinOrderSizeIxArgs);
pub const UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM: [u8; 8] = [93, 128, 11, 119, 26, 20, 181, 50];
impl<'me> From<&'me UpdateSpotMarketMinOrderSizeIxArgs>
    for UpdateSpotMarketMinOrderSizeIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketMinOrderSizeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketMinOrderSizeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_min_order_size_ix<
    K: Into<UpdateSpotMarketMinOrderSizeKeys>,
    A: Into<UpdateSpotMarketMinOrderSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketMinOrderSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketMinOrderSizeIxArgs = args.into();
    let data: UpdateSpotMarketMinOrderSizeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_min_order_size_invoke<'a, A: Into<UpdateSpotMarketMinOrderSizeIxArgs>>(
    accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_min_order_size_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketMinOrderSizeIxArgs>,
>(
    accounts: &UpdateSpotMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOrdersEnabledAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOrdersEnabledKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketOrdersEnabledAccounts<'me, '_, '_, '_>>
    for UpdateSpotMarketOrdersEnabledKeys
{
    fn from(accounts: &UpdateSpotMarketOrdersEnabledAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketOrdersEnabledKeys>
    for [AccountMeta; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotMarketOrdersEnabledKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketOrdersEnabledAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketOrdersEnabledIxArgs {
    pub orders_enabled: bool,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketOrdersEnabledIxData<'me>(pub &'me UpdateSpotMarketOrdersEnabledIxArgs);
pub const UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM: [u8; 8] =
    [190, 79, 206, 15, 26, 229, 229, 43];
impl<'me> From<&'me UpdateSpotMarketOrdersEnabledIxArgs>
    for UpdateSpotMarketOrdersEnabledIxData<'me>
{
    fn from(args: &'me UpdateSpotMarketOrdersEnabledIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketOrdersEnabledIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_orders_enabled_ix<
    K: Into<UpdateSpotMarketOrdersEnabledKeys>,
    A: Into<UpdateSpotMarketOrdersEnabledIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketOrdersEnabledKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketOrdersEnabledIxArgs = args.into();
    let data: UpdateSpotMarketOrdersEnabledIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_orders_enabled_invoke<
    'a,
    A: Into<UpdateSpotMarketOrdersEnabledIxArgs>,
>(
    accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_orders_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_orders_enabled_invoke_signed<
    'a,
    A: Into<UpdateSpotMarketOrdersEnabledIxArgs>,
>(
    accounts: &UpdateSpotMarketOrdersEnabledAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_orders_enabled_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_ORDERS_ENABLED_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketNameAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketNameKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
}
impl<'me> From<&UpdateSpotMarketNameAccounts<'me, '_, '_, '_>> for UpdateSpotMarketNameKeys {
    fn from(accounts: &UpdateSpotMarketNameAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
        }
    }
}
impl From<&UpdateSpotMarketNameKeys> for [AccountMeta; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateSpotMarketNameKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.spot_market, false),
        ]
    }
}
impl<'a> From<&UpdateSpotMarketNameAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotMarketNameAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotMarketNameIxArgs {
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotMarketNameIxData<'me>(pub &'me UpdateSpotMarketNameIxArgs);
pub const UPDATE_SPOT_MARKET_NAME_IX_DISCM: [u8; 8] = [17, 208, 1, 1, 162, 211, 188, 224];
impl<'me> From<&'me UpdateSpotMarketNameIxArgs> for UpdateSpotMarketNameIxData<'me> {
    fn from(args: &'me UpdateSpotMarketNameIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotMarketNameIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_MARKET_NAME_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_market_name_ix<
    K: Into<UpdateSpotMarketNameKeys>,
    A: Into<UpdateSpotMarketNameIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotMarketNameKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotMarketNameIxArgs = args.into();
    let data: UpdateSpotMarketNameIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_market_name_invoke<'a, A: Into<UpdateSpotMarketNameIxArgs>>(
    accounts: &UpdateSpotMarketNameAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_market_name_invoke_signed<'a, A: Into<UpdateSpotMarketNameIxArgs>>(
    accounts: &UpdateSpotMarketNameAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_MARKET_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStatusAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStatusKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketStatusAccounts<'me, '_, '_, '_>> for UpdatePerpMarketStatusKeys {
    fn from(accounts: &UpdatePerpMarketStatusAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketStatusKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketStatusAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketStatusAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketStatusIxArgs {
    pub status: MarketStatus,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStatusIxData<'me>(pub &'me UpdatePerpMarketStatusIxArgs);
pub const UPDATE_PERP_MARKET_STATUS_IX_DISCM: [u8; 8] = [71, 201, 175, 122, 255, 207, 196, 207];
impl<'me> From<&'me UpdatePerpMarketStatusIxArgs> for UpdatePerpMarketStatusIxData<'me> {
    fn from(args: &'me UpdatePerpMarketStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketStatusIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_STATUS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_status_ix<
    K: Into<UpdatePerpMarketStatusKeys>,
    A: Into<UpdatePerpMarketStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketStatusIxArgs = args.into();
    let data: UpdatePerpMarketStatusIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_status_invoke<'a, A: Into<UpdatePerpMarketStatusIxArgs>>(
    accounts: &UpdatePerpMarketStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_status_invoke_signed<'a, A: Into<UpdatePerpMarketStatusIxArgs>>(
    accounts: &UpdatePerpMarketStatusAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_STATUS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketContractTierAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketContractTierKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketContractTierAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketContractTierKeys
{
    fn from(accounts: &UpdatePerpMarketContractTierAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketContractTierKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketContractTierKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketContractTierAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketContractTierAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketContractTierIxArgs {
    pub contract_tier: ContractTier,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketContractTierIxData<'me>(pub &'me UpdatePerpMarketContractTierIxArgs);
pub const UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM: [u8; 8] =
    [236, 128, 15, 95, 203, 214, 68, 117];
impl<'me> From<&'me UpdatePerpMarketContractTierIxArgs>
    for UpdatePerpMarketContractTierIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketContractTierIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketContractTierIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_CONTRACT_TIER_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_contract_tier_ix<
    K: Into<UpdatePerpMarketContractTierKeys>,
    A: Into<UpdatePerpMarketContractTierIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketContractTierKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketContractTierIxArgs = args.into();
    let data: UpdatePerpMarketContractTierIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_contract_tier_invoke<'a, A: Into<UpdatePerpMarketContractTierIxArgs>>(
    accounts: &UpdatePerpMarketContractTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_contract_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_contract_tier_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketContractTierIxArgs>,
>(
    accounts: &UpdatePerpMarketContractTierAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_contract_tier_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_CONTRACT_TIER_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketImfFactorAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketImfFactorKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketImfFactorAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketImfFactorKeys
{
    fn from(accounts: &UpdatePerpMarketImfFactorAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketImfFactorKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketImfFactorKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketImfFactorAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketImfFactorAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketImfFactorIxArgs {
    pub imf_factor: u32,
    pub unrealized_pnl_imf_factor: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketImfFactorIxData<'me>(pub &'me UpdatePerpMarketImfFactorIxArgs);
pub const UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM: [u8; 8] = [207, 194, 56, 132, 35, 67, 71, 244];
impl<'me> From<&'me UpdatePerpMarketImfFactorIxArgs> for UpdatePerpMarketImfFactorIxData<'me> {
    fn from(args: &'me UpdatePerpMarketImfFactorIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketImfFactorIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_IMF_FACTOR_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_imf_factor_ix<
    K: Into<UpdatePerpMarketImfFactorKeys>,
    A: Into<UpdatePerpMarketImfFactorIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketImfFactorKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketImfFactorIxArgs = args.into();
    let data: UpdatePerpMarketImfFactorIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_imf_factor_invoke<'a, A: Into<UpdatePerpMarketImfFactorIxArgs>>(
    accounts: &UpdatePerpMarketImfFactorAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_imf_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_imf_factor_invoke_signed<'a, A: Into<UpdatePerpMarketImfFactorIxArgs>>(
    accounts: &UpdatePerpMarketImfFactorAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_imf_factor_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_IMF_FACTOR_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketUnrealizedAssetWeightAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketUnrealizedAssetWeightKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketUnrealizedAssetWeightAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketUnrealizedAssetWeightKeys
{
    fn from(accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketUnrealizedAssetWeightKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketUnrealizedAssetWeightKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketUnrealizedAssetWeightIxArgs {
    pub unrealized_initial_asset_weight: u32,
    pub unrealized_maintenance_asset_weight: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketUnrealizedAssetWeightIxData<'me>(
    pub &'me UpdatePerpMarketUnrealizedAssetWeightIxArgs,
);
pub const UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM: [u8; 8] =
    [135, 132, 205, 165, 109, 150, 166, 106];
impl<'me> From<&'me UpdatePerpMarketUnrealizedAssetWeightIxArgs>
    for UpdatePerpMarketUnrealizedAssetWeightIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketUnrealizedAssetWeightIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketUnrealizedAssetWeightIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_unrealized_asset_weight_ix<
    K: Into<UpdatePerpMarketUnrealizedAssetWeightKeys>,
    A: Into<UpdatePerpMarketUnrealizedAssetWeightIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketUnrealizedAssetWeightKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketUnrealizedAssetWeightIxArgs = args.into();
    let data: UpdatePerpMarketUnrealizedAssetWeightIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_unrealized_asset_weight_invoke<
    'a,
    A: Into<UpdatePerpMarketUnrealizedAssetWeightIxArgs>,
>(
    accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_unrealized_asset_weight_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_unrealized_asset_weight_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketUnrealizedAssetWeightIxArgs>,
>(
    accounts: &UpdatePerpMarketUnrealizedAssetWeightAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_unrealized_asset_weight_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_UNREALIZED_ASSET_WEIGHT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketConcentrationCoefAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketConcentrationCoefKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketConcentrationCoefAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketConcentrationCoefKeys
{
    fn from(accounts: &UpdatePerpMarketConcentrationCoefAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketConcentrationCoefKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketConcentrationCoefKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketConcentrationCoefAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketConcentrationCoefIxArgs {
    pub concentration_scale: u128,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketConcentrationCoefIxData<'me>(
    pub &'me UpdatePerpMarketConcentrationCoefIxArgs,
);
pub const UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM: [u8; 8] =
    [24, 78, 232, 126, 169, 176, 230, 16];
impl<'me> From<&'me UpdatePerpMarketConcentrationCoefIxArgs>
    for UpdatePerpMarketConcentrationCoefIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketConcentrationCoefIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketConcentrationCoefIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_concentration_coef_ix<
    K: Into<UpdatePerpMarketConcentrationCoefKeys>,
    A: Into<UpdatePerpMarketConcentrationCoefIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketConcentrationCoefKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketConcentrationCoefIxArgs = args.into();
    let data: UpdatePerpMarketConcentrationCoefIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_concentration_coef_invoke<
    'a,
    A: Into<UpdatePerpMarketConcentrationCoefIxArgs>,
>(
    accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_concentration_coef_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_concentration_coef_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketConcentrationCoefIxArgs>,
>(
    accounts: &UpdatePerpMarketConcentrationCoefAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_concentration_coef_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_CONCENTRATION_COEF_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketCurveUpdateIntensityAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketCurveUpdateIntensityKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketCurveUpdateIntensityAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketCurveUpdateIntensityKeys
{
    fn from(accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketCurveUpdateIntensityKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketCurveUpdateIntensityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketCurveUpdateIntensityIxArgs {
    pub curve_update_intensity: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketCurveUpdateIntensityIxData<'me>(
    pub &'me UpdatePerpMarketCurveUpdateIntensityIxArgs,
);
pub const UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM: [u8; 8] =
    [50, 131, 6, 156, 226, 231, 189, 72];
impl<'me> From<&'me UpdatePerpMarketCurveUpdateIntensityIxArgs>
    for UpdatePerpMarketCurveUpdateIntensityIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketCurveUpdateIntensityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketCurveUpdateIntensityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_curve_update_intensity_ix<
    K: Into<UpdatePerpMarketCurveUpdateIntensityKeys>,
    A: Into<UpdatePerpMarketCurveUpdateIntensityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketCurveUpdateIntensityKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketCurveUpdateIntensityIxArgs = args.into();
    let data: UpdatePerpMarketCurveUpdateIntensityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_curve_update_intensity_invoke<
    'a,
    A: Into<UpdatePerpMarketCurveUpdateIntensityIxArgs>,
>(
    accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_curve_update_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_curve_update_intensity_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketCurveUpdateIntensityIxArgs>,
>(
    accounts: &UpdatePerpMarketCurveUpdateIntensityAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_curve_update_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_CURVE_UPDATE_INTENSITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketTargetBaseAssetAmountPerLpKeys
{
    fn from(
        accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'me, '_, '_, '_>,
    ) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketTargetBaseAssetAmountPerLpKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketTargetBaseAssetAmountPerLpKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs {
    pub target_base_asset_amount_per_lp: i32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketTargetBaseAssetAmountPerLpIxData<'me>(
    pub &'me UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs,
);
pub const UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM: [u8; 8] =
    [62, 87, 68, 115, 29, 150, 150, 165];
impl<'me> From<&'me UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>
    for UpdatePerpMarketTargetBaseAssetAmountPerLpIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketTargetBaseAssetAmountPerLpIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_target_base_asset_amount_per_lp_ix<
    K: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpKeys>,
    A: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketTargetBaseAssetAmountPerLpKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs = args.into();
    let data: UpdatePerpMarketTargetBaseAssetAmountPerLpIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_target_base_asset_amount_per_lp_invoke<
    'a,
    A: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>,
>(
    accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_target_base_asset_amount_per_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_target_base_asset_amount_per_lp_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketTargetBaseAssetAmountPerLpIxArgs>,
>(
    accounts: &UpdatePerpMarketTargetBaseAssetAmountPerLpAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_target_base_asset_amount_per_lp_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_TARGET_BASE_ASSET_AMOUNT_PER_LP_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLpCooldownTimeAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateLpCooldownTimeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateLpCooldownTimeAccounts<'me, '_, '_>> for UpdateLpCooldownTimeKeys {
    fn from(accounts: &UpdateLpCooldownTimeAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateLpCooldownTimeKeys> for [AccountMeta; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateLpCooldownTimeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateLpCooldownTimeAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateLpCooldownTimeAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateLpCooldownTimeIxArgs {
    pub lp_cooldown_time: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateLpCooldownTimeIxData<'me>(pub &'me UpdateLpCooldownTimeIxArgs);
pub const UPDATE_LP_COOLDOWN_TIME_IX_DISCM: [u8; 8] = [198, 133, 88, 41, 241, 119, 61, 14];
impl<'me> From<&'me UpdateLpCooldownTimeIxArgs> for UpdateLpCooldownTimeIxData<'me> {
    fn from(args: &'me UpdateLpCooldownTimeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateLpCooldownTimeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LP_COOLDOWN_TIME_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_lp_cooldown_time_ix<
    K: Into<UpdateLpCooldownTimeKeys>,
    A: Into<UpdateLpCooldownTimeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateLpCooldownTimeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateLpCooldownTimeIxArgs = args.into();
    let data: UpdateLpCooldownTimeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_lp_cooldown_time_invoke<'a, A: Into<UpdateLpCooldownTimeIxArgs>>(
    accounts: &UpdateLpCooldownTimeAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_lp_cooldown_time_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_lp_cooldown_time_invoke_signed<'a, A: Into<UpdateLpCooldownTimeIxArgs>>(
    accounts: &UpdateLpCooldownTimeAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_lp_cooldown_time_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_LP_COOLDOWN_TIME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpFeeStructureAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpFeeStructureKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdatePerpFeeStructureAccounts<'me, '_, '_>> for UpdatePerpFeeStructureKeys {
    fn from(accounts: &UpdatePerpFeeStructureAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdatePerpFeeStructureKeys>
    for [AccountMeta; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpFeeStructureKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdatePerpFeeStructureAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpFeeStructureAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpFeeStructureIxArgs {
    pub fee_structure: FeeStructure,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpFeeStructureIxData<'me>(pub &'me UpdatePerpFeeStructureIxArgs);
pub const UPDATE_PERP_FEE_STRUCTURE_IX_DISCM: [u8; 8] = [23, 178, 111, 203, 73, 22, 140, 75];
impl<'me> From<&'me UpdatePerpFeeStructureIxArgs> for UpdatePerpFeeStructureIxData<'me> {
    fn from(args: &'me UpdatePerpFeeStructureIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpFeeStructureIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_FEE_STRUCTURE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_fee_structure_ix<
    K: Into<UpdatePerpFeeStructureKeys>,
    A: Into<UpdatePerpFeeStructureIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpFeeStructureKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpFeeStructureIxArgs = args.into();
    let data: UpdatePerpFeeStructureIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_fee_structure_invoke<'a, A: Into<UpdatePerpFeeStructureIxArgs>>(
    accounts: &UpdatePerpFeeStructureAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_fee_structure_invoke_signed<'a, A: Into<UpdatePerpFeeStructureIxArgs>>(
    accounts: &UpdatePerpFeeStructureAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotFeeStructureAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotFeeStructureKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateSpotFeeStructureAccounts<'me, '_, '_>> for UpdateSpotFeeStructureKeys {
    fn from(accounts: &UpdateSpotFeeStructureAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateSpotFeeStructureKeys>
    for [AccountMeta; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotFeeStructureKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateSpotFeeStructureAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotFeeStructureAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotFeeStructureIxArgs {
    pub fee_structure: FeeStructure,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotFeeStructureIxData<'me>(pub &'me UpdateSpotFeeStructureIxArgs);
pub const UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM: [u8; 8] = [97, 216, 105, 131, 113, 246, 142, 141];
impl<'me> From<&'me UpdateSpotFeeStructureIxArgs> for UpdateSpotFeeStructureIxData<'me> {
    fn from(args: &'me UpdateSpotFeeStructureIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotFeeStructureIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_FEE_STRUCTURE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_fee_structure_ix<
    K: Into<UpdateSpotFeeStructureKeys>,
    A: Into<UpdateSpotFeeStructureIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotFeeStructureKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotFeeStructureIxArgs = args.into();
    let data: UpdateSpotFeeStructureIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_fee_structure_invoke<'a, A: Into<UpdateSpotFeeStructureIxArgs>>(
    accounts: &UpdateSpotFeeStructureAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_fee_structure_invoke_signed<'a, A: Into<UpdateSpotFeeStructureIxArgs>>(
    accounts: &UpdateSpotFeeStructureAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_fee_structure_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_FEE_STRUCTURE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateInitialPctToLiquidateAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateInitialPctToLiquidateKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateInitialPctToLiquidateAccounts<'me, '_, '_>>
    for UpdateInitialPctToLiquidateKeys
{
    fn from(accounts: &UpdateInitialPctToLiquidateAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateInitialPctToLiquidateKeys>
    for [AccountMeta; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateInitialPctToLiquidateKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateInitialPctToLiquidateAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateInitialPctToLiquidateAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateInitialPctToLiquidateIxArgs {
    pub initial_pct_to_liquidate: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateInitialPctToLiquidateIxData<'me>(pub &'me UpdateInitialPctToLiquidateIxArgs);
pub const UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM: [u8; 8] =
    [210, 133, 225, 128, 194, 50, 13, 109];
impl<'me> From<&'me UpdateInitialPctToLiquidateIxArgs> for UpdateInitialPctToLiquidateIxData<'me> {
    fn from(args: &'me UpdateInitialPctToLiquidateIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateInitialPctToLiquidateIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_initial_pct_to_liquidate_ix<
    K: Into<UpdateInitialPctToLiquidateKeys>,
    A: Into<UpdateInitialPctToLiquidateIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateInitialPctToLiquidateKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateInitialPctToLiquidateIxArgs = args.into();
    let data: UpdateInitialPctToLiquidateIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_initial_pct_to_liquidate_invoke<'a, A: Into<UpdateInitialPctToLiquidateIxArgs>>(
    accounts: &UpdateInitialPctToLiquidateAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_initial_pct_to_liquidate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_initial_pct_to_liquidate_invoke_signed<
    'a,
    A: Into<UpdateInitialPctToLiquidateIxArgs>,
>(
    accounts: &UpdateInitialPctToLiquidateAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_initial_pct_to_liquidate_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_INITIAL_PCT_TO_LIQUIDATE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateLiquidationDurationAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateLiquidationDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateLiquidationDurationAccounts<'me, '_, '_>> for UpdateLiquidationDurationKeys {
    fn from(accounts: &UpdateLiquidationDurationAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateLiquidationDurationKeys>
    for [AccountMeta; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateLiquidationDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateLiquidationDurationAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateLiquidationDurationAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateLiquidationDurationIxArgs {
    pub liquidation_duration: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateLiquidationDurationIxData<'me>(pub &'me UpdateLiquidationDurationIxArgs);
pub const UPDATE_LIQUIDATION_DURATION_IX_DISCM: [u8; 8] = [28, 154, 20, 249, 102, 192, 73, 71];
impl<'me> From<&'me UpdateLiquidationDurationIxArgs> for UpdateLiquidationDurationIxData<'me> {
    fn from(args: &'me UpdateLiquidationDurationIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateLiquidationDurationIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_LIQUIDATION_DURATION_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_liquidation_duration_ix<
    K: Into<UpdateLiquidationDurationKeys>,
    A: Into<UpdateLiquidationDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateLiquidationDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateLiquidationDurationIxArgs = args.into();
    let data: UpdateLiquidationDurationIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_liquidation_duration_invoke<'a, A: Into<UpdateLiquidationDurationIxArgs>>(
    accounts: &UpdateLiquidationDurationAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_liquidation_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_liquidation_duration_invoke_signed<'a, A: Into<UpdateLiquidationDurationIxArgs>>(
    accounts: &UpdateLiquidationDurationAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_liquidation_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_LIQUIDATION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleGuardRailsAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleGuardRailsKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateOracleGuardRailsAccounts<'me, '_, '_>> for UpdateOracleGuardRailsKeys {
    fn from(accounts: &UpdateOracleGuardRailsAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateOracleGuardRailsKeys>
    for [AccountMeta; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateOracleGuardRailsKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateOracleGuardRailsAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateOracleGuardRailsAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateOracleGuardRailsIxArgs {
    pub oracle_guard_rails: OracleGuardRails,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateOracleGuardRailsIxData<'me>(pub &'me UpdateOracleGuardRailsIxArgs);
pub const UPDATE_ORACLE_GUARD_RAILS_IX_DISCM: [u8; 8] = [131, 112, 10, 59, 32, 54, 40, 164];
impl<'me> From<&'me UpdateOracleGuardRailsIxArgs> for UpdateOracleGuardRailsIxData<'me> {
    fn from(args: &'me UpdateOracleGuardRailsIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateOracleGuardRailsIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ORACLE_GUARD_RAILS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_oracle_guard_rails_ix<
    K: Into<UpdateOracleGuardRailsKeys>,
    A: Into<UpdateOracleGuardRailsIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateOracleGuardRailsKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateOracleGuardRailsIxArgs = args.into();
    let data: UpdateOracleGuardRailsIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_oracle_guard_rails_invoke<'a, A: Into<UpdateOracleGuardRailsIxArgs>>(
    accounts: &UpdateOracleGuardRailsAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_oracle_guard_rails_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_oracle_guard_rails_invoke_signed<'a, A: Into<UpdateOracleGuardRailsIxArgs>>(
    accounts: &UpdateOracleGuardRailsAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_oracle_guard_rails_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_ORACLE_GUARD_RAILS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateStateSettlementDurationAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateStateSettlementDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateStateSettlementDurationAccounts<'me, '_, '_>>
    for UpdateStateSettlementDurationKeys
{
    fn from(accounts: &UpdateStateSettlementDurationAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateStateSettlementDurationKeys>
    for [AccountMeta; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateStateSettlementDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateStateSettlementDurationAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateStateSettlementDurationAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateStateSettlementDurationIxArgs {
    pub settlement_duration: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateStateSettlementDurationIxData<'me>(pub &'me UpdateStateSettlementDurationIxArgs);
pub const UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM: [u8; 8] = [97, 68, 199, 235, 131, 80, 61, 173];
impl<'me> From<&'me UpdateStateSettlementDurationIxArgs>
    for UpdateStateSettlementDurationIxData<'me>
{
    fn from(args: &'me UpdateStateSettlementDurationIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateStateSettlementDurationIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_STATE_SETTLEMENT_DURATION_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_state_settlement_duration_ix<
    K: Into<UpdateStateSettlementDurationKeys>,
    A: Into<UpdateStateSettlementDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateStateSettlementDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateStateSettlementDurationIxArgs = args.into();
    let data: UpdateStateSettlementDurationIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_state_settlement_duration_invoke<'a, A: Into<UpdateStateSettlementDurationIxArgs>>(
    accounts: &UpdateStateSettlementDurationAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_state_settlement_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_state_settlement_duration_invoke_signed<
    'a,
    A: Into<UpdateStateSettlementDurationIxArgs>,
>(
    accounts: &UpdateStateSettlementDurationAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_state_settlement_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_STATE_SETTLEMENT_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketOracleAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub state: &'me AccountInfo<'a0>,
    pub perp_market: &'me AccountInfo<'a1>,
    pub oracle: &'me AccountInfo<'a2>,
    pub admin: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketOracleKeys {
    pub state: Pubkey,
    pub perp_market: Pubkey,
    pub oracle: Pubkey,
    pub admin: Pubkey,
}
impl<'me> From<&UpdatePerpMarketOracleAccounts<'me, '_, '_, '_, '_>>
    for UpdatePerpMarketOracleKeys
{
    fn from(accounts: &UpdatePerpMarketOracleAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
            oracle: *accounts.oracle.key,
            admin: *accounts.admin.key,
        }
    }
}
impl From<&UpdatePerpMarketOracleKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketOracleKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
            AccountMeta::new_readonly(keys.oracle, false),
            AccountMeta::new_readonly(keys.admin, true),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.state.clone(),
            accounts.perp_market.clone(),
            accounts.oracle.clone(),
            accounts.admin.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketOracleIxArgs {
    pub oracle: Pubkey,
    pub oracle_source: OracleSource,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketOracleIxData<'me>(pub &'me UpdatePerpMarketOracleIxArgs);
pub const UPDATE_PERP_MARKET_ORACLE_IX_DISCM: [u8; 8] = [182, 113, 111, 160, 67, 174, 89, 191];
impl<'me> From<&'me UpdatePerpMarketOracleIxArgs> for UpdatePerpMarketOracleIxData<'me> {
    fn from(args: &'me UpdatePerpMarketOracleIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketOracleIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_ORACLE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_oracle_ix<
    K: Into<UpdatePerpMarketOracleKeys>,
    A: Into<UpdatePerpMarketOracleIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketOracleKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketOracleIxArgs = args.into();
    let data: UpdatePerpMarketOracleIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_oracle_invoke<'a, A: Into<UpdatePerpMarketOracleIxArgs>>(
    accounts: &UpdatePerpMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_oracle_invoke_signed<'a, A: Into<UpdatePerpMarketOracleIxArgs>>(
    accounts: &UpdatePerpMarketOracleAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_oracle_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_ORACLE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketBaseSpreadAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketBaseSpreadKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketBaseSpreadAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketBaseSpreadKeys
{
    fn from(accounts: &UpdatePerpMarketBaseSpreadAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketBaseSpreadKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketBaseSpreadKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketBaseSpreadAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketBaseSpreadIxArgs {
    pub base_spread: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketBaseSpreadIxData<'me>(pub &'me UpdatePerpMarketBaseSpreadIxArgs);
pub const UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM: [u8; 8] = [71, 95, 84, 168, 9, 157, 198, 65];
impl<'me> From<&'me UpdatePerpMarketBaseSpreadIxArgs> for UpdatePerpMarketBaseSpreadIxData<'me> {
    fn from(args: &'me UpdatePerpMarketBaseSpreadIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketBaseSpreadIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_BASE_SPREAD_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_base_spread_ix<
    K: Into<UpdatePerpMarketBaseSpreadKeys>,
    A: Into<UpdatePerpMarketBaseSpreadIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketBaseSpreadKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketBaseSpreadIxArgs = args.into();
    let data: UpdatePerpMarketBaseSpreadIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_base_spread_invoke<'a, A: Into<UpdatePerpMarketBaseSpreadIxArgs>>(
    accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_base_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_base_spread_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketBaseSpreadIxArgs>,
>(
    accounts: &UpdatePerpMarketBaseSpreadAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_base_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_BASE_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmJitIntensityAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmJitIntensityKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdateAmmJitIntensityAccounts<'me, '_, '_, '_>> for UpdateAmmJitIntensityKeys {
    fn from(accounts: &UpdateAmmJitIntensityAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdateAmmJitIntensityKeys> for [AccountMeta; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateAmmJitIntensityKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdateAmmJitIntensityAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateAmmJitIntensityAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateAmmJitIntensityIxArgs {
    pub amm_jit_intensity: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAmmJitIntensityIxData<'me>(pub &'me UpdateAmmJitIntensityIxArgs);
pub const UPDATE_AMM_JIT_INTENSITY_IX_DISCM: [u8; 8] = [181, 191, 53, 109, 166, 249, 55, 142];
impl<'me> From<&'me UpdateAmmJitIntensityIxArgs> for UpdateAmmJitIntensityIxData<'me> {
    fn from(args: &'me UpdateAmmJitIntensityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateAmmJitIntensityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_AMM_JIT_INTENSITY_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_amm_jit_intensity_ix<
    K: Into<UpdateAmmJitIntensityKeys>,
    A: Into<UpdateAmmJitIntensityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateAmmJitIntensityKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateAmmJitIntensityIxArgs = args.into();
    let data: UpdateAmmJitIntensityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_amm_jit_intensity_invoke<'a, A: Into<UpdateAmmJitIntensityIxArgs>>(
    accounts: &UpdateAmmJitIntensityAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_amm_jit_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_amm_jit_intensity_invoke_signed<'a, A: Into<UpdateAmmJitIntensityIxArgs>>(
    accounts: &UpdateAmmJitIntensityAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_amm_jit_intensity_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_AMM_JIT_INTENSITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSpreadAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSpreadKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMaxSpreadAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMaxSpreadKeys
{
    fn from(accounts: &UpdatePerpMarketMaxSpreadAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxSpreadKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxSpreadKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMaxSpreadAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMaxSpreadIxArgs {
    pub max_spread: u32,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSpreadIxData<'me>(pub &'me UpdatePerpMarketMaxSpreadIxArgs);
pub const UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM: [u8; 8] = [80, 252, 122, 62, 40, 218, 91, 100];
impl<'me> From<&'me UpdatePerpMarketMaxSpreadIxArgs> for UpdatePerpMarketMaxSpreadIxData<'me> {
    fn from(args: &'me UpdatePerpMarketMaxSpreadIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMaxSpreadIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_SPREAD_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_max_spread_ix<
    K: Into<UpdatePerpMarketMaxSpreadKeys>,
    A: Into<UpdatePerpMarketMaxSpreadIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxSpreadKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMaxSpreadIxArgs = args.into();
    let data: UpdatePerpMarketMaxSpreadIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_spread_invoke<'a, A: Into<UpdatePerpMarketMaxSpreadIxArgs>>(
    accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_spread_invoke_signed<'a, A: Into<UpdatePerpMarketMaxSpreadIxArgs>>(
    accounts: &UpdatePerpMarketMaxSpreadAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_spread_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_SPREAD_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStepSizeAndTickSizeAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStepSizeAndTickSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketStepSizeAndTickSizeAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketStepSizeAndTickSizeKeys
{
    fn from(accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketStepSizeAndTickSizeKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketStepSizeAndTickSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketStepSizeAndTickSizeIxArgs {
    pub step_size: u64,
    pub tick_size: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketStepSizeAndTickSizeIxData<'me>(
    pub &'me UpdatePerpMarketStepSizeAndTickSizeIxArgs,
);
pub const UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM: [u8; 8] =
    [231, 255, 97, 25, 146, 139, 174, 4];
impl<'me> From<&'me UpdatePerpMarketStepSizeAndTickSizeIxArgs>
    for UpdatePerpMarketStepSizeAndTickSizeIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketStepSizeAndTickSizeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketStepSizeAndTickSizeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_step_size_and_tick_size_ix<
    K: Into<UpdatePerpMarketStepSizeAndTickSizeKeys>,
    A: Into<UpdatePerpMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketStepSizeAndTickSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketStepSizeAndTickSizeIxArgs = args.into();
    let data: UpdatePerpMarketStepSizeAndTickSizeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_step_size_and_tick_size_invoke<
    'a,
    A: Into<UpdatePerpMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_step_size_and_tick_size_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketStepSizeAndTickSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketStepSizeAndTickSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_step_size_and_tick_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_STEP_SIZE_AND_TICK_SIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketNameAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketNameKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketNameAccounts<'me, '_, '_, '_>> for UpdatePerpMarketNameKeys {
    fn from(accounts: &UpdatePerpMarketNameAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketNameKeys> for [AccountMeta; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdatePerpMarketNameKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketNameAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketNameAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketNameIxArgs {
    pub name: [u8; 32],
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketNameIxData<'me>(pub &'me UpdatePerpMarketNameIxArgs);
pub const UPDATE_PERP_MARKET_NAME_IX_DISCM: [u8; 8] = [211, 31, 21, 210, 64, 108, 66, 201];
impl<'me> From<&'me UpdatePerpMarketNameIxArgs> for UpdatePerpMarketNameIxData<'me> {
    fn from(args: &'me UpdatePerpMarketNameIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketNameIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_NAME_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_name_ix<
    K: Into<UpdatePerpMarketNameKeys>,
    A: Into<UpdatePerpMarketNameIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketNameKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketNameIxArgs = args.into();
    let data: UpdatePerpMarketNameIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_name_invoke<'a, A: Into<UpdatePerpMarketNameIxArgs>>(
    accounts: &UpdatePerpMarketNameAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_name_invoke_signed<'a, A: Into<UpdatePerpMarketNameIxArgs>>(
    accounts: &UpdatePerpMarketNameAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_name_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_NAME_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMinOrderSizeAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMinOrderSizeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMinOrderSizeAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMinOrderSizeKeys
{
    fn from(accounts: &UpdatePerpMarketMinOrderSizeAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMinOrderSizeKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMinOrderSizeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMinOrderSizeIxArgs {
    pub order_size: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMinOrderSizeIxData<'me>(pub &'me UpdatePerpMarketMinOrderSizeIxArgs);
pub const UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM: [u8; 8] = [226, 74, 5, 89, 108, 223, 46, 141];
impl<'me> From<&'me UpdatePerpMarketMinOrderSizeIxArgs>
    for UpdatePerpMarketMinOrderSizeIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketMinOrderSizeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMinOrderSizeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_min_order_size_ix<
    K: Into<UpdatePerpMarketMinOrderSizeKeys>,
    A: Into<UpdatePerpMarketMinOrderSizeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMinOrderSizeKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMinOrderSizeIxArgs = args.into();
    let data: UpdatePerpMarketMinOrderSizeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_min_order_size_invoke<'a, A: Into<UpdatePerpMarketMinOrderSizeIxArgs>>(
    accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_min_order_size_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketMinOrderSizeIxArgs>,
>(
    accounts: &UpdatePerpMarketMinOrderSizeAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_min_order_size_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MIN_ORDER_SIZE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSlippageRatioAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSlippageRatioKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMaxSlippageRatioAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMaxSlippageRatioKeys
{
    fn from(accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxSlippageRatioKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxSlippageRatioKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMaxSlippageRatioIxArgs {
    pub max_slippage_ratio: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxSlippageRatioIxData<'me>(
    pub &'me UpdatePerpMarketMaxSlippageRatioIxArgs,
);
pub const UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM: [u8; 8] =
    [235, 37, 40, 196, 70, 146, 54, 201];
impl<'me> From<&'me UpdatePerpMarketMaxSlippageRatioIxArgs>
    for UpdatePerpMarketMaxSlippageRatioIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketMaxSlippageRatioIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMaxSlippageRatioIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_max_slippage_ratio_ix<
    K: Into<UpdatePerpMarketMaxSlippageRatioKeys>,
    A: Into<UpdatePerpMarketMaxSlippageRatioIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxSlippageRatioKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketMaxSlippageRatioIxArgs = args.into();
    let data: UpdatePerpMarketMaxSlippageRatioIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_slippage_ratio_invoke<
    'a,
    A: Into<UpdatePerpMarketMaxSlippageRatioIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_slippage_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_slippage_ratio_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketMaxSlippageRatioIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxSlippageRatioAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_slippage_ratio_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_SLIPPAGE_RATIO_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxFillReserveFractionAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxFillReserveFractionKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMaxFillReserveFractionAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMaxFillReserveFractionKeys
{
    fn from(accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxFillReserveFractionKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxFillReserveFractionKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMaxFillReserveFractionIxArgs {
    pub max_fill_reserve_fraction: u16,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxFillReserveFractionIxData<'me>(
    pub &'me UpdatePerpMarketMaxFillReserveFractionIxArgs,
);
pub const UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM: [u8; 8] =
    [19, 172, 114, 154, 42, 135, 161, 133];
impl<'me> From<&'me UpdatePerpMarketMaxFillReserveFractionIxArgs>
    for UpdatePerpMarketMaxFillReserveFractionIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketMaxFillReserveFractionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMaxFillReserveFractionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_max_fill_reserve_fraction_ix<
    K: Into<UpdatePerpMarketMaxFillReserveFractionKeys>,
    A: Into<UpdatePerpMarketMaxFillReserveFractionIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxFillReserveFractionKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePerpMarketMaxFillReserveFractionIxArgs = args.into();
    let data: UpdatePerpMarketMaxFillReserveFractionIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_fill_reserve_fraction_invoke<
    'a,
    A: Into<UpdatePerpMarketMaxFillReserveFractionIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_fill_reserve_fraction_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_fill_reserve_fraction_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketMaxFillReserveFractionIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxFillReserveFractionAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_fill_reserve_fraction_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        UPDATE_PERP_MARKET_MAX_FILL_RESERVE_FRACTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxOpenInterestAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub perp_market: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxOpenInterestKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub perp_market: Pubkey,
}
impl<'me> From<&UpdatePerpMarketMaxOpenInterestAccounts<'me, '_, '_, '_>>
    for UpdatePerpMarketMaxOpenInterestKeys
{
    fn from(accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            perp_market: *accounts.perp_market.key,
        }
    }
}
impl From<&UpdatePerpMarketMaxOpenInterestKeys>
    for [AccountMeta; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpMarketMaxOpenInterestKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new(keys.perp_market, false),
        ]
    }
}
impl<'a> From<&UpdatePerpMarketMaxOpenInterestAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.perp_market.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpMarketMaxOpenInterestIxArgs {
    pub max_open_interest: u128,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpMarketMaxOpenInterestIxData<'me>(
    pub &'me UpdatePerpMarketMaxOpenInterestIxArgs,
);
pub const UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM: [u8; 8] =
    [194, 79, 149, 224, 246, 102, 186, 140];
impl<'me> From<&'me UpdatePerpMarketMaxOpenInterestIxArgs>
    for UpdatePerpMarketMaxOpenInterestIxData<'me>
{
    fn from(args: &'me UpdatePerpMarketMaxOpenInterestIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpMarketMaxOpenInterestIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_market_max_open_interest_ix<
    K: Into<UpdatePerpMarketMaxOpenInterestKeys>,
    A: Into<UpdatePerpMarketMaxOpenInterestIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpMarketMaxOpenInterestKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpMarketMaxOpenInterestIxArgs = args.into();
    let data: UpdatePerpMarketMaxOpenInterestIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_market_max_open_interest_invoke<
    'a,
    A: Into<UpdatePerpMarketMaxOpenInterestIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_market_max_open_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_market_max_open_interest_invoke_signed<
    'a,
    A: Into<UpdatePerpMarketMaxOpenInterestIxArgs>,
>(
    accounts: &UpdatePerpMarketMaxOpenInterestAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_market_max_open_interest_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_MARKET_MAX_OPEN_INTEREST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_ADMIN_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateAdminAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAdminKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateAdminAccounts<'me, '_, '_>> for UpdateAdminKeys {
    fn from(accounts: &UpdateAdminAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateAdminKeys> for [AccountMeta; UPDATE_ADMIN_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateAdminKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateAdminAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_ADMIN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateAdminAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateAdminIxArgs {
    pub admin: Pubkey,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateAdminIxData<'me>(pub &'me UpdateAdminIxArgs);
pub const UPDATE_ADMIN_IX_DISCM: [u8; 8] = [161, 176, 40, 213, 60, 184, 179, 228];
impl<'me> From<&'me UpdateAdminIxArgs> for UpdateAdminIxData<'me> {
    fn from(args: &'me UpdateAdminIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateAdminIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_ADMIN_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_admin_ix<K: Into<UpdateAdminKeys>, A: Into<UpdateAdminIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateAdminKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateAdminIxArgs = args.into();
    let data: UpdateAdminIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_admin_invoke<'a, A: Into<UpdateAdminIxArgs>>(
    accounts: &UpdateAdminAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_admin_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_admin_invoke_signed<'a, A: Into<UpdateAdminIxArgs>>(
    accounts: &UpdateAdminAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_admin_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_ADMIN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateWhitelistMintAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateWhitelistMintKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateWhitelistMintAccounts<'me, '_, '_>> for UpdateWhitelistMintKeys {
    fn from(accounts: &UpdateWhitelistMintAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateWhitelistMintKeys> for [AccountMeta; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateWhitelistMintKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateWhitelistMintAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateWhitelistMintAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateWhitelistMintIxArgs {
    pub whitelist_mint: Pubkey,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateWhitelistMintIxData<'me>(pub &'me UpdateWhitelistMintIxArgs);
pub const UPDATE_WHITELIST_MINT_IX_DISCM: [u8; 8] = [161, 15, 162, 19, 148, 120, 144, 151];
impl<'me> From<&'me UpdateWhitelistMintIxArgs> for UpdateWhitelistMintIxData<'me> {
    fn from(args: &'me UpdateWhitelistMintIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateWhitelistMintIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_WHITELIST_MINT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_whitelist_mint_ix<
    K: Into<UpdateWhitelistMintKeys>,
    A: Into<UpdateWhitelistMintIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateWhitelistMintKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateWhitelistMintIxArgs = args.into();
    let data: UpdateWhitelistMintIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_whitelist_mint_invoke<'a, A: Into<UpdateWhitelistMintIxArgs>>(
    accounts: &UpdateWhitelistMintAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_whitelist_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_whitelist_mint_invoke_signed<'a, A: Into<UpdateWhitelistMintIxArgs>>(
    accounts: &UpdateWhitelistMintAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_whitelist_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_WHITELIST_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateDiscountMintAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateDiscountMintKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateDiscountMintAccounts<'me, '_, '_>> for UpdateDiscountMintKeys {
    fn from(accounts: &UpdateDiscountMintAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateDiscountMintKeys> for [AccountMeta; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateDiscountMintKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateDiscountMintAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateDiscountMintAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateDiscountMintIxArgs {
    pub discount_mint: Pubkey,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateDiscountMintIxData<'me>(pub &'me UpdateDiscountMintIxArgs);
pub const UPDATE_DISCOUNT_MINT_IX_DISCM: [u8; 8] = [32, 252, 122, 211, 66, 31, 47, 241];
impl<'me> From<&'me UpdateDiscountMintIxArgs> for UpdateDiscountMintIxData<'me> {
    fn from(args: &'me UpdateDiscountMintIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateDiscountMintIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_DISCOUNT_MINT_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_discount_mint_ix<
    K: Into<UpdateDiscountMintKeys>,
    A: Into<UpdateDiscountMintIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateDiscountMintKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateDiscountMintIxArgs = args.into();
    let data: UpdateDiscountMintIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_discount_mint_invoke<'a, A: Into<UpdateDiscountMintIxArgs>>(
    accounts: &UpdateDiscountMintAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_discount_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_discount_mint_invoke_signed<'a, A: Into<UpdateDiscountMintIxArgs>>(
    accounts: &UpdateDiscountMintAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_discount_mint_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_DISCOUNT_MINT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateExchangeStatusAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateExchangeStatusKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateExchangeStatusAccounts<'me, '_, '_>> for UpdateExchangeStatusKeys {
    fn from(accounts: &UpdateExchangeStatusAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateExchangeStatusKeys> for [AccountMeta; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateExchangeStatusKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateExchangeStatusAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateExchangeStatusAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateExchangeStatusIxArgs {
    pub exchange_status: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateExchangeStatusIxData<'me>(pub &'me UpdateExchangeStatusIxArgs);
pub const UPDATE_EXCHANGE_STATUS_IX_DISCM: [u8; 8] = [83, 160, 252, 250, 129, 116, 49, 223];
impl<'me> From<&'me UpdateExchangeStatusIxArgs> for UpdateExchangeStatusIxData<'me> {
    fn from(args: &'me UpdateExchangeStatusIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateExchangeStatusIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_EXCHANGE_STATUS_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_exchange_status_ix<
    K: Into<UpdateExchangeStatusKeys>,
    A: Into<UpdateExchangeStatusIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateExchangeStatusKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateExchangeStatusIxArgs = args.into();
    let data: UpdateExchangeStatusIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_exchange_status_invoke<'a, A: Into<UpdateExchangeStatusIxArgs>>(
    accounts: &UpdateExchangeStatusAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_exchange_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_exchange_status_invoke_signed<'a, A: Into<UpdateExchangeStatusIxArgs>>(
    accounts: &UpdateExchangeStatusAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_exchange_status_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_EXCHANGE_STATUS_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpAuctionDurationAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpAuctionDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdatePerpAuctionDurationAccounts<'me, '_, '_>> for UpdatePerpAuctionDurationKeys {
    fn from(accounts: &UpdatePerpAuctionDurationAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdatePerpAuctionDurationKeys>
    for [AccountMeta; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePerpAuctionDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdatePerpAuctionDurationAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePerpAuctionDurationAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePerpAuctionDurationIxArgs {
    pub min_perp_auction_duration: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePerpAuctionDurationIxData<'me>(pub &'me UpdatePerpAuctionDurationIxArgs);
pub const UPDATE_PERP_AUCTION_DURATION_IX_DISCM: [u8; 8] = [126, 110, 52, 174, 30, 206, 215, 90];
impl<'me> From<&'me UpdatePerpAuctionDurationIxArgs> for UpdatePerpAuctionDurationIxData<'me> {
    fn from(args: &'me UpdatePerpAuctionDurationIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePerpAuctionDurationIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_PERP_AUCTION_DURATION_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_perp_auction_duration_ix<
    K: Into<UpdatePerpAuctionDurationKeys>,
    A: Into<UpdatePerpAuctionDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePerpAuctionDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdatePerpAuctionDurationIxArgs = args.into();
    let data: UpdatePerpAuctionDurationIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_perp_auction_duration_invoke<'a, A: Into<UpdatePerpAuctionDurationIxArgs>>(
    accounts: &UpdatePerpAuctionDurationAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_perp_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_perp_auction_duration_invoke_signed<'a, A: Into<UpdatePerpAuctionDurationIxArgs>>(
    accounts: &UpdatePerpAuctionDurationAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_perp_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PERP_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotAuctionDurationAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotAuctionDurationKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
}
impl<'me> From<&UpdateSpotAuctionDurationAccounts<'me, '_, '_>> for UpdateSpotAuctionDurationKeys {
    fn from(accounts: &UpdateSpotAuctionDurationAccounts<'me, '_, '_>) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
        }
    }
}
impl From<&UpdateSpotAuctionDurationKeys>
    for [AccountMeta; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateSpotAuctionDurationKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new(keys.state, false),
        ]
    }
}
impl<'a> From<&UpdateSpotAuctionDurationAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateSpotAuctionDurationAccounts<'_, 'a, 'a>) -> Self {
        [accounts.admin.clone(), accounts.state.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateSpotAuctionDurationIxArgs {
    pub default_spot_auction_duration: u8,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateSpotAuctionDurationIxData<'me>(pub &'me UpdateSpotAuctionDurationIxArgs);
pub const UPDATE_SPOT_AUCTION_DURATION_IX_DISCM: [u8; 8] = [182, 178, 203, 72, 187, 143, 157, 107];
impl<'me> From<&'me UpdateSpotAuctionDurationIxArgs> for UpdateSpotAuctionDurationIxData<'me> {
    fn from(args: &'me UpdateSpotAuctionDurationIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateSpotAuctionDurationIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SPOT_AUCTION_DURATION_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn update_spot_auction_duration_ix<
    K: Into<UpdateSpotAuctionDurationKeys>,
    A: Into<UpdateSpotAuctionDurationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateSpotAuctionDurationKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateSpotAuctionDurationIxArgs = args.into();
    let data: UpdateSpotAuctionDurationIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_spot_auction_duration_invoke<'a, A: Into<UpdateSpotAuctionDurationIxArgs>>(
    accounts: &UpdateSpotAuctionDurationAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_spot_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_spot_auction_duration_invoke_signed<'a, A: Into<UpdateSpotAuctionDurationIxArgs>>(
    accounts: &UpdateSpotAuctionDurationAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_spot_auction_duration_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_SPOT_AUCTION_DURATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN: usize = 7usize;
#[derive(Copy, Clone, Debug)]
pub struct AdminRemoveInsuranceFundStakeAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub admin: &'me AccountInfo<'a0>,
    pub state: &'me AccountInfo<'a1>,
    pub spot_market: &'me AccountInfo<'a2>,
    pub insurance_fund_vault: &'me AccountInfo<'a3>,
    pub drift_signer: &'me AccountInfo<'a4>,
    pub admin_token_account: &'me AccountInfo<'a5>,
    pub token_program: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct AdminRemoveInsuranceFundStakeKeys {
    pub admin: Pubkey,
    pub state: Pubkey,
    pub spot_market: Pubkey,
    pub insurance_fund_vault: Pubkey,
    pub drift_signer: Pubkey,
    pub admin_token_account: Pubkey,
    pub token_program: Pubkey,
}
impl<'me> From<&AdminRemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for AdminRemoveInsuranceFundStakeKeys
{
    fn from(
        accounts: &AdminRemoveInsuranceFundStakeAccounts<'me, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            admin: *accounts.admin.key,
            state: *accounts.state.key,
            spot_market: *accounts.spot_market.key,
            insurance_fund_vault: *accounts.insurance_fund_vault.key,
            drift_signer: *accounts.drift_signer.key,
            admin_token_account: *accounts.admin_token_account.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&AdminRemoveInsuranceFundStakeKeys>
    for [AccountMeta; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(keys: &AdminRemoveInsuranceFundStakeKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.admin, true),
            AccountMeta::new_readonly(keys.state, false),
            AccountMeta::new_readonly(keys.spot_market, false),
            AccountMeta::new(keys.insurance_fund_vault, false),
            AccountMeta::new_readonly(keys.drift_signer, false),
            AccountMeta::new(keys.admin_token_account, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&AdminRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.admin.clone(),
            accounts.state.clone(),
            accounts.spot_market.clone(),
            accounts.insurance_fund_vault.clone(),
            accounts.drift_signer.clone(),
            accounts.admin_token_account.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct AdminRemoveInsuranceFundStakeIxArgs {
    pub market_index: u16,
    pub amount: u64,
}
#[derive(Copy, Clone, Debug)]
pub struct AdminRemoveInsuranceFundStakeIxData<'me>(pub &'me AdminRemoveInsuranceFundStakeIxArgs);
pub const ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM: [u8; 8] =
    [35, 13, 111, 220, 103, 217, 174, 115];
impl<'me> From<&'me AdminRemoveInsuranceFundStakeIxArgs>
    for AdminRemoveInsuranceFundStakeIxData<'me>
{
    fn from(args: &'me AdminRemoveInsuranceFundStakeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for AdminRemoveInsuranceFundStakeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_DISCM)?;
        self.0.serialize(writer)
    }
}
pub fn admin_remove_insurance_fund_stake_ix<
    K: Into<AdminRemoveInsuranceFundStakeKeys>,
    A: Into<AdminRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: AdminRemoveInsuranceFundStakeKeys = accounts.into();
    let metas: [AccountMeta; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: AdminRemoveInsuranceFundStakeIxArgs = args.into();
    let data: AdminRemoveInsuranceFundStakeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn admin_remove_insurance_fund_stake_invoke<
    'a,
    A: Into<AdminRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = admin_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn admin_remove_insurance_fund_stake_invoke_signed<
    'a,
    A: Into<AdminRemoveInsuranceFundStakeIxArgs>,
>(
    accounts: &AdminRemoveInsuranceFundStakeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = admin_remove_insurance_fund_stake_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; ADMIN_REMOVE_INSURANCE_FUND_STAKE_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
