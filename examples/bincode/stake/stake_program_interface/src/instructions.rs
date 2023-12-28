use crate::*;
use serde::{Deserialize, Serialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StakeProgramProgramIx {
    Initialize(InitializeIxArgs),
    Authorize(AuthorizeIxArgs),
    DelegateStake,
    Split(SplitIxArgs),
    Withdraw(WithdrawIxArgs),
    Deactivate,
    SetLockup(SetLockupIxArgs),
    Merge,
    AuthorizeWithSeed(AuthorizeWithSeedIxArgs),
    InitializeChecked,
    AuthorizeChecked(AuthorizeCheckedIxArgs),
    AuthorizeCheckedWithSeed(AuthorizeCheckedWithSeedIxArgs),
    SetLockupChecked(SetLockupCheckedIxArgs),
    GetMinimumDelegation,
    DeactivateDelinquent,
    Redelegate,
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_IX_DISCM: [u8; 4] = [0, 0, 0, 0];
pub fn initialize_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeKeys,
    args: InitializeIxArgs,
) -> Instruction {
    let metas: [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::Initialize(args),
        Vec::from(metas),
    )
}
pub fn initialize_ix(keys: InitializeKeys, args: InitializeIxArgs) -> Instruction {
    initialize_ix_with_program_id(crate::ID, keys, args)
}
pub const INITIALIZE_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct InitializeAccounts<'me, 'info> {
    ///The stake account to initialize
    pub stake: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeKeys {
    ///The stake account to initialize
    pub stake: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
}
impl From<InitializeAccounts<'_, '_>> for InitializeKeys {
    fn from(accounts: InitializeAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeKeys> for [AccountMeta; INITIALIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]> for InitializeKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            rent: pubkeys[1],
        }
    }
}
impl<'info> From<InitializeAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeAccounts<'_, 'info>) -> Self {
        [accounts.stake.clone(), accounts.rent.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]>
    for InitializeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            rent: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InitializeIxArgs {
    pub authorized: Authorized,
    pub lockup: Lockup,
}
pub fn initialize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn initialize_invoke(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
) -> ProgramResult {
    initialize_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeKeys = accounts.into();
    let ix = initialize_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_invoke_signed(
    accounts: InitializeAccounts<'_, '_>,
    args: InitializeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_verify_account_keys(
    accounts: InitializeAccounts<'_, '_>,
    keys: InitializeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_verify_writable_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_verify_account_privileges<'me, 'info>(
    accounts: InitializeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const AUTHORIZE_IX_DISCM: [u8; 4] = [1, 0, 0, 0];
pub fn authorize_ix_with_program_id(
    program_id: Pubkey,
    keys: AuthorizeKeys,
    args: AuthorizeIxArgs,
) -> Instruction {
    let metas: [AccountMeta; AUTHORIZE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::Authorize(args),
        Vec::from(metas),
    )
}
pub fn authorize_ix(keys: AuthorizeKeys, args: AuthorizeIxArgs) -> Instruction {
    authorize_ix_with_program_id(crate::ID, keys, args)
}
pub const AUTHORIZE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeAccounts<'me, 'info> {
    ///The stake account to be updated
    pub stake: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///stake's current stake or withdraw authority to change away from. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeKeys {
    ///The stake account to be updated
    pub stake: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///stake's current stake or withdraw authority to change away from. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer
    pub authority: Pubkey,
}
impl From<AuthorizeAccounts<'_, '_>> for AuthorizeKeys {
    fn from(accounts: AuthorizeAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            clock: *accounts.clock.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<AuthorizeKeys> for [AccountMeta; AUTHORIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: AuthorizeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; AUTHORIZE_IX_ACCOUNTS_LEN]> for AuthorizeKeys {
    fn from(pubkeys: [Pubkey; AUTHORIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            clock: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<AuthorizeAccounts<'_, 'info>> for [AccountInfo<'info>; AUTHORIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AuthorizeAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.clock.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; AUTHORIZE_IX_ACCOUNTS_LEN]>
    for AuthorizeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; AUTHORIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            clock: &arr[1],
            authority: &arr[2],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthorizeIxArgs {
    pub new_authority: Pubkey,
    pub stake_authorize: StakeAuthorize,
}
pub fn authorize_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeAccounts<'_, '_>,
    args: AuthorizeIxArgs,
) -> ProgramResult {
    let keys: AuthorizeKeys = accounts.into();
    let ix = authorize_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn authorize_invoke(
    accounts: AuthorizeAccounts<'_, '_>,
    args: AuthorizeIxArgs,
) -> ProgramResult {
    authorize_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn authorize_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeAccounts<'_, '_>,
    args: AuthorizeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AuthorizeKeys = accounts.into();
    let ix = authorize_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn authorize_invoke_signed(
    accounts: AuthorizeAccounts<'_, '_>,
    args: AuthorizeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    authorize_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn authorize_verify_account_keys(
    accounts: AuthorizeAccounts<'_, '_>,
    keys: AuthorizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.clock.key, &keys.clock),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn authorize_verify_writable_privileges<'me, 'info>(
    accounts: AuthorizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn authorize_verify_signer_privileges<'me, 'info>(
    accounts: AuthorizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn authorize_verify_account_privileges<'me, 'info>(
    accounts: AuthorizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    authorize_verify_writable_privileges(accounts)?;
    authorize_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DELEGATE_STAKE_IX_DISCM: [u8; 4] = [2, 0, 0, 0];
pub fn delegate_stake_ix_with_program_id(
    program_id: Pubkey,
    keys: DelegateStakeKeys,
) -> Instruction {
    let metas: [AccountMeta; DELEGATE_STAKE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::DelegateStake,
        Vec::from(metas),
    )
}
pub fn delegate_stake_ix(keys: DelegateStakeKeys) -> Instruction {
    delegate_stake_ix_with_program_id(crate::ID, keys)
}
pub const DELEGATE_STAKE_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct DelegateStakeAccounts<'me, 'info> {
    ///The stake account to be delegated
    pub stake: &'me AccountInfo<'info>,
    ///Vote account to which stake will be delegated
    pub vote: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///Stake config native program
    pub stake_config: &'me AccountInfo<'info>,
    ///stake's stake authority
    pub stake_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DelegateStakeKeys {
    ///The stake account to be delegated
    pub stake: Pubkey,
    ///Vote account to which stake will be delegated
    pub vote: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///Stake config native program
    pub stake_config: Pubkey,
    ///stake's stake authority
    pub stake_authority: Pubkey,
}
impl From<DelegateStakeAccounts<'_, '_>> for DelegateStakeKeys {
    fn from(accounts: DelegateStakeAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            vote: *accounts.vote.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_config: *accounts.stake_config.key,
            stake_authority: *accounts.stake_authority.key,
        }
    }
}
impl From<DelegateStakeKeys> for [AccountMeta; DELEGATE_STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: DelegateStakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DELEGATE_STAKE_IX_ACCOUNTS_LEN]> for DelegateStakeKeys {
    fn from(pubkeys: [Pubkey; DELEGATE_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            vote: pubkeys[1],
            clock: pubkeys[2],
            stake_history: pubkeys[3],
            stake_config: pubkeys[4],
            stake_authority: pubkeys[5],
        }
    }
}
impl<'info> From<DelegateStakeAccounts<'_, 'info>>
    for [AccountInfo<'info>; DELEGATE_STAKE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DelegateStakeAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.vote.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_config.clone(),
            accounts.stake_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DELEGATE_STAKE_IX_ACCOUNTS_LEN]>
    for DelegateStakeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DELEGATE_STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            vote: &arr[1],
            clock: &arr[2],
            stake_history: &arr[3],
            stake_config: &arr[4],
            stake_authority: &arr[5],
        }
    }
}
pub fn delegate_stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DelegateStakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DelegateStakeKeys = accounts.into();
    let ix = delegate_stake_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn delegate_stake_invoke(accounts: DelegateStakeAccounts<'_, '_>) -> ProgramResult {
    delegate_stake_invoke_with_program_id(crate::ID, accounts)
}
pub fn delegate_stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DelegateStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DelegateStakeKeys = accounts.into();
    let ix = delegate_stake_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn delegate_stake_invoke_signed(
    accounts: DelegateStakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    delegate_stake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn delegate_stake_verify_account_keys(
    accounts: DelegateStakeAccounts<'_, '_>,
    keys: DelegateStakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.vote.key, &keys.vote),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_config.key, &keys.stake_config),
        (accounts.stake_authority.key, &keys.stake_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn delegate_stake_verify_writable_privileges<'me, 'info>(
    accounts: DelegateStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn delegate_stake_verify_signer_privileges<'me, 'info>(
    accounts: DelegateStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.stake_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn delegate_stake_verify_account_privileges<'me, 'info>(
    accounts: DelegateStakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    delegate_stake_verify_writable_privileges(accounts)?;
    delegate_stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SPLIT_IX_DISCM: [u8; 4] = [3, 0, 0, 0];
pub fn split_ix_with_program_id(
    program_id: Pubkey,
    keys: SplitKeys,
    args: SplitIxArgs,
) -> Instruction {
    let metas: [AccountMeta; SPLIT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::Split(args),
        Vec::from(metas),
    )
}
pub fn split_ix(keys: SplitKeys, args: SplitIxArgs) -> Instruction {
    split_ix_with_program_id(crate::ID, keys, args)
}
pub const SPLIT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct SplitAccounts<'me, 'info> {
    ///The stake account to split. Must be in the Initialized or Stake state
    pub from: &'me AccountInfo<'info>,
    ///The uninitialized stake account to split to. Must be rent-exempt starting from solana 1.17.
    pub to: &'me AccountInfo<'info>,
    ///from's stake authority
    pub stake_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SplitKeys {
    ///The stake account to split. Must be in the Initialized or Stake state
    pub from: Pubkey,
    ///The uninitialized stake account to split to. Must be rent-exempt starting from solana 1.17.
    pub to: Pubkey,
    ///from's stake authority
    pub stake_authority: Pubkey,
}
impl From<SplitAccounts<'_, '_>> for SplitKeys {
    fn from(accounts: SplitAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            to: *accounts.to.key,
            stake_authority: *accounts.stake_authority.key,
        }
    }
}
impl From<SplitKeys> for [AccountMeta; SPLIT_IX_ACCOUNTS_LEN] {
    fn from(keys: SplitKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SPLIT_IX_ACCOUNTS_LEN]> for SplitKeys {
    fn from(pubkeys: [Pubkey; SPLIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            to: pubkeys[1],
            stake_authority: pubkeys[2],
        }
    }
}
impl<'info> From<SplitAccounts<'_, 'info>> for [AccountInfo<'info>; SPLIT_IX_ACCOUNTS_LEN] {
    fn from(accounts: SplitAccounts<'_, 'info>) -> Self {
        [
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.stake_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SPLIT_IX_ACCOUNTS_LEN]>
    for SplitAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SPLIT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            to: &arr[1],
            stake_authority: &arr[2],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SplitIxArgs {
    pub lamports: u64,
}
pub fn split_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SplitAccounts<'_, '_>,
    args: SplitIxArgs,
) -> ProgramResult {
    let keys: SplitKeys = accounts.into();
    let ix = split_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn split_invoke(accounts: SplitAccounts<'_, '_>, args: SplitIxArgs) -> ProgramResult {
    split_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn split_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SplitAccounts<'_, '_>,
    args: SplitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SplitKeys = accounts.into();
    let ix = split_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn split_invoke_signed(
    accounts: SplitAccounts<'_, '_>,
    args: SplitIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    split_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn split_verify_account_keys(
    accounts: SplitAccounts<'_, '_>,
    keys: SplitKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.from.key, &keys.from),
        (accounts.to.key, &keys.to),
        (accounts.stake_authority.key, &keys.stake_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn split_verify_writable_privileges<'me, 'info>(
    accounts: SplitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn split_verify_signer_privileges<'me, 'info>(
    accounts: SplitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.stake_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn split_verify_account_privileges<'me, 'info>(
    accounts: SplitAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    split_verify_writable_privileges(accounts)?;
    split_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_DISCM: [u8; 4] = [4, 0, 0, 0];
pub fn withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawKeys,
    args: WithdrawIxArgs,
) -> Instruction {
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::Withdraw(args),
        Vec::from(metas),
    )
}
pub fn withdraw_ix(keys: WithdrawKeys, args: WithdrawIxArgs) -> Instruction {
    withdraw_ix_with_program_id(crate::ID, keys, args)
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    ///The stake account to withdraw from
    pub from: &'me AccountInfo<'info>,
    ///Recipient account
    pub to: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///from's withdraw authority. If stake Lockup is active, the signing lockup authority must follow.
    pub withdraw_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawKeys {
    ///The stake account to withdraw from
    pub from: Pubkey,
    ///Recipient account
    pub to: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///from's withdraw authority. If stake Lockup is active, the signing lockup authority must follow.
    pub withdraw_authority: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            to: *accounts.to.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            withdraw_authority: *accounts.withdraw_authority.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            to: pubkeys[1],
            clock: pubkeys[2],
            stake_history: pubkeys[3],
            withdraw_authority: pubkeys[4],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>> for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.withdraw_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
    for WithdrawAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            to: &arr[1],
            clock: &arr[2],
            stake_history: &arr[3],
            withdraw_authority: &arr[4],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WithdrawIxArgs {
    pub lamports: u64,
}
pub fn withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_invoke(accounts: WithdrawAccounts<'_, '_>, args: WithdrawIxArgs) -> ProgramResult {
    withdraw_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_invoke_signed(
    accounts: WithdrawAccounts<'_, '_>,
    args: WithdrawIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.from.key, &keys.from),
        (accounts.to.key, &keys.to),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.withdraw_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_writable_privileges(accounts)?;
    withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DEACTIVATE_IX_DISCM: [u8; 4] = [5, 0, 0, 0];
pub fn deactivate_ix_with_program_id(program_id: Pubkey, keys: DeactivateKeys) -> Instruction {
    let metas: [AccountMeta; DEACTIVATE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::Deactivate,
        Vec::from(metas),
    )
}
pub fn deactivate_ix(keys: DeactivateKeys) -> Instruction {
    deactivate_ix_with_program_id(crate::ID, keys)
}
pub const DEACTIVATE_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DeactivateAccounts<'me, 'info> {
    ///The stake account to deactivate
    pub stake: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///stake's stake authority
    pub stake_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeactivateKeys {
    ///The stake account to deactivate
    pub stake: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///stake's stake authority
    pub stake_authority: Pubkey,
}
impl From<DeactivateAccounts<'_, '_>> for DeactivateKeys {
    fn from(accounts: DeactivateAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            clock: *accounts.clock.key,
            stake_authority: *accounts.stake_authority.key,
        }
    }
}
impl From<DeactivateKeys> for [AccountMeta; DEACTIVATE_IX_ACCOUNTS_LEN] {
    fn from(keys: DeactivateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEACTIVATE_IX_ACCOUNTS_LEN]> for DeactivateKeys {
    fn from(pubkeys: [Pubkey; DEACTIVATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            clock: pubkeys[1],
            stake_authority: pubkeys[2],
        }
    }
}
impl<'info> From<DeactivateAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEACTIVATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DeactivateAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.clock.clone(),
            accounts.stake_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEACTIVATE_IX_ACCOUNTS_LEN]>
    for DeactivateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEACTIVATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            clock: &arr[1],
            stake_authority: &arr[2],
        }
    }
}
pub fn deactivate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeactivateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeactivateKeys = accounts.into();
    let ix = deactivate_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn deactivate_invoke(accounts: DeactivateAccounts<'_, '_>) -> ProgramResult {
    deactivate_invoke_with_program_id(crate::ID, accounts)
}
pub fn deactivate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeactivateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeactivateKeys = accounts.into();
    let ix = deactivate_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deactivate_invoke_signed(
    accounts: DeactivateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deactivate_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn deactivate_verify_account_keys(
    accounts: DeactivateAccounts<'_, '_>,
    keys: DeactivateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_authority.key, &keys.stake_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deactivate_verify_writable_privileges<'me, 'info>(
    accounts: DeactivateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deactivate_verify_signer_privileges<'me, 'info>(
    accounts: DeactivateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.stake_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn deactivate_verify_account_privileges<'me, 'info>(
    accounts: DeactivateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deactivate_verify_writable_privileges(accounts)?;
    deactivate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LOCKUP_IX_DISCM: [u8; 4] = [6, 0, 0, 0];
pub fn set_lockup_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLockupKeys,
    args: SetLockupIxArgs,
) -> Instruction {
    let metas: [AccountMeta; SET_LOCKUP_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::SetLockup(args),
        Vec::from(metas),
    )
}
pub fn set_lockup_ix(keys: SetLockupKeys, args: SetLockupIxArgs) -> Instruction {
    set_lockup_ix_with_program_id(crate::ID, keys, args)
}
pub const SET_LOCKUP_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLockupAccounts<'me, 'info> {
    ///The stake account to set the lockup of
    pub stake: &'me AccountInfo<'info>,
    ///stake's withdraw authority or lockup authority if lockup is active
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetLockupKeys {
    ///The stake account to set the lockup of
    pub stake: Pubkey,
    ///stake's withdraw authority or lockup authority if lockup is active
    pub authority: Pubkey,
}
impl From<SetLockupAccounts<'_, '_>> for SetLockupKeys {
    fn from(accounts: SetLockupAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetLockupKeys> for [AccountMeta; SET_LOCKUP_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLockupKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LOCKUP_IX_ACCOUNTS_LEN]> for SetLockupKeys {
    fn from(pubkeys: [Pubkey; SET_LOCKUP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetLockupAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LOCKUP_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetLockupAccounts<'_, 'info>) -> Self {
        [accounts.stake.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LOCKUP_IX_ACCOUNTS_LEN]>
    for SetLockupAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LOCKUP_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SetLockupIxArgs {
    pub unix_timestamp: Option<i64>,
    pub epoch: Option<u64>,
    pub custodian: Option<Pubkey>,
}
pub fn set_lockup_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLockupAccounts<'_, '_>,
    args: SetLockupIxArgs,
) -> ProgramResult {
    let keys: SetLockupKeys = accounts.into();
    let ix = set_lockup_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn set_lockup_invoke(
    accounts: SetLockupAccounts<'_, '_>,
    args: SetLockupIxArgs,
) -> ProgramResult {
    set_lockup_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_lockup_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLockupAccounts<'_, '_>,
    args: SetLockupIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLockupKeys = accounts.into();
    let ix = set_lockup_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lockup_invoke_signed(
    accounts: SetLockupAccounts<'_, '_>,
    args: SetLockupIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lockup_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_lockup_verify_account_keys(
    accounts: SetLockupAccounts<'_, '_>,
    keys: SetLockupKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lockup_verify_writable_privileges<'me, 'info>(
    accounts: SetLockupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lockup_verify_signer_privileges<'me, 'info>(
    accounts: SetLockupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lockup_verify_account_privileges<'me, 'info>(
    accounts: SetLockupAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lockup_verify_writable_privileges(accounts)?;
    set_lockup_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const MERGE_IX_DISCM: [u8; 4] = [7, 0, 0, 0];
pub fn merge_ix_with_program_id(program_id: Pubkey, keys: MergeKeys) -> Instruction {
    let metas: [AccountMeta; MERGE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(program_id, &StakeProgramProgramIx::Merge, Vec::from(metas))
}
pub fn merge_ix(keys: MergeKeys) -> Instruction {
    merge_ix_with_program_id(crate::ID, keys)
}
pub const MERGE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct MergeAccounts<'me, 'info> {
    ///The destination stake account to merge into
    pub to: &'me AccountInfo<'info>,
    ///The stake account to merge from. Must have exact same lockup and authority as to. This account will be drained.
    pub from: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///Stake history sysvar
    pub stake_history: &'me AccountInfo<'info>,
    ///Both from and to's stake authority
    pub stake_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct MergeKeys {
    ///The destination stake account to merge into
    pub to: Pubkey,
    ///The stake account to merge from. Must have exact same lockup and authority as to. This account will be drained.
    pub from: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///Stake history sysvar
    pub stake_history: Pubkey,
    ///Both from and to's stake authority
    pub stake_authority: Pubkey,
}
impl From<MergeAccounts<'_, '_>> for MergeKeys {
    fn from(accounts: MergeAccounts) -> Self {
        Self {
            to: *accounts.to.key,
            from: *accounts.from.key,
            clock: *accounts.clock.key,
            stake_history: *accounts.stake_history.key,
            stake_authority: *accounts.stake_authority.key,
        }
    }
}
impl From<MergeKeys> for [AccountMeta; MERGE_IX_ACCOUNTS_LEN] {
    fn from(keys: MergeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_history,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; MERGE_IX_ACCOUNTS_LEN]> for MergeKeys {
    fn from(pubkeys: [Pubkey; MERGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            to: pubkeys[0],
            from: pubkeys[1],
            clock: pubkeys[2],
            stake_history: pubkeys[3],
            stake_authority: pubkeys[4],
        }
    }
}
impl<'info> From<MergeAccounts<'_, 'info>> for [AccountInfo<'info>; MERGE_IX_ACCOUNTS_LEN] {
    fn from(accounts: MergeAccounts<'_, 'info>) -> Self {
        [
            accounts.to.clone(),
            accounts.from.clone(),
            accounts.clock.clone(),
            accounts.stake_history.clone(),
            accounts.stake_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; MERGE_IX_ACCOUNTS_LEN]>
    for MergeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; MERGE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            to: &arr[0],
            from: &arr[1],
            clock: &arr[2],
            stake_history: &arr[3],
            stake_authority: &arr[4],
        }
    }
}
pub fn merge_invoke_with_program_id(
    program_id: Pubkey,
    accounts: MergeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: MergeKeys = accounts.into();
    let ix = merge_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn merge_invoke(accounts: MergeAccounts<'_, '_>) -> ProgramResult {
    merge_invoke_with_program_id(crate::ID, accounts)
}
pub fn merge_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: MergeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: MergeKeys = accounts.into();
    let ix = merge_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn merge_invoke_signed(accounts: MergeAccounts<'_, '_>, seeds: &[&[&[u8]]]) -> ProgramResult {
    merge_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn merge_verify_account_keys(
    accounts: MergeAccounts<'_, '_>,
    keys: MergeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.to.key, &keys.to),
        (accounts.from.key, &keys.from),
        (accounts.clock.key, &keys.clock),
        (accounts.stake_history.key, &keys.stake_history),
        (accounts.stake_authority.key, &keys.stake_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn merge_verify_writable_privileges<'me, 'info>(
    accounts: MergeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.to, accounts.from] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn merge_verify_signer_privileges<'me, 'info>(
    accounts: MergeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.stake_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn merge_verify_account_privileges<'me, 'info>(
    accounts: MergeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    merge_verify_writable_privileges(accounts)?;
    merge_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const AUTHORIZE_WITH_SEED_IX_DISCM: [u8; 4] = [8, 0, 0, 0];
pub fn authorize_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: AuthorizeWithSeedKeys,
    args: AuthorizeWithSeedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::AuthorizeWithSeed(args),
        Vec::from(metas),
    )
}
pub fn authorize_with_seed_ix(
    keys: AuthorizeWithSeedKeys,
    args: AuthorizeWithSeedIxArgs,
) -> Instruction {
    authorize_with_seed_ix_with_program_id(crate::ID, keys, args)
}
pub const AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeWithSeedAccounts<'me, 'info> {
    ///The stake account to be updated, with the authority to be updated being an account created with Pubkey::create_with_seed()
    pub stake: &'me AccountInfo<'info>,
    ///Base account of stake's authority to be updated
    pub authority_base: &'me AccountInfo<'info>,
    ///Clock sysvar. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub clock: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeWithSeedKeys {
    ///The stake account to be updated, with the authority to be updated being an account created with Pubkey::create_with_seed()
    pub stake: Pubkey,
    ///Base account of stake's authority to be updated
    pub authority_base: Pubkey,
    ///Clock sysvar. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub clock: Pubkey,
}
impl From<AuthorizeWithSeedAccounts<'_, '_>> for AuthorizeWithSeedKeys {
    fn from(accounts: AuthorizeWithSeedAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            authority_base: *accounts.authority_base.key,
            clock: *accounts.clock.key,
        }
    }
}
impl From<AuthorizeWithSeedKeys> for [AccountMeta; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: AuthorizeWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN]> for AuthorizeWithSeedKeys {
    fn from(pubkeys: [Pubkey; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            authority_base: pubkeys[1],
            clock: pubkeys[2],
        }
    }
}
impl<'info> From<AuthorizeWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AuthorizeWithSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.authority_base.clone(),
            accounts.clock.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN]>
    for AuthorizeWithSeedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; AUTHORIZE_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            authority_base: &arr[1],
            clock: &arr[2],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthorizeWithSeedIxArgs {
    pub new_authority: Pubkey,
    pub stake_authorize: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: Pubkey,
}
pub fn authorize_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeWithSeedAccounts<'_, '_>,
    args: AuthorizeWithSeedIxArgs,
) -> ProgramResult {
    let keys: AuthorizeWithSeedKeys = accounts.into();
    let ix = authorize_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn authorize_with_seed_invoke(
    accounts: AuthorizeWithSeedAccounts<'_, '_>,
    args: AuthorizeWithSeedIxArgs,
) -> ProgramResult {
    authorize_with_seed_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn authorize_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeWithSeedAccounts<'_, '_>,
    args: AuthorizeWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AuthorizeWithSeedKeys = accounts.into();
    let ix = authorize_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn authorize_with_seed_invoke_signed(
    accounts: AuthorizeWithSeedAccounts<'_, '_>,
    args: AuthorizeWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    authorize_with_seed_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn authorize_with_seed_verify_account_keys(
    accounts: AuthorizeWithSeedAccounts<'_, '_>,
    keys: AuthorizeWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.authority_base.key, &keys.authority_base),
        (accounts.clock.key, &keys.clock),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn authorize_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: AuthorizeWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn authorize_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: AuthorizeWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn authorize_with_seed_verify_account_privileges<'me, 'info>(
    accounts: AuthorizeWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    authorize_with_seed_verify_writable_privileges(accounts)?;
    authorize_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_CHECKED_IX_DISCM: [u8; 4] = [9, 0, 0, 0];
pub fn initialize_checked_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeCheckedKeys,
) -> Instruction {
    let metas: [AccountMeta; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::InitializeChecked,
        Vec::from(metas),
    )
}
pub fn initialize_checked_ix(keys: InitializeCheckedKeys) -> Instruction {
    initialize_checked_ix_with_program_id(crate::ID, keys)
}
pub const INITIALIZE_CHECKED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct InitializeCheckedAccounts<'me, 'info> {
    ///The stake account to initialize
    pub stake: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
    ///stake's new stake authority
    pub stake_authority: &'me AccountInfo<'info>,
    ///stake's new withdraw authority
    pub withdraw_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeCheckedKeys {
    ///The stake account to initialize
    pub stake: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
    ///stake's new stake authority
    pub stake_authority: Pubkey,
    ///stake's new withdraw authority
    pub withdraw_authority: Pubkey,
}
impl From<InitializeCheckedAccounts<'_, '_>> for InitializeCheckedKeys {
    fn from(accounts: InitializeCheckedAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            rent: *accounts.rent.key,
            stake_authority: *accounts.stake_authority.key,
            withdraw_authority: *accounts.withdraw_authority.key,
        }
    }
}
impl From<InitializeCheckedKeys> for [AccountMeta; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.withdraw_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN]> for InitializeCheckedKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            rent: pubkeys[1],
            stake_authority: pubkeys[2],
            withdraw_authority: pubkeys[3],
        }
    }
}
impl<'info> From<InitializeCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeCheckedAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.rent.clone(),
            accounts.stake_authority.clone(),
            accounts.withdraw_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN]>
    for InitializeCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            rent: &arr[1],
            stake_authority: &arr[2],
            withdraw_authority: &arr[3],
        }
    }
}
pub fn initialize_checked_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCheckedAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeCheckedKeys = accounts.into();
    let ix = initialize_checked_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn initialize_checked_invoke(accounts: InitializeCheckedAccounts<'_, '_>) -> ProgramResult {
    initialize_checked_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_checked_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeCheckedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeCheckedKeys = accounts.into();
    let ix = initialize_checked_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_checked_invoke_signed(
    accounts: InitializeCheckedAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_checked_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn initialize_checked_verify_account_keys(
    accounts: InitializeCheckedAccounts<'_, '_>,
    keys: InitializeCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.rent.key, &keys.rent),
        (accounts.stake_authority.key, &keys.stake_authority),
        (accounts.withdraw_authority.key, &keys.withdraw_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_checked_verify_writable_privileges<'me, 'info>(
    accounts: InitializeCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_checked_verify_signer_privileges<'me, 'info>(
    accounts: InitializeCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.withdraw_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_checked_verify_account_privileges<'me, 'info>(
    accounts: InitializeCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_checked_verify_writable_privileges(accounts)?;
    initialize_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const AUTHORIZE_CHECKED_IX_DISCM: [u8; 4] = [10, 0, 0, 0];
pub fn authorize_checked_ix_with_program_id(
    program_id: Pubkey,
    keys: AuthorizeCheckedKeys,
    args: AuthorizeCheckedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::AuthorizeChecked(args),
        Vec::from(metas),
    )
}
pub fn authorize_checked_ix(
    keys: AuthorizeCheckedKeys,
    args: AuthorizeCheckedIxArgs,
) -> Instruction {
    authorize_checked_ix_with_program_id(crate::ID, keys, args)
}
pub const AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeCheckedAccounts<'me, 'info> {
    ///The stake account to be updated
    pub stake: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///stake's current stake or withdraw authority to change away from
    pub authority: &'me AccountInfo<'info>,
    ///stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeCheckedKeys {
    ///The stake account to be updated
    pub stake: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///stake's current stake or withdraw authority to change away from
    pub authority: Pubkey,
    ///stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: Pubkey,
}
impl From<AuthorizeCheckedAccounts<'_, '_>> for AuthorizeCheckedKeys {
    fn from(accounts: AuthorizeCheckedAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            clock: *accounts.clock.key,
            authority: *accounts.authority.key,
            new_authority: *accounts.new_authority.key,
        }
    }
}
impl From<AuthorizeCheckedKeys> for [AccountMeta; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: AuthorizeCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN]> for AuthorizeCheckedKeys {
    fn from(pubkeys: [Pubkey; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            clock: pubkeys[1],
            authority: pubkeys[2],
            new_authority: pubkeys[3],
        }
    }
}
impl<'info> From<AuthorizeCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AuthorizeCheckedAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.clock.clone(),
            accounts.authority.clone(),
            accounts.new_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN]>
    for AuthorizeCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; AUTHORIZE_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            clock: &arr[1],
            authority: &arr[2],
            new_authority: &arr[3],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthorizeCheckedIxArgs {
    pub stake_authorize: StakeAuthorize,
}
pub fn authorize_checked_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeCheckedAccounts<'_, '_>,
    args: AuthorizeCheckedIxArgs,
) -> ProgramResult {
    let keys: AuthorizeCheckedKeys = accounts.into();
    let ix = authorize_checked_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn authorize_checked_invoke(
    accounts: AuthorizeCheckedAccounts<'_, '_>,
    args: AuthorizeCheckedIxArgs,
) -> ProgramResult {
    authorize_checked_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn authorize_checked_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeCheckedAccounts<'_, '_>,
    args: AuthorizeCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AuthorizeCheckedKeys = accounts.into();
    let ix = authorize_checked_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn authorize_checked_invoke_signed(
    accounts: AuthorizeCheckedAccounts<'_, '_>,
    args: AuthorizeCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    authorize_checked_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn authorize_checked_verify_account_keys(
    accounts: AuthorizeCheckedAccounts<'_, '_>,
    keys: AuthorizeCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.clock.key, &keys.clock),
        (accounts.authority.key, &keys.authority),
        (accounts.new_authority.key, &keys.new_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn authorize_checked_verify_writable_privileges<'me, 'info>(
    accounts: AuthorizeCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn authorize_checked_verify_signer_privileges<'me, 'info>(
    accounts: AuthorizeCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority, accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn authorize_checked_verify_account_privileges<'me, 'info>(
    accounts: AuthorizeCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    authorize_checked_verify_writable_privileges(accounts)?;
    authorize_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const AUTHORIZE_CHECKED_WITH_SEED_IX_DISCM: [u8; 4] = [11, 0, 0, 0];
pub fn authorize_checked_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: AuthorizeCheckedWithSeedKeys,
    args: AuthorizeCheckedWithSeedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::AuthorizeCheckedWithSeed(args),
        Vec::from(metas),
    )
}
pub fn authorize_checked_with_seed_ix(
    keys: AuthorizeCheckedWithSeedKeys,
    args: AuthorizeCheckedWithSeedIxArgs,
) -> Instruction {
    authorize_checked_with_seed_ix_with_program_id(crate::ID, keys, args)
}
pub const AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeCheckedWithSeedAccounts<'me, 'info> {
    ///The stake account to be updated
    pub stake: &'me AccountInfo<'info>,
    ///Base account of stake's authority to be updated
    pub authority_base: &'me AccountInfo<'info>,
    ///Clock sysvar
    pub clock: &'me AccountInfo<'info>,
    ///stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeCheckedWithSeedKeys {
    ///The stake account to be updated
    pub stake: Pubkey,
    ///Base account of stake's authority to be updated
    pub authority_base: Pubkey,
    ///Clock sysvar
    pub clock: Pubkey,
    ///stake's new stake or withdraw authority to change to. If stake Lockup is active, the signing lockup authority must follow if updating withdrawer.
    pub new_authority: Pubkey,
}
impl From<AuthorizeCheckedWithSeedAccounts<'_, '_>> for AuthorizeCheckedWithSeedKeys {
    fn from(accounts: AuthorizeCheckedWithSeedAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            authority_base: *accounts.authority_base.key,
            clock: *accounts.clock.key,
            new_authority: *accounts.new_authority.key,
        }
    }
}
impl From<AuthorizeCheckedWithSeedKeys>
    for [AccountMeta; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(keys: AuthorizeCheckedWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority_base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.clock,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.new_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN]> for AuthorizeCheckedWithSeedKeys {
    fn from(pubkeys: [Pubkey; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            authority_base: pubkeys[1],
            clock: pubkeys[2],
            new_authority: pubkeys[3],
        }
    }
}
impl<'info> From<AuthorizeCheckedWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AuthorizeCheckedWithSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.authority_base.clone(),
            accounts.clock.clone(),
            accounts.new_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN]>
    for AuthorizeCheckedWithSeedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; AUTHORIZE_CHECKED_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            authority_base: &arr[1],
            clock: &arr[2],
            new_authority: &arr[3],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthorizeCheckedWithSeedIxArgs {
    pub stake_authorize: StakeAuthorize,
    pub authority_seed: String,
    pub authority_owner: Pubkey,
}
pub fn authorize_checked_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeCheckedWithSeedAccounts<'_, '_>,
    args: AuthorizeCheckedWithSeedIxArgs,
) -> ProgramResult {
    let keys: AuthorizeCheckedWithSeedKeys = accounts.into();
    let ix = authorize_checked_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn authorize_checked_with_seed_invoke(
    accounts: AuthorizeCheckedWithSeedAccounts<'_, '_>,
    args: AuthorizeCheckedWithSeedIxArgs,
) -> ProgramResult {
    authorize_checked_with_seed_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn authorize_checked_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeCheckedWithSeedAccounts<'_, '_>,
    args: AuthorizeCheckedWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AuthorizeCheckedWithSeedKeys = accounts.into();
    let ix = authorize_checked_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn authorize_checked_with_seed_invoke_signed(
    accounts: AuthorizeCheckedWithSeedAccounts<'_, '_>,
    args: AuthorizeCheckedWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    authorize_checked_with_seed_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn authorize_checked_with_seed_verify_account_keys(
    accounts: AuthorizeCheckedWithSeedAccounts<'_, '_>,
    keys: AuthorizeCheckedWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.authority_base.key, &keys.authority_base),
        (accounts.clock.key, &keys.clock),
        (accounts.new_authority.key, &keys.new_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn authorize_checked_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: AuthorizeCheckedWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn authorize_checked_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: AuthorizeCheckedWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority_base, accounts.new_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn authorize_checked_with_seed_verify_account_privileges<'me, 'info>(
    accounts: AuthorizeCheckedWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    authorize_checked_with_seed_verify_writable_privileges(accounts)?;
    authorize_checked_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const SET_LOCKUP_CHECKED_IX_DISCM: [u8; 4] = [12, 0, 0, 0];
pub fn set_lockup_checked_ix_with_program_id(
    program_id: Pubkey,
    keys: SetLockupCheckedKeys,
    args: SetLockupCheckedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::SetLockupChecked(args),
        Vec::from(metas),
    )
}
pub fn set_lockup_checked_ix(
    keys: SetLockupCheckedKeys,
    args: SetLockupCheckedIxArgs,
) -> Instruction {
    set_lockup_checked_ix_with_program_id(crate::ID, keys, args)
}
pub const SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SetLockupCheckedAccounts<'me, 'info> {
    ///The stake account to set the lockup of
    pub stake: &'me AccountInfo<'info>,
    ///stake's withdraw authority or lockup authority if lockup is active. If setting a new lockup authority, the signing new lockup authority must follow.
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetLockupCheckedKeys {
    ///The stake account to set the lockup of
    pub stake: Pubkey,
    ///stake's withdraw authority or lockup authority if lockup is active. If setting a new lockup authority, the signing new lockup authority must follow.
    pub authority: Pubkey,
}
impl From<SetLockupCheckedAccounts<'_, '_>> for SetLockupCheckedKeys {
    fn from(accounts: SetLockupCheckedAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<SetLockupCheckedKeys> for [AccountMeta; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN] {
    fn from(keys: SetLockupCheckedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN]> for SetLockupCheckedKeys {
    fn from(pubkeys: [Pubkey; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<SetLockupCheckedAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: SetLockupCheckedAccounts<'_, 'info>) -> Self {
        [accounts.stake.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN]>
    for SetLockupCheckedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_LOCKUP_CHECKED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct SetLockupCheckedIxArgs {
    pub unix_timestamp: Option<i64>,
    pub epoch: Option<u64>,
}
pub fn set_lockup_checked_invoke_with_program_id(
    program_id: Pubkey,
    accounts: SetLockupCheckedAccounts<'_, '_>,
    args: SetLockupCheckedIxArgs,
) -> ProgramResult {
    let keys: SetLockupCheckedKeys = accounts.into();
    let ix = set_lockup_checked_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn set_lockup_checked_invoke(
    accounts: SetLockupCheckedAccounts<'_, '_>,
    args: SetLockupCheckedIxArgs,
) -> ProgramResult {
    set_lockup_checked_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn set_lockup_checked_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: SetLockupCheckedAccounts<'_, '_>,
    args: SetLockupCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: SetLockupCheckedKeys = accounts.into();
    let ix = set_lockup_checked_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn set_lockup_checked_invoke_signed(
    accounts: SetLockupCheckedAccounts<'_, '_>,
    args: SetLockupCheckedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    set_lockup_checked_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn set_lockup_checked_verify_account_keys(
    accounts: SetLockupCheckedAccounts<'_, '_>,
    keys: SetLockupCheckedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_lockup_checked_verify_writable_privileges<'me, 'info>(
    accounts: SetLockupCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn set_lockup_checked_verify_signer_privileges<'me, 'info>(
    accounts: SetLockupCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn set_lockup_checked_verify_account_privileges<'me, 'info>(
    accounts: SetLockupCheckedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    set_lockup_checked_verify_writable_privileges(accounts)?;
    set_lockup_checked_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const GET_MINIMUM_DELEGATION_IX_DISCM: [u8; 4] = [13, 0, 0, 0];
pub fn get_minimum_delegation_ix_with_program_id(program_id: Pubkey) -> Instruction {
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::GetMinimumDelegation,
        Vec::new(),
    )
}
pub fn get_minimum_delegation_ix() -> Instruction {
    get_minimum_delegation_ix_with_program_id(crate::ID)
}
pub fn get_minimum_delegation_invoke_with_program_id(program_id: Pubkey) -> ProgramResult {
    let ix = get_minimum_delegation_ix_with_program_id(program_id);
    invoke(&ix, &[])
}
pub fn get_minimum_delegation_invoke() -> ProgramResult {
    get_minimum_delegation_invoke_with_program_id(crate::ID)
}
pub fn get_minimum_delegation_invoke_signed_with_program_id(
    program_id: Pubkey,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = get_minimum_delegation_ix_with_program_id(program_id);
    invoke_signed(&ix, &[], seeds)
}
pub fn get_minimum_delegation_invoke_signed(seeds: &[&[&[u8]]]) -> ProgramResult {
    get_minimum_delegation_invoke_signed_with_program_id(crate::ID, seeds)
}
pub const DEACTIVATE_DELINQUENT_IX_DISCM: [u8; 4] = [14, 0, 0, 0];
pub fn deactivate_delinquent_ix_with_program_id(
    program_id: Pubkey,
    keys: DeactivateDelinquentKeys,
) -> Instruction {
    let metas: [AccountMeta; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::DeactivateDelinquent,
        Vec::from(metas),
    )
}
pub fn deactivate_delinquent_ix(keys: DeactivateDelinquentKeys) -> Instruction {
    deactivate_delinquent_ix_with_program_id(crate::ID, keys)
}
pub const DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DeactivateDelinquentAccounts<'me, 'info> {
    ///The delinquent stake account to deactivate
    pub stake: &'me AccountInfo<'info>,
    ///stake's delinquent vote account
    pub vote: &'me AccountInfo<'info>,
    ///Reference vote account that has voted at least once in the last MINIMUM_DELINQUENT_EPOCHS_FOR_DEACTIVATION epochs
    pub reference_vote: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeactivateDelinquentKeys {
    ///The delinquent stake account to deactivate
    pub stake: Pubkey,
    ///stake's delinquent vote account
    pub vote: Pubkey,
    ///Reference vote account that has voted at least once in the last MINIMUM_DELINQUENT_EPOCHS_FOR_DEACTIVATION epochs
    pub reference_vote: Pubkey,
}
impl From<DeactivateDelinquentAccounts<'_, '_>> for DeactivateDelinquentKeys {
    fn from(accounts: DeactivateDelinquentAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            vote: *accounts.vote.key,
            reference_vote: *accounts.reference_vote.key,
        }
    }
}
impl From<DeactivateDelinquentKeys> for [AccountMeta; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN] {
    fn from(keys: DeactivateDelinquentKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.reference_vote,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN]> for DeactivateDelinquentKeys {
    fn from(pubkeys: [Pubkey; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            vote: pubkeys[1],
            reference_vote: pubkeys[2],
        }
    }
}
impl<'info> From<DeactivateDelinquentAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: DeactivateDelinquentAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.vote.clone(),
            accounts.reference_vote.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN]>
    for DeactivateDelinquentAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; DEACTIVATE_DELINQUENT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            vote: &arr[1],
            reference_vote: &arr[2],
        }
    }
}
pub fn deactivate_delinquent_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DeactivateDelinquentAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DeactivateDelinquentKeys = accounts.into();
    let ix = deactivate_delinquent_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn deactivate_delinquent_invoke(
    accounts: DeactivateDelinquentAccounts<'_, '_>,
) -> ProgramResult {
    deactivate_delinquent_invoke_with_program_id(crate::ID, accounts)
}
pub fn deactivate_delinquent_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DeactivateDelinquentAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DeactivateDelinquentKeys = accounts.into();
    let ix = deactivate_delinquent_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn deactivate_delinquent_invoke_signed(
    accounts: DeactivateDelinquentAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    deactivate_delinquent_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn deactivate_delinquent_verify_account_keys(
    accounts: DeactivateDelinquentAccounts<'_, '_>,
    keys: DeactivateDelinquentKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.vote.key, &keys.vote),
        (accounts.reference_vote.key, &keys.reference_vote),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deactivate_delinquent_verify_writable_privileges<'me, 'info>(
    accounts: DeactivateDelinquentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn deactivate_delinquent_verify_account_privileges<'me, 'info>(
    accounts: DeactivateDelinquentAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    deactivate_delinquent_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const REDELEGATE_IX_DISCM: [u8; 4] = [15, 0, 0, 0];
pub fn redelegate_ix_with_program_id(program_id: Pubkey, keys: RedelegateKeys) -> Instruction {
    let metas: [AccountMeta; REDELEGATE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &StakeProgramProgramIx::Redelegate,
        Vec::from(metas),
    )
}
pub fn redelegate_ix(keys: RedelegateKeys) -> Instruction {
    redelegate_ix_with_program_id(crate::ID, keys)
}
pub const REDELEGATE_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct RedelegateAccounts<'me, 'info> {
    ///The delegated stake account to be redelegated. The account must be fully activated and carry a balance greater than or equal to the minimum delegation amount plus rent exempt minimum
    pub stake: &'me AccountInfo<'info>,
    ///Uninitialized stake account that will hold the redelegated stake
    pub uninitialized_stake: &'me AccountInfo<'info>,
    ///Vote account to which stake will be redelegated
    pub vote: &'me AccountInfo<'info>,
    ///Stake config native program
    pub stake_config: &'me AccountInfo<'info>,
    ///stake's stake authority
    pub stake_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RedelegateKeys {
    ///The delegated stake account to be redelegated. The account must be fully activated and carry a balance greater than or equal to the minimum delegation amount plus rent exempt minimum
    pub stake: Pubkey,
    ///Uninitialized stake account that will hold the redelegated stake
    pub uninitialized_stake: Pubkey,
    ///Vote account to which stake will be redelegated
    pub vote: Pubkey,
    ///Stake config native program
    pub stake_config: Pubkey,
    ///stake's stake authority
    pub stake_authority: Pubkey,
}
impl From<RedelegateAccounts<'_, '_>> for RedelegateKeys {
    fn from(accounts: RedelegateAccounts) -> Self {
        Self {
            stake: *accounts.stake.key,
            uninitialized_stake: *accounts.uninitialized_stake.key,
            vote: *accounts.vote.key,
            stake_config: *accounts.stake_config.key,
            stake_authority: *accounts.stake_authority.key,
        }
    }
}
impl From<RedelegateKeys> for [AccountMeta; REDELEGATE_IX_ACCOUNTS_LEN] {
    fn from(keys: RedelegateKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.uninitialized_stake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vote,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_config,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_authority,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REDELEGATE_IX_ACCOUNTS_LEN]> for RedelegateKeys {
    fn from(pubkeys: [Pubkey; REDELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: pubkeys[0],
            uninitialized_stake: pubkeys[1],
            vote: pubkeys[2],
            stake_config: pubkeys[3],
            stake_authority: pubkeys[4],
        }
    }
}
impl<'info> From<RedelegateAccounts<'_, 'info>>
    for [AccountInfo<'info>; REDELEGATE_IX_ACCOUNTS_LEN]
{
    fn from(accounts: RedelegateAccounts<'_, 'info>) -> Self {
        [
            accounts.stake.clone(),
            accounts.uninitialized_stake.clone(),
            accounts.vote.clone(),
            accounts.stake_config.clone(),
            accounts.stake_authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REDELEGATE_IX_ACCOUNTS_LEN]>
    for RedelegateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REDELEGATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            stake: &arr[0],
            uninitialized_stake: &arr[1],
            vote: &arr[2],
            stake_config: &arr[3],
            stake_authority: &arr[4],
        }
    }
}
pub fn redelegate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RedelegateAccounts<'_, '_>,
) -> ProgramResult {
    let keys: RedelegateKeys = accounts.into();
    let ix = redelegate_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn redelegate_invoke(accounts: RedelegateAccounts<'_, '_>) -> ProgramResult {
    redelegate_invoke_with_program_id(crate::ID, accounts)
}
pub fn redelegate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RedelegateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RedelegateKeys = accounts.into();
    let ix = redelegate_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn redelegate_invoke_signed(
    accounts: RedelegateAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    redelegate_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn redelegate_verify_account_keys(
    accounts: RedelegateAccounts<'_, '_>,
    keys: RedelegateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.stake.key, &keys.stake),
        (accounts.uninitialized_stake.key, &keys.uninitialized_stake),
        (accounts.vote.key, &keys.vote),
        (accounts.stake_config.key, &keys.stake_config),
        (accounts.stake_authority.key, &keys.stake_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn redelegate_verify_writable_privileges<'me, 'info>(
    accounts: RedelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.stake, accounts.uninitialized_stake] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn redelegate_verify_signer_privileges<'me, 'info>(
    accounts: RedelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.stake_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn redelegate_verify_account_privileges<'me, 'info>(
    accounts: RedelegateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    redelegate_verify_writable_privileges(accounts)?;
    redelegate_verify_signer_privileges(accounts)?;
    Ok(())
}
