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
pub enum SystemProgramProgramIx {
    CreateAccount(CreateAccountIxArgs),
    Assign(AssignIxArgs),
    Transfer(TransferIxArgs),
    CreateAccountWithSeed(CreateAccountWithSeedIxArgs),
    AdvanceNonceAccount,
    WithdrawNonceAccount(WithdrawNonceAccountIxArgs),
    InitializeNonceAccount(InitializeNonceAccountIxArgs),
    AuthorizeNonceAccount(AuthorizeNonceAccountIxArgs),
    Allocate(AllocateIxArgs),
    AllocateWithSeed(AllocateWithSeedIxArgs),
    AssignWithSeed(AssignWithSeedIxArgs),
    TransferWithSeed(TransferWithSeedIxArgs),
    UpgradeNonceAccount,
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
pub const CREATE_ACCOUNT_IX_DISCM: [u8; 4] = [0, 0, 0, 0];
pub fn create_account_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAccountKeys,
    args: CreateAccountIxArgs,
) -> Instruction {
    let metas: [AccountMeta; CREATE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::CreateAccount(args),
        Vec::from(metas),
    )
}
pub fn create_account_ix(keys: CreateAccountKeys, args: CreateAccountIxArgs) -> Instruction {
    create_account_ix_with_program_id(crate::ID, keys, args)
}
pub const CREATE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccountAccounts<'me, 'info> {
    ///Funding account
    pub from: &'me AccountInfo<'info>,
    ///The new account to be created
    pub to: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateAccountKeys {
    ///Funding account
    pub from: Pubkey,
    ///The new account to be created
    pub to: Pubkey,
}
impl From<CreateAccountAccounts<'_, '_>> for CreateAccountKeys {
    fn from(accounts: CreateAccountAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            to: *accounts.to.key,
        }
    }
}
impl From<CreateAccountKeys> for [AccountMeta; CREATE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.from,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: true,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            to: pubkeys[1],
        }
    }
}
impl<'info> From<CreateAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateAccountAccounts<'_, 'info>) -> Self {
        [accounts.from.clone(), accounts.to.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for CreateAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            to: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateAccountIxArgs {
    pub lamports: u64,
    pub space: u64,
    pub owner: Pubkey,
}
pub fn create_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccountAccounts<'_, '_>,
    args: CreateAccountIxArgs,
) -> ProgramResult {
    let keys: CreateAccountKeys = accounts.into();
    let ix = create_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn create_account_invoke(
    accounts: CreateAccountAccounts<'_, '_>,
    args: CreateAccountIxArgs,
) -> ProgramResult {
    create_account_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccountAccounts<'_, '_>,
    args: CreateAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAccountKeys = accounts.into();
    let ix = create_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_account_invoke_signed(
    accounts: CreateAccountAccounts<'_, '_>,
    args: CreateAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_account_verify_account_keys(
    accounts: CreateAccountAccounts<'_, '_>,
    keys: CreateAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.from.key, &keys.from), (accounts.to.key, &keys.to)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_account_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_account_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.from, accounts.to] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_account_verify_account_privileges<'me, 'info>(
    accounts: CreateAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_account_verify_writable_privileges(accounts)?;
    create_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ASSIGN_IX_DISCM: [u8; 4] = [1, 0, 0, 0];
pub fn assign_ix_with_program_id(
    program_id: Pubkey,
    keys: AssignKeys,
    args: AssignIxArgs,
) -> Instruction {
    let metas: [AccountMeta; ASSIGN_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::Assign(args),
        Vec::from(metas),
    )
}
pub fn assign_ix(keys: AssignKeys, args: AssignIxArgs) -> Instruction {
    assign_ix_with_program_id(crate::ID, keys, args)
}
pub const ASSIGN_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct AssignAccounts<'me, 'info> {
    ///The system account to assign a new program owner to
    pub assign: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AssignKeys {
    ///The system account to assign a new program owner to
    pub assign: Pubkey,
}
impl From<AssignAccounts<'_, '_>> for AssignKeys {
    fn from(accounts: AssignAccounts) -> Self {
        Self {
            assign: *accounts.assign.key,
        }
    }
}
impl From<AssignKeys> for [AccountMeta; ASSIGN_IX_ACCOUNTS_LEN] {
    fn from(keys: AssignKeys) -> Self {
        [AccountMeta {
            pubkey: keys.assign,
            is_signer: true,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; ASSIGN_IX_ACCOUNTS_LEN]> for AssignKeys {
    fn from(pubkeys: [Pubkey; ASSIGN_IX_ACCOUNTS_LEN]) -> Self {
        Self { assign: pubkeys[0] }
    }
}
impl<'info> From<AssignAccounts<'_, 'info>> for [AccountInfo<'info>; ASSIGN_IX_ACCOUNTS_LEN] {
    fn from(accounts: AssignAccounts<'_, 'info>) -> Self {
        [accounts.assign.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ASSIGN_IX_ACCOUNTS_LEN]>
    for AssignAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ASSIGN_IX_ACCOUNTS_LEN]) -> Self {
        Self { assign: &arr[0] }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AssignIxArgs {
    pub owner: Pubkey,
}
pub fn assign_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AssignAccounts<'_, '_>,
    args: AssignIxArgs,
) -> ProgramResult {
    let keys: AssignKeys = accounts.into();
    let ix = assign_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn assign_invoke(accounts: AssignAccounts<'_, '_>, args: AssignIxArgs) -> ProgramResult {
    assign_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn assign_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AssignAccounts<'_, '_>,
    args: AssignIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AssignKeys = accounts.into();
    let ix = assign_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn assign_invoke_signed(
    accounts: AssignAccounts<'_, '_>,
    args: AssignIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    assign_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn assign_verify_account_keys(
    accounts: AssignAccounts<'_, '_>,
    keys: AssignKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.assign.key, &keys.assign)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn assign_verify_writable_privileges<'me, 'info>(
    accounts: AssignAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.assign] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn assign_verify_signer_privileges<'me, 'info>(
    accounts: AssignAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.assign] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn assign_verify_account_privileges<'me, 'info>(
    accounts: AssignAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    assign_verify_writable_privileges(accounts)?;
    assign_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_IX_DISCM: [u8; 4] = [2, 0, 0, 0];
pub fn transfer_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferKeys,
    args: TransferIxArgs,
) -> Instruction {
    let metas: [AccountMeta; TRANSFER_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::Transfer(args),
        Vec::from(metas),
    )
}
pub fn transfer_ix(keys: TransferKeys, args: TransferIxArgs) -> Instruction {
    transfer_ix_with_program_id(crate::ID, keys, args)
}
pub const TRANSFER_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct TransferAccounts<'me, 'info> {
    ///Funding account
    pub from: &'me AccountInfo<'info>,
    ///Recipient account
    pub to: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct TransferKeys {
    ///Funding account
    pub from: Pubkey,
    ///Recipient account
    pub to: Pubkey,
}
impl From<TransferAccounts<'_, '_>> for TransferKeys {
    fn from(accounts: TransferAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            to: *accounts.to.key,
        }
    }
}
impl From<TransferKeys> for [AccountMeta; TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.from,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_IX_ACCOUNTS_LEN]> for TransferKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            to: pubkeys[1],
        }
    }
}
impl<'info> From<TransferAccounts<'_, 'info>> for [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN] {
    fn from(accounts: TransferAccounts<'_, 'info>) -> Self {
        [accounts.from.clone(), accounts.to.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN]>
    for TransferAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            to: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransferIxArgs {
    pub lamports: u64,
}
pub fn transfer_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferAccounts<'_, '_>,
    args: TransferIxArgs,
) -> ProgramResult {
    let keys: TransferKeys = accounts.into();
    let ix = transfer_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn transfer_invoke(accounts: TransferAccounts<'_, '_>, args: TransferIxArgs) -> ProgramResult {
    transfer_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn transfer_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferAccounts<'_, '_>,
    args: TransferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferKeys = accounts.into();
    let ix = transfer_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_invoke_signed(
    accounts: TransferAccounts<'_, '_>,
    args: TransferIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn transfer_verify_account_keys(
    accounts: TransferAccounts<'_, '_>,
    keys: TransferKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.from.key, &keys.from), (accounts.to.key, &keys.to)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn transfer_verify_writable_privileges<'me, 'info>(
    accounts: TransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_verify_signer_privileges<'me, 'info>(
    accounts: TransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.from] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_verify_account_privileges<'me, 'info>(
    accounts: TransferAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_verify_writable_privileges(accounts)?;
    transfer_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CREATE_ACCOUNT_WITH_SEED_IX_DISCM: [u8; 4] = [3, 0, 0, 0];
pub fn create_account_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: CreateAccountWithSeedKeys,
    args: CreateAccountWithSeedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::CreateAccountWithSeed(args),
        Vec::from(metas),
    )
}
pub fn create_account_with_seed_ix(
    keys: CreateAccountWithSeedKeys,
    args: CreateAccountWithSeedIxArgs,
) -> Instruction {
    create_account_with_seed_ix_with_program_id(crate::ID, keys, args)
}
pub const CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct CreateAccountWithSeedAccounts<'me, 'info> {
    ///Funding account
    pub from: &'me AccountInfo<'info>,
    ///The new account to be created
    pub to: &'me AccountInfo<'info>,
    ///Base account. Optional. The account matching the base Pubkey below must be provided as a signer, but may be the same as from
    pub base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateAccountWithSeedKeys {
    ///Funding account
    pub from: Pubkey,
    ///The new account to be created
    pub to: Pubkey,
    ///Base account. Optional. The account matching the base Pubkey below must be provided as a signer, but may be the same as from
    pub base: Pubkey,
}
impl From<CreateAccountWithSeedAccounts<'_, '_>> for CreateAccountWithSeedKeys {
    fn from(accounts: CreateAccountWithSeedAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            to: *accounts.to.key,
            base: *accounts.base.key,
        }
    }
}
impl From<CreateAccountWithSeedKeys> for [AccountMeta; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: CreateAccountWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.from,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN]> for CreateAccountWithSeedKeys {
    fn from(pubkeys: [Pubkey; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            to: pubkeys[1],
            base: pubkeys[2],
        }
    }
}
impl<'info> From<CreateAccountWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: CreateAccountWithSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.from.clone(),
            accounts.to.clone(),
            accounts.base.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN]>
    for CreateAccountWithSeedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_ACCOUNT_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            to: &arr[1],
            base: &arr[2],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct CreateAccountWithSeedIxArgs {
    pub base: Pubkey,
    pub seed: String,
    pub lamports: u64,
    pub space: u64,
    pub owner: Pubkey,
}
pub fn create_account_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccountWithSeedAccounts<'_, '_>,
    args: CreateAccountWithSeedIxArgs,
) -> ProgramResult {
    let keys: CreateAccountWithSeedKeys = accounts.into();
    let ix = create_account_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn create_account_with_seed_invoke(
    accounts: CreateAccountWithSeedAccounts<'_, '_>,
    args: CreateAccountWithSeedIxArgs,
) -> ProgramResult {
    create_account_with_seed_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn create_account_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CreateAccountWithSeedAccounts<'_, '_>,
    args: CreateAccountWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CreateAccountWithSeedKeys = accounts.into();
    let ix = create_account_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn create_account_with_seed_invoke_signed(
    accounts: CreateAccountWithSeedAccounts<'_, '_>,
    args: CreateAccountWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    create_account_with_seed_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn create_account_with_seed_verify_account_keys(
    accounts: CreateAccountWithSeedAccounts<'_, '_>,
    keys: CreateAccountWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.from.key, &keys.from),
        (accounts.to.key, &keys.to),
        (accounts.base.key, &keys.base),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_account_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: CreateAccountWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn create_account_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: CreateAccountWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.from, accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn create_account_with_seed_verify_account_privileges<'me, 'info>(
    accounts: CreateAccountWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    create_account_with_seed_verify_writable_privileges(accounts)?;
    create_account_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ADVANCE_NONCE_ACCOUNT_IX_DISCM: [u8; 4] = [4, 0, 0, 0];
pub fn advance_nonce_account_ix_with_program_id(
    program_id: Pubkey,
    keys: AdvanceNonceAccountKeys,
) -> Instruction {
    let metas: [AccountMeta; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::AdvanceNonceAccount,
        Vec::from(metas),
    )
}
pub fn advance_nonce_account_ix(keys: AdvanceNonceAccountKeys) -> Instruction {
    advance_nonce_account_ix_with_program_id(crate::ID, keys)
}
pub const ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct AdvanceNonceAccountAccounts<'me, 'info> {
    ///Nonce account
    pub nonce: &'me AccountInfo<'info>,
    ///RecentBlockhashes sysvar
    pub recent_blockhashes: &'me AccountInfo<'info>,
    ///nonce's authority
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AdvanceNonceAccountKeys {
    ///Nonce account
    pub nonce: Pubkey,
    ///RecentBlockhashes sysvar
    pub recent_blockhashes: Pubkey,
    ///nonce's authority
    pub authority: Pubkey,
}
impl From<AdvanceNonceAccountAccounts<'_, '_>> for AdvanceNonceAccountKeys {
    fn from(accounts: AdvanceNonceAccountAccounts) -> Self {
        Self {
            nonce: *accounts.nonce.key,
            recent_blockhashes: *accounts.recent_blockhashes.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<AdvanceNonceAccountKeys> for [AccountMeta; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: AdvanceNonceAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nonce,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recent_blockhashes,
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
impl From<[Pubkey; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]> for AdvanceNonceAccountKeys {
    fn from(pubkeys: [Pubkey; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: pubkeys[0],
            recent_blockhashes: pubkeys[1],
            authority: pubkeys[2],
        }
    }
}
impl<'info> From<AdvanceNonceAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AdvanceNonceAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.nonce.clone(),
            accounts.recent_blockhashes.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for AdvanceNonceAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ADVANCE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: &arr[0],
            recent_blockhashes: &arr[1],
            authority: &arr[2],
        }
    }
}
pub fn advance_nonce_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AdvanceNonceAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: AdvanceNonceAccountKeys = accounts.into();
    let ix = advance_nonce_account_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn advance_nonce_account_invoke(
    accounts: AdvanceNonceAccountAccounts<'_, '_>,
) -> ProgramResult {
    advance_nonce_account_invoke_with_program_id(crate::ID, accounts)
}
pub fn advance_nonce_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AdvanceNonceAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AdvanceNonceAccountKeys = accounts.into();
    let ix = advance_nonce_account_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn advance_nonce_account_invoke_signed(
    accounts: AdvanceNonceAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    advance_nonce_account_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn advance_nonce_account_verify_account_keys(
    accounts: AdvanceNonceAccountAccounts<'_, '_>,
    keys: AdvanceNonceAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.nonce.key, &keys.nonce),
        (accounts.recent_blockhashes.key, &keys.recent_blockhashes),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn advance_nonce_account_verify_writable_privileges<'me, 'info>(
    accounts: AdvanceNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.nonce] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn advance_nonce_account_verify_signer_privileges<'me, 'info>(
    accounts: AdvanceNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn advance_nonce_account_verify_account_privileges<'me, 'info>(
    accounts: AdvanceNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    advance_nonce_account_verify_writable_privileges(accounts)?;
    advance_nonce_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_NONCE_ACCOUNT_IX_DISCM: [u8; 4] = [5, 0, 0, 0];
pub fn withdraw_nonce_account_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawNonceAccountKeys,
    args: WithdrawNonceAccountIxArgs,
) -> Instruction {
    let metas: [AccountMeta; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::WithdrawNonceAccount(args),
        Vec::from(metas),
    )
}
pub fn withdraw_nonce_account_ix(
    keys: WithdrawNonceAccountKeys,
    args: WithdrawNonceAccountIxArgs,
) -> Instruction {
    withdraw_nonce_account_ix_with_program_id(crate::ID, keys, args)
}
pub const WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawNonceAccountAccounts<'me, 'info> {
    ///Nonce account
    pub nonce: &'me AccountInfo<'info>,
    ///Recipient account
    pub to: &'me AccountInfo<'info>,
    ///RecentBlockhashes sysvar
    pub recent_blockhashes: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
    ///nonce's authority
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct WithdrawNonceAccountKeys {
    ///Nonce account
    pub nonce: Pubkey,
    ///Recipient account
    pub to: Pubkey,
    ///RecentBlockhashes sysvar
    pub recent_blockhashes: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
    ///nonce's authority
    pub authority: Pubkey,
}
impl From<WithdrawNonceAccountAccounts<'_, '_>> for WithdrawNonceAccountKeys {
    fn from(accounts: WithdrawNonceAccountAccounts) -> Self {
        Self {
            nonce: *accounts.nonce.key,
            to: *accounts.to.key,
            recent_blockhashes: *accounts.recent_blockhashes.key,
            rent: *accounts.rent.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<WithdrawNonceAccountKeys> for [AccountMeta; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawNonceAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nonce,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recent_blockhashes,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
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
impl From<[Pubkey; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]> for WithdrawNonceAccountKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: pubkeys[0],
            to: pubkeys[1],
            recent_blockhashes: pubkeys[2],
            rent: pubkeys[3],
            authority: pubkeys[4],
        }
    }
}
impl<'info> From<WithdrawNonceAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: WithdrawNonceAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.nonce.clone(),
            accounts.to.clone(),
            accounts.recent_blockhashes.clone(),
            accounts.rent.clone(),
            accounts.authority.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for WithdrawNonceAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: &arr[0],
            to: &arr[1],
            recent_blockhashes: &arr[2],
            rent: &arr[3],
            authority: &arr[4],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct WithdrawNonceAccountIxArgs {
    pub lamports: u64,
}
pub fn withdraw_nonce_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawNonceAccountAccounts<'_, '_>,
    args: WithdrawNonceAccountIxArgs,
) -> ProgramResult {
    let keys: WithdrawNonceAccountKeys = accounts.into();
    let ix = withdraw_nonce_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_nonce_account_invoke(
    accounts: WithdrawNonceAccountAccounts<'_, '_>,
    args: WithdrawNonceAccountIxArgs,
) -> ProgramResult {
    withdraw_nonce_account_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn withdraw_nonce_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawNonceAccountAccounts<'_, '_>,
    args: WithdrawNonceAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawNonceAccountKeys = accounts.into();
    let ix = withdraw_nonce_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_nonce_account_invoke_signed(
    accounts: WithdrawNonceAccountAccounts<'_, '_>,
    args: WithdrawNonceAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_nonce_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn withdraw_nonce_account_verify_account_keys(
    accounts: WithdrawNonceAccountAccounts<'_, '_>,
    keys: WithdrawNonceAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.nonce.key, &keys.nonce),
        (accounts.to.key, &keys.to),
        (accounts.recent_blockhashes.key, &keys.recent_blockhashes),
        (accounts.rent.key, &keys.rent),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn withdraw_nonce_account_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.nonce, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_nonce_account_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_nonce_account_verify_account_privileges<'me, 'info>(
    accounts: WithdrawNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_nonce_account_verify_writable_privileges(accounts)?;
    withdraw_nonce_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_NONCE_ACCOUNT_IX_DISCM: [u8; 4] = [6, 0, 0, 0];
pub fn initialize_nonce_account_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeNonceAccountKeys,
    args: InitializeNonceAccountIxArgs,
) -> Instruction {
    let metas: [AccountMeta; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::InitializeNonceAccount(args),
        Vec::from(metas),
    )
}
pub fn initialize_nonce_account_ix(
    keys: InitializeNonceAccountKeys,
    args: InitializeNonceAccountIxArgs,
) -> Instruction {
    initialize_nonce_account_ix_with_program_id(crate::ID, keys, args)
}
pub const INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct InitializeNonceAccountAccounts<'me, 'info> {
    ///Nonce account
    pub nonce: &'me AccountInfo<'info>,
    ///RecentBlockhashes sysvar
    pub recent_blockhashes: &'me AccountInfo<'info>,
    ///Rent sysvar
    pub rent: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct InitializeNonceAccountKeys {
    ///Nonce account
    pub nonce: Pubkey,
    ///RecentBlockhashes sysvar
    pub recent_blockhashes: Pubkey,
    ///Rent sysvar
    pub rent: Pubkey,
}
impl From<InitializeNonceAccountAccounts<'_, '_>> for InitializeNonceAccountKeys {
    fn from(accounts: InitializeNonceAccountAccounts) -> Self {
        Self {
            nonce: *accounts.nonce.key,
            recent_blockhashes: *accounts.recent_blockhashes.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<InitializeNonceAccountKeys> for [AccountMeta; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeNonceAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nonce,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.recent_blockhashes,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.rent,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]> for InitializeNonceAccountKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: pubkeys[0],
            recent_blockhashes: pubkeys[1],
            rent: pubkeys[2],
        }
    }
}
impl<'info> From<InitializeNonceAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: InitializeNonceAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.nonce.clone(),
            accounts.recent_blockhashes.clone(),
            accounts.rent.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for InitializeNonceAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: &arr[0],
            recent_blockhashes: &arr[1],
            rent: &arr[2],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct InitializeNonceAccountIxArgs {
    pub authority: Pubkey,
}
pub fn initialize_nonce_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNonceAccountAccounts<'_, '_>,
    args: InitializeNonceAccountIxArgs,
) -> ProgramResult {
    let keys: InitializeNonceAccountKeys = accounts.into();
    let ix = initialize_nonce_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn initialize_nonce_account_invoke(
    accounts: InitializeNonceAccountAccounts<'_, '_>,
    args: InitializeNonceAccountIxArgs,
) -> ProgramResult {
    initialize_nonce_account_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_nonce_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeNonceAccountAccounts<'_, '_>,
    args: InitializeNonceAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeNonceAccountKeys = accounts.into();
    let ix = initialize_nonce_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_nonce_account_invoke_signed(
    accounts: InitializeNonceAccountAccounts<'_, '_>,
    args: InitializeNonceAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_nonce_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_nonce_account_verify_account_keys(
    accounts: InitializeNonceAccountAccounts<'_, '_>,
    keys: InitializeNonceAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.nonce.key, &keys.nonce),
        (accounts.recent_blockhashes.key, &keys.recent_blockhashes),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn initialize_nonce_account_verify_writable_privileges<'me, 'info>(
    accounts: InitializeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.nonce] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_nonce_account_verify_account_privileges<'me, 'info>(
    accounts: InitializeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_nonce_account_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const AUTHORIZE_NONCE_ACCOUNT_IX_DISCM: [u8; 4] = [7, 0, 0, 0];
pub fn authorize_nonce_account_ix_with_program_id(
    program_id: Pubkey,
    keys: AuthorizeNonceAccountKeys,
    args: AuthorizeNonceAccountIxArgs,
) -> Instruction {
    let metas: [AccountMeta; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::AuthorizeNonceAccount(args),
        Vec::from(metas),
    )
}
pub fn authorize_nonce_account_ix(
    keys: AuthorizeNonceAccountKeys,
    args: AuthorizeNonceAccountIxArgs,
) -> Instruction {
    authorize_nonce_account_ix_with_program_id(crate::ID, keys, args)
}
pub const AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeNonceAccountAccounts<'me, 'info> {
    ///Nonce account
    pub nonce: &'me AccountInfo<'info>,
    ///nonce's authority
    pub authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AuthorizeNonceAccountKeys {
    ///Nonce account
    pub nonce: Pubkey,
    ///nonce's authority
    pub authority: Pubkey,
}
impl From<AuthorizeNonceAccountAccounts<'_, '_>> for AuthorizeNonceAccountKeys {
    fn from(accounts: AuthorizeNonceAccountAccounts) -> Self {
        Self {
            nonce: *accounts.nonce.key,
            authority: *accounts.authority.key,
        }
    }
}
impl From<AuthorizeNonceAccountKeys> for [AccountMeta; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: AuthorizeNonceAccountKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.nonce,
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
impl From<[Pubkey; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]> for AuthorizeNonceAccountKeys {
    fn from(pubkeys: [Pubkey; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: pubkeys[0],
            authority: pubkeys[1],
        }
    }
}
impl<'info> From<AuthorizeNonceAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AuthorizeNonceAccountAccounts<'_, 'info>) -> Self {
        [accounts.nonce.clone(), accounts.authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for AuthorizeNonceAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; AUTHORIZE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            nonce: &arr[0],
            authority: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AuthorizeNonceAccountIxArgs {
    pub new_authority: Pubkey,
}
pub fn authorize_nonce_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeNonceAccountAccounts<'_, '_>,
    args: AuthorizeNonceAccountIxArgs,
) -> ProgramResult {
    let keys: AuthorizeNonceAccountKeys = accounts.into();
    let ix = authorize_nonce_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn authorize_nonce_account_invoke(
    accounts: AuthorizeNonceAccountAccounts<'_, '_>,
    args: AuthorizeNonceAccountIxArgs,
) -> ProgramResult {
    authorize_nonce_account_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn authorize_nonce_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AuthorizeNonceAccountAccounts<'_, '_>,
    args: AuthorizeNonceAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AuthorizeNonceAccountKeys = accounts.into();
    let ix = authorize_nonce_account_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn authorize_nonce_account_invoke_signed(
    accounts: AuthorizeNonceAccountAccounts<'_, '_>,
    args: AuthorizeNonceAccountIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    authorize_nonce_account_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn authorize_nonce_account_verify_account_keys(
    accounts: AuthorizeNonceAccountAccounts<'_, '_>,
    keys: AuthorizeNonceAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.nonce.key, &keys.nonce),
        (accounts.authority.key, &keys.authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn authorize_nonce_account_verify_writable_privileges<'me, 'info>(
    accounts: AuthorizeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.nonce] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn authorize_nonce_account_verify_signer_privileges<'me, 'info>(
    accounts: AuthorizeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn authorize_nonce_account_verify_account_privileges<'me, 'info>(
    accounts: AuthorizeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    authorize_nonce_account_verify_writable_privileges(accounts)?;
    authorize_nonce_account_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ALLOCATE_IX_DISCM: [u8; 4] = [8, 0, 0, 0];
pub fn allocate_ix_with_program_id(
    program_id: Pubkey,
    keys: AllocateKeys,
    args: AllocateIxArgs,
) -> Instruction {
    let metas: [AccountMeta; ALLOCATE_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::Allocate(args),
        Vec::from(metas),
    )
}
pub fn allocate_ix(keys: AllocateKeys, args: AllocateIxArgs) -> Instruction {
    allocate_ix_with_program_id(crate::ID, keys, args)
}
pub const ALLOCATE_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct AllocateAccounts<'me, 'info> {
    ///The new account to allocate space for
    pub allocate: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AllocateKeys {
    ///The new account to allocate space for
    pub allocate: Pubkey,
}
impl From<AllocateAccounts<'_, '_>> for AllocateKeys {
    fn from(accounts: AllocateAccounts) -> Self {
        Self {
            allocate: *accounts.allocate.key,
        }
    }
}
impl From<AllocateKeys> for [AccountMeta; ALLOCATE_IX_ACCOUNTS_LEN] {
    fn from(keys: AllocateKeys) -> Self {
        [AccountMeta {
            pubkey: keys.allocate,
            is_signer: true,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; ALLOCATE_IX_ACCOUNTS_LEN]> for AllocateKeys {
    fn from(pubkeys: [Pubkey; ALLOCATE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            allocate: pubkeys[0],
        }
    }
}
impl<'info> From<AllocateAccounts<'_, 'info>> for [AccountInfo<'info>; ALLOCATE_IX_ACCOUNTS_LEN] {
    fn from(accounts: AllocateAccounts<'_, 'info>) -> Self {
        [accounts.allocate.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ALLOCATE_IX_ACCOUNTS_LEN]>
    for AllocateAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ALLOCATE_IX_ACCOUNTS_LEN]) -> Self {
        Self { allocate: &arr[0] }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AllocateIxArgs {
    pub space: u64,
}
pub fn allocate_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AllocateAccounts<'_, '_>,
    args: AllocateIxArgs,
) -> ProgramResult {
    let keys: AllocateKeys = accounts.into();
    let ix = allocate_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn allocate_invoke(accounts: AllocateAccounts<'_, '_>, args: AllocateIxArgs) -> ProgramResult {
    allocate_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn allocate_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AllocateAccounts<'_, '_>,
    args: AllocateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AllocateKeys = accounts.into();
    let ix = allocate_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn allocate_invoke_signed(
    accounts: AllocateAccounts<'_, '_>,
    args: AllocateIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    allocate_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn allocate_verify_account_keys(
    accounts: AllocateAccounts<'_, '_>,
    keys: AllocateKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.allocate.key, &keys.allocate)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn allocate_verify_writable_privileges<'me, 'info>(
    accounts: AllocateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.allocate] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn allocate_verify_signer_privileges<'me, 'info>(
    accounts: AllocateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.allocate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn allocate_verify_account_privileges<'me, 'info>(
    accounts: AllocateAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    allocate_verify_writable_privileges(accounts)?;
    allocate_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ALLOCATE_WITH_SEED_IX_DISCM: [u8; 4] = [9, 0, 0, 0];
pub fn allocate_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: AllocateWithSeedKeys,
    args: AllocateWithSeedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::AllocateWithSeed(args),
        Vec::from(metas),
    )
}
pub fn allocate_with_seed_ix(
    keys: AllocateWithSeedKeys,
    args: AllocateWithSeedIxArgs,
) -> Instruction {
    allocate_with_seed_ix_with_program_id(crate::ID, keys, args)
}
pub const ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AllocateWithSeedAccounts<'me, 'info> {
    ///The new account to allocate space for
    pub allocate: &'me AccountInfo<'info>,
    ///Base account
    pub base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AllocateWithSeedKeys {
    ///The new account to allocate space for
    pub allocate: Pubkey,
    ///Base account
    pub base: Pubkey,
}
impl From<AllocateWithSeedAccounts<'_, '_>> for AllocateWithSeedKeys {
    fn from(accounts: AllocateWithSeedAccounts) -> Self {
        Self {
            allocate: *accounts.allocate.key,
            base: *accounts.base.key,
        }
    }
}
impl From<AllocateWithSeedKeys> for [AccountMeta; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: AllocateWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.allocate,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN]> for AllocateWithSeedKeys {
    fn from(pubkeys: [Pubkey; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            allocate: pubkeys[0],
            base: pubkeys[1],
        }
    }
}
impl<'info> From<AllocateWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AllocateWithSeedAccounts<'_, 'info>) -> Self {
        [accounts.allocate.clone(), accounts.base.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN]>
    for AllocateWithSeedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ALLOCATE_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            allocate: &arr[0],
            base: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AllocateWithSeedIxArgs {
    pub base: Pubkey,
    pub seed: String,
    pub space: u64,
    pub owner: Pubkey,
}
pub fn allocate_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AllocateWithSeedAccounts<'_, '_>,
    args: AllocateWithSeedIxArgs,
) -> ProgramResult {
    let keys: AllocateWithSeedKeys = accounts.into();
    let ix = allocate_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn allocate_with_seed_invoke(
    accounts: AllocateWithSeedAccounts<'_, '_>,
    args: AllocateWithSeedIxArgs,
) -> ProgramResult {
    allocate_with_seed_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn allocate_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AllocateWithSeedAccounts<'_, '_>,
    args: AllocateWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AllocateWithSeedKeys = accounts.into();
    let ix = allocate_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn allocate_with_seed_invoke_signed(
    accounts: AllocateWithSeedAccounts<'_, '_>,
    args: AllocateWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    allocate_with_seed_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn allocate_with_seed_verify_account_keys(
    accounts: AllocateWithSeedAccounts<'_, '_>,
    keys: AllocateWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.allocate.key, &keys.allocate),
        (accounts.base.key, &keys.base),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn allocate_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: AllocateWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.allocate] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn allocate_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: AllocateWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn allocate_with_seed_verify_account_privileges<'me, 'info>(
    accounts: AllocateWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    allocate_with_seed_verify_writable_privileges(accounts)?;
    allocate_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const ASSIGN_WITH_SEED_IX_DISCM: [u8; 4] = [10, 0, 0, 0];
pub fn assign_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: AssignWithSeedKeys,
    args: AssignWithSeedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::AssignWithSeed(args),
        Vec::from(metas),
    )
}
pub fn assign_with_seed_ix(keys: AssignWithSeedKeys, args: AssignWithSeedIxArgs) -> Instruction {
    assign_with_seed_ix_with_program_id(crate::ID, keys, args)
}
pub const ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct AssignWithSeedAccounts<'me, 'info> {
    ///The system account to assign a new program owner to
    pub assign: &'me AccountInfo<'info>,
    ///Base account
    pub base: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct AssignWithSeedKeys {
    ///The system account to assign a new program owner to
    pub assign: Pubkey,
    ///Base account
    pub base: Pubkey,
}
impl From<AssignWithSeedAccounts<'_, '_>> for AssignWithSeedKeys {
    fn from(accounts: AssignWithSeedAccounts) -> Self {
        Self {
            assign: *accounts.assign.key,
            base: *accounts.base.key,
        }
    }
}
impl From<AssignWithSeedKeys> for [AccountMeta; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: AssignWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.assign,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN]> for AssignWithSeedKeys {
    fn from(pubkeys: [Pubkey; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            assign: pubkeys[0],
            base: pubkeys[1],
        }
    }
}
impl<'info> From<AssignWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: AssignWithSeedAccounts<'_, 'info>) -> Self {
        [accounts.assign.clone(), accounts.base.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN]>
    for AssignWithSeedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; ASSIGN_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            assign: &arr[0],
            base: &arr[1],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct AssignWithSeedIxArgs {
    pub base: Pubkey,
    pub seed: String,
    pub owner: Pubkey,
}
pub fn assign_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: AssignWithSeedAccounts<'_, '_>,
    args: AssignWithSeedIxArgs,
) -> ProgramResult {
    let keys: AssignWithSeedKeys = accounts.into();
    let ix = assign_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn assign_with_seed_invoke(
    accounts: AssignWithSeedAccounts<'_, '_>,
    args: AssignWithSeedIxArgs,
) -> ProgramResult {
    assign_with_seed_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn assign_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: AssignWithSeedAccounts<'_, '_>,
    args: AssignWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: AssignWithSeedKeys = accounts.into();
    let ix = assign_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn assign_with_seed_invoke_signed(
    accounts: AssignWithSeedAccounts<'_, '_>,
    args: AssignWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    assign_with_seed_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn assign_with_seed_verify_account_keys(
    accounts: AssignWithSeedAccounts<'_, '_>,
    keys: AssignWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.assign.key, &keys.assign),
        (accounts.base.key, &keys.base),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn assign_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: AssignWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.assign] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn assign_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: AssignWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn assign_with_seed_verify_account_privileges<'me, 'info>(
    accounts: AssignWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    assign_with_seed_verify_writable_privileges(accounts)?;
    assign_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const TRANSFER_WITH_SEED_IX_DISCM: [u8; 4] = [11, 0, 0, 0];
pub fn transfer_with_seed_ix_with_program_id(
    program_id: Pubkey,
    keys: TransferWithSeedKeys,
    args: TransferWithSeedIxArgs,
) -> Instruction {
    let metas: [AccountMeta; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::TransferWithSeed(args),
        Vec::from(metas),
    )
}
pub fn transfer_with_seed_ix(
    keys: TransferWithSeedKeys,
    args: TransferWithSeedIxArgs,
) -> Instruction {
    transfer_with_seed_ix_with_program_id(crate::ID, keys, args)
}
pub const TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct TransferWithSeedAccounts<'me, 'info> {
    ///Funding account
    pub from: &'me AccountInfo<'info>,
    ///from's base account
    pub base: &'me AccountInfo<'info>,
    ///Recipient account
    pub to: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct TransferWithSeedKeys {
    ///Funding account
    pub from: Pubkey,
    ///from's base account
    pub base: Pubkey,
    ///Recipient account
    pub to: Pubkey,
}
impl From<TransferWithSeedAccounts<'_, '_>> for TransferWithSeedKeys {
    fn from(accounts: TransferWithSeedAccounts) -> Self {
        Self {
            from: *accounts.from.key,
            base: *accounts.base.key,
            to: *accounts.to.key,
        }
    }
}
impl From<TransferWithSeedKeys> for [AccountMeta; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN] {
    fn from(keys: TransferWithSeedKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.from,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.base,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.to,
                is_signer: false,
                is_writable: true,
            },
        ]
    }
}
impl From<[Pubkey; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN]> for TransferWithSeedKeys {
    fn from(pubkeys: [Pubkey; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: pubkeys[0],
            base: pubkeys[1],
            to: pubkeys[2],
        }
    }
}
impl<'info> From<TransferWithSeedAccounts<'_, 'info>>
    for [AccountInfo<'info>; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN]
{
    fn from(accounts: TransferWithSeedAccounts<'_, 'info>) -> Self {
        [
            accounts.from.clone(),
            accounts.base.clone(),
            accounts.to.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN]>
    for TransferWithSeedAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; TRANSFER_WITH_SEED_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            from: &arr[0],
            base: &arr[1],
            to: &arr[2],
        }
    }
}
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct TransferWithSeedIxArgs {
    pub lamports: u64,
    pub from_seed: String,
    pub from_owner: Pubkey,
}
pub fn transfer_with_seed_invoke_with_program_id(
    program_id: Pubkey,
    accounts: TransferWithSeedAccounts<'_, '_>,
    args: TransferWithSeedIxArgs,
) -> ProgramResult {
    let keys: TransferWithSeedKeys = accounts.into();
    let ix = transfer_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction(&ix, accounts)
}
pub fn transfer_with_seed_invoke(
    accounts: TransferWithSeedAccounts<'_, '_>,
    args: TransferWithSeedIxArgs,
) -> ProgramResult {
    transfer_with_seed_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn transfer_with_seed_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: TransferWithSeedAccounts<'_, '_>,
    args: TransferWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: TransferWithSeedKeys = accounts.into();
    let ix = transfer_with_seed_ix_with_program_id(program_id, keys, args);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn transfer_with_seed_invoke_signed(
    accounts: TransferWithSeedAccounts<'_, '_>,
    args: TransferWithSeedIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    transfer_with_seed_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn transfer_with_seed_verify_account_keys(
    accounts: TransferWithSeedAccounts<'_, '_>,
    keys: TransferWithSeedKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.from.key, &keys.from),
        (accounts.base.key, &keys.base),
        (accounts.to.key, &keys.to),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn transfer_with_seed_verify_writable_privileges<'me, 'info>(
    accounts: TransferWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.from, accounts.to] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn transfer_with_seed_verify_signer_privileges<'me, 'info>(
    accounts: TransferWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.base] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn transfer_with_seed_verify_account_privileges<'me, 'info>(
    accounts: TransferWithSeedAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    transfer_with_seed_verify_writable_privileges(accounts)?;
    transfer_with_seed_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPGRADE_NONCE_ACCOUNT_IX_DISCM: [u8; 4] = [12, 0, 0, 0];
pub fn upgrade_nonce_account_ix_with_program_id(
    program_id: Pubkey,
    keys: UpgradeNonceAccountKeys,
) -> Instruction {
    let metas: [AccountMeta; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] = keys.into();
    Instruction::new_with_bincode(
        program_id,
        &SystemProgramProgramIx::UpgradeNonceAccount,
        Vec::from(metas),
    )
}
pub fn upgrade_nonce_account_ix(keys: UpgradeNonceAccountKeys) -> Instruction {
    upgrade_nonce_account_ix_with_program_id(crate::ID, keys)
}
pub const UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct UpgradeNonceAccountAccounts<'me, 'info> {
    ///Nonce account
    pub nonce: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpgradeNonceAccountKeys {
    ///Nonce account
    pub nonce: Pubkey,
}
impl From<UpgradeNonceAccountAccounts<'_, '_>> for UpgradeNonceAccountKeys {
    fn from(accounts: UpgradeNonceAccountAccounts) -> Self {
        Self {
            nonce: *accounts.nonce.key,
        }
    }
}
impl From<UpgradeNonceAccountKeys> for [AccountMeta; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: UpgradeNonceAccountKeys) -> Self {
        [AccountMeta {
            pubkey: keys.nonce,
            is_signer: false,
            is_writable: true,
        }]
    }
}
impl From<[Pubkey; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]> for UpgradeNonceAccountKeys {
    fn from(pubkeys: [Pubkey; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self { nonce: pubkeys[0] }
    }
}
impl<'info> From<UpgradeNonceAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: UpgradeNonceAccountAccounts<'_, 'info>) -> Self {
        [accounts.nonce.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]>
    for UpgradeNonceAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPGRADE_NONCE_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self { nonce: &arr[0] }
    }
}
pub fn upgrade_nonce_account_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpgradeNonceAccountAccounts<'_, '_>,
) -> ProgramResult {
    let keys: UpgradeNonceAccountKeys = accounts.into();
    let ix = upgrade_nonce_account_ix_with_program_id(program_id, keys);
    invoke_instruction(&ix, accounts)
}
pub fn upgrade_nonce_account_invoke(
    accounts: UpgradeNonceAccountAccounts<'_, '_>,
) -> ProgramResult {
    upgrade_nonce_account_invoke_with_program_id(crate::ID, accounts)
}
pub fn upgrade_nonce_account_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpgradeNonceAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpgradeNonceAccountKeys = accounts.into();
    let ix = upgrade_nonce_account_ix_with_program_id(program_id, keys);
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn upgrade_nonce_account_invoke_signed(
    accounts: UpgradeNonceAccountAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    upgrade_nonce_account_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn upgrade_nonce_account_verify_account_keys(
    accounts: UpgradeNonceAccountAccounts<'_, '_>,
    keys: UpgradeNonceAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.nonce.key, &keys.nonce)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn upgrade_nonce_account_verify_writable_privileges<'me, 'info>(
    accounts: UpgradeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.nonce] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn upgrade_nonce_account_verify_account_privileges<'me, 'info>(
    accounts: UpgradeNonceAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    upgrade_nonce_account_verify_writable_privileges(accounts)?;
    Ok(())
}
