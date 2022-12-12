use crate::*;
use borsh::BorshSerialize;
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub metadata: &'me AccountInfo<'a0>,
    pub mint: &'me AccountInfo<'a1>,
    pub mint_authority: &'me AccountInfo<'a2>,
    pub payer: &'me AccountInfo<'a3>,
    pub update_authority: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
    pub rent: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountKeys<'me> {
    pub metadata: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub mint_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&CreateMetadataAccountAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMetadataAccountKeys<'me>
{
    fn from(accounts: &CreateMetadataAccountAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            mint: accounts.mint.key,
            mint_authority: accounts.mint_authority.key,
            payer: accounts.payer.key,
            update_authority: accounts.update_authority.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&CreateMetadataAccountKeys<'_>> for [AccountMeta; 7] {
    fn from(keys: &CreateMetadataAccountKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.mint_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.update_authority, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 7]
{
    fn from(accounts: &CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.payer.clone(),
            accounts.update_authority.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct CreateMetadataAccountIxArgs {
    pub create_metadata_account_args: CreateMetadataAccountArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountIxData<'me>(pub &'me CreateMetadataAccountIxArgs);
impl<'me> From<&'me CreateMetadataAccountIxArgs> for CreateMetadataAccountIxData<'me> {
    fn from(args: &'me CreateMetadataAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMetadataAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[0u8])?;
        self.0.serialize(writer)
    }
}
pub fn create_metadata_account_ix<
    'k,
    'd,
    K: Into<CreateMetadataAccountKeys<'k>>,
    D: Into<CreateMetadataAccountIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: CreateMetadataAccountKeys = accounts.into();
    let metas: [AccountMeta; 7] = (&keys).into();
    let data: CreateMetadataAccountIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_account_invoke<'a, 'd, D: Into<CreateMetadataAccountIxData<'d>>>(
    accounts: &CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = create_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 7] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_metadata_account_invoke_signed<'a, 'd, D: Into<CreateMetadataAccountIxData<'d>>>(
    accounts: &CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 7] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub metadata: &'me AccountInfo<'a0>,
    pub update_authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountKeys<'me> {
    pub metadata: &'me Pubkey,
    pub update_authority: &'me Pubkey,
}
impl<'me> From<&UpdateMetadataAccountAccounts<'me, '_, '_>> for UpdateMetadataAccountKeys<'me> {
    fn from(accounts: &UpdateMetadataAccountAccounts<'me, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            update_authority: accounts.update_authority.key,
        }
    }
}
impl From<&UpdateMetadataAccountKeys<'_>> for [AccountMeta; 2] {
    fn from(keys: &UpdateMetadataAccountKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
        ]
    }
}
impl<'a> From<&UpdateMetadataAccountAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; 2] {
    fn from(accounts: &UpdateMetadataAccountAccounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.update_authority.clone()]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct UpdateMetadataAccountIxArgs {
    pub update_metadata_account_args: UpdateMetadataAccountArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountIxData<'me>(pub &'me UpdateMetadataAccountIxArgs);
impl<'me> From<&'me UpdateMetadataAccountIxArgs> for UpdateMetadataAccountIxData<'me> {
    fn from(args: &'me UpdateMetadataAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateMetadataAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[1u8])?;
        self.0.serialize(writer)
    }
}
pub fn update_metadata_account_ix<
    'k,
    'd,
    K: Into<UpdateMetadataAccountKeys<'k>>,
    D: Into<UpdateMetadataAccountIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: UpdateMetadataAccountKeys = accounts.into();
    let metas: [AccountMeta; 2] = (&keys).into();
    let data: UpdateMetadataAccountIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_metadata_account_invoke<'a, 'd, D: Into<UpdateMetadataAccountIxData<'d>>>(
    accounts: &UpdateMetadataAccountAccounts<'_, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = update_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_metadata_account_invoke_signed<'a, 'd, D: Into<UpdateMetadataAccountIxData<'d>>>(
    accounts: &UpdateMetadataAccountAccounts<'_, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionAccounts<
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
    'a11: 'me,
    'a12: 'me,
> {
    pub edition: &'me AccountInfo<'a0>,
    pub mint: &'me AccountInfo<'a1>,
    pub printing_mint: &'me AccountInfo<'a2>,
    pub one_time_printing_authorization_mint: &'me AccountInfo<'a3>,
    pub update_authority: &'me AccountInfo<'a4>,
    pub printing_mint_authority: &'me AccountInfo<'a5>,
    pub mint_authority: &'me AccountInfo<'a6>,
    pub metadata: &'me AccountInfo<'a7>,
    pub payer: &'me AccountInfo<'a8>,
    pub token_program: &'me AccountInfo<'a9>,
    pub system_program: &'me AccountInfo<'a10>,
    pub rent: &'me AccountInfo<'a11>,
    pub one_time_printing_authorization_mint_authority: &'me AccountInfo<'a12>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionKeys<'me> {
    pub edition: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub printing_mint: &'me Pubkey,
    pub one_time_printing_authorization_mint: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub printing_mint_authority: &'me Pubkey,
    pub mint_authority: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
    pub one_time_printing_authorization_mint_authority: &'me Pubkey,
}
impl<'me>
    From<
        &DeprecatedCreateMasterEditionAccounts<
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
            '_,
            '_,
        >,
    > for DeprecatedCreateMasterEditionKeys<'me>
{
    fn from(
        accounts: &DeprecatedCreateMasterEditionAccounts<
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
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            edition: accounts.edition.key,
            mint: accounts.mint.key,
            printing_mint: accounts.printing_mint.key,
            one_time_printing_authorization_mint: accounts.one_time_printing_authorization_mint.key,
            update_authority: accounts.update_authority.key,
            printing_mint_authority: accounts.printing_mint_authority.key,
            mint_authority: accounts.mint_authority.key,
            metadata: accounts.metadata.key,
            payer: accounts.payer.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
            one_time_printing_authorization_mint_authority: accounts
                .one_time_printing_authorization_mint_authority
                .key,
        }
    }
}
impl From<&DeprecatedCreateMasterEditionKeys<'_>> for [AccountMeta; 13] {
    fn from(keys: &DeprecatedCreateMasterEditionKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.edition, false),
            AccountMeta::new(*keys.mint, false),
            AccountMeta::new(*keys.printing_mint, false),
            AccountMeta::new(*keys.one_time_printing_authorization_mint, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.printing_mint_authority, true),
            AccountMeta::new_readonly(*keys.mint_authority, true),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
            AccountMeta::new_readonly(*keys.one_time_printing_authorization_mint_authority, true),
        ]
    }
}
impl<'a>
    From<
        &DeprecatedCreateMasterEditionAccounts<
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
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 13]
{
    fn from(
        accounts: &DeprecatedCreateMasterEditionAccounts<
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
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.printing_mint.clone(),
            accounts.one_time_printing_authorization_mint.clone(),
            accounts.update_authority.clone(),
            accounts.printing_mint_authority.clone(),
            accounts.mint_authority.clone(),
            accounts.metadata.clone(),
            accounts.payer.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts
                .one_time_printing_authorization_mint_authority
                .clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionIxArgs {
    pub create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionIxData<'me>(pub &'me DeprecatedCreateMasterEditionIxArgs);
impl<'me> From<&'me DeprecatedCreateMasterEditionIxArgs>
    for DeprecatedCreateMasterEditionIxData<'me>
{
    fn from(args: &'me DeprecatedCreateMasterEditionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedCreateMasterEditionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[2u8])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_create_master_edition_ix<
    'k,
    'd,
    K: Into<DeprecatedCreateMasterEditionKeys<'k>>,
    D: Into<DeprecatedCreateMasterEditionIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedCreateMasterEditionKeys = accounts.into();
    let metas: [AccountMeta; 13] = (&keys).into();
    let data: DeprecatedCreateMasterEditionIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_create_master_edition_invoke<
    'a,
    'd,
    D: Into<DeprecatedCreateMasterEditionIxData<'d>>,
>(
    accounts: &DeprecatedCreateMasterEditionAccounts<
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
        'a,
        'a,
    >,
    args: D,
) -> ProgramResult {
    let ix = deprecated_create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 13] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_create_master_edition_invoke_signed<
    'a,
    'd,
    D: Into<DeprecatedCreateMasterEditionIxData<'d>>,
>(
    accounts: &DeprecatedCreateMasterEditionAccounts<
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
        'a,
        'a,
    >,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 13] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
    'a11: 'me,
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
> {
    pub metadata: &'me AccountInfo<'a0>,
    pub edition: &'me AccountInfo<'a1>,
    pub master_edition: &'me AccountInfo<'a2>,
    pub mint: &'me AccountInfo<'a3>,
    pub mint_authority: &'me AccountInfo<'a4>,
    pub printing_mint: &'me AccountInfo<'a5>,
    pub master_token_account: &'me AccountInfo<'a6>,
    pub edition_marker: &'me AccountInfo<'a7>,
    pub burn_authority: &'me AccountInfo<'a8>,
    pub payer: &'me AccountInfo<'a9>,
    pub master_update_authority: &'me AccountInfo<'a10>,
    pub master_metadata: &'me AccountInfo<'a11>,
    pub token_program: &'me AccountInfo<'a12>,
    pub system_program: &'me AccountInfo<'a13>,
    pub rent: &'me AccountInfo<'a14>,
    pub reservation_list: &'me AccountInfo<'a15>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys<'me> {
    pub metadata: &'me Pubkey,
    pub edition: &'me Pubkey,
    pub master_edition: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub mint_authority: &'me Pubkey,
    pub printing_mint: &'me Pubkey,
    pub master_token_account: &'me Pubkey,
    pub edition_marker: &'me Pubkey,
    pub burn_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub master_update_authority: &'me Pubkey,
    pub master_metadata: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
    pub reservation_list: &'me Pubkey,
}
impl<'me>
    From<
        &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    > for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys<'me>
{
    fn from(
        accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            metadata: accounts.metadata.key,
            edition: accounts.edition.key,
            master_edition: accounts.master_edition.key,
            mint: accounts.mint.key,
            mint_authority: accounts.mint_authority.key,
            printing_mint: accounts.printing_mint.key,
            master_token_account: accounts.master_token_account.key,
            edition_marker: accounts.edition_marker.key,
            burn_authority: accounts.burn_authority.key,
            payer: accounts.payer.key,
            master_update_authority: accounts.master_update_authority.key,
            master_metadata: accounts.master_metadata.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
            reservation_list: accounts.reservation_list.key,
        }
    }
}
impl From<&DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys<'_>>
    for [AccountMeta; 16]
{
    fn from(keys: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new(*keys.edition, false),
            AccountMeta::new(*keys.master_edition, false),
            AccountMeta::new(*keys.mint, false),
            AccountMeta::new_readonly(*keys.mint_authority, true),
            AccountMeta::new(*keys.printing_mint, false),
            AccountMeta::new(*keys.master_token_account, false),
            AccountMeta::new(*keys.edition_marker, false),
            AccountMeta::new_readonly(*keys.burn_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.master_update_authority, false),
            AccountMeta::new_readonly(*keys.master_metadata, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
            AccountMeta::new(*keys.reservation_list, false),
        ]
    }
}
impl<'a>
    From<
        &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 16]
{
    fn from(
        accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.metadata.clone(),
            accounts.edition.clone(),
            accounts.master_edition.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.printing_mint.clone(),
            accounts.master_token_account.clone(),
            accounts.edition_marker.clone(),
            accounts.burn_authority.clone(),
            accounts.payer.clone(),
            accounts.master_update_authority.clone(),
            accounts.master_metadata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.reservation_list.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'me>(
    pub &'me DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs,
);
impl<'me> From<&'me DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'me>
{
    fn from(args: &'me DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[3u8])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_ix<
    'k,
    'd,
    K: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys<'k>>,
    D: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys = accounts.into();
    let metas: [AccountMeta; 16] = (&keys).into();
    let data: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_invoke<
    'a,
    'd,
    D: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'d>>,
>(
    accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: D,
) -> ProgramResult {
    let ix = deprecated_mint_new_edition_from_master_edition_via_printing_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 16] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_invoke_signed<
    'a,
    'd,
    D: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'d>>,
>(
    accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<
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
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_new_edition_from_master_edition_via_printing_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 16] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub metadata: &'me AccountInfo<'a0>,
    pub owner: &'me AccountInfo<'a1>,
    pub token: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenKeys<'me> {
    pub metadata: &'me Pubkey,
    pub owner: &'me Pubkey,
    pub token: &'me Pubkey,
}
impl<'me> From<&UpdatePrimarySaleHappenedViaTokenAccounts<'me, '_, '_, '_>>
    for UpdatePrimarySaleHappenedViaTokenKeys<'me>
{
    fn from(accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            owner: accounts.owner.key,
            token: accounts.token.key,
        }
    }
}
impl From<&UpdatePrimarySaleHappenedViaTokenKeys<'_>> for [AccountMeta; 3] {
    fn from(keys: &UpdatePrimarySaleHappenedViaTokenKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.owner, true),
            AccountMeta::new_readonly(*keys.token, false),
        ]
    }
}
impl<'a> From<&UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>> for [AccountInfo<'a>; 3] {
    fn from(accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.owner.clone(),
            accounts.token.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenIxData<'me>(
    pub &'me UpdatePrimarySaleHappenedViaTokenIxArgs,
);
impl<'me> From<&'me UpdatePrimarySaleHappenedViaTokenIxArgs>
    for UpdatePrimarySaleHappenedViaTokenIxData<'me>
{
    fn from(args: &'me UpdatePrimarySaleHappenedViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePrimarySaleHappenedViaTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[4u8])?;
        self.0.serialize(writer)
    }
}
pub fn update_primary_sale_happened_via_token_ix<
    'k,
    'd,
    K: Into<UpdatePrimarySaleHappenedViaTokenKeys<'k>>,
    D: Into<UpdatePrimarySaleHappenedViaTokenIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: UpdatePrimarySaleHappenedViaTokenKeys = accounts.into();
    let metas: [AccountMeta; 3] = (&keys).into();
    let data: UpdatePrimarySaleHappenedViaTokenIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_primary_sale_happened_via_token_invoke<
    'a,
    'd,
    D: Into<UpdatePrimarySaleHappenedViaTokenIxData<'d>>,
>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = update_primary_sale_happened_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 3] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_primary_sale_happened_via_token_invoke_signed<
    'a,
    'd,
    D: Into<UpdatePrimarySaleHappenedViaTokenIxData<'d>>,
>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_primary_sale_happened_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 3] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub master_edition: &'me AccountInfo<'a0>,
    pub reservation_list: &'me AccountInfo<'a1>,
    pub resource: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListKeys<'me> {
    pub master_edition: &'me Pubkey,
    pub reservation_list: &'me Pubkey,
    pub resource: &'me Pubkey,
}
impl<'me> From<&DeprecatedSetReservationListAccounts<'me, '_, '_, '_>>
    for DeprecatedSetReservationListKeys<'me>
{
    fn from(accounts: &DeprecatedSetReservationListAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            master_edition: accounts.master_edition.key,
            reservation_list: accounts.reservation_list.key,
            resource: accounts.resource.key,
        }
    }
}
impl From<&DeprecatedSetReservationListKeys<'_>> for [AccountMeta; 3] {
    fn from(keys: &DeprecatedSetReservationListKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.master_edition, false),
            AccountMeta::new(*keys.reservation_list, false),
            AccountMeta::new_readonly(*keys.resource, true),
        ]
    }
}
impl<'a> From<&DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>> for [AccountInfo<'a>; 3] {
    fn from(accounts: &DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.master_edition.clone(),
            accounts.reservation_list.clone(),
            accounts.resource.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct DeprecatedSetReservationListIxArgs {
    pub set_reservation_list_args: SetReservationListArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListIxData<'me>(pub &'me DeprecatedSetReservationListIxArgs);
impl<'me> From<&'me DeprecatedSetReservationListIxArgs>
    for DeprecatedSetReservationListIxData<'me>
{
    fn from(args: &'me DeprecatedSetReservationListIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedSetReservationListIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[5u8])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_set_reservation_list_ix<
    'k,
    'd,
    K: Into<DeprecatedSetReservationListKeys<'k>>,
    D: Into<DeprecatedSetReservationListIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedSetReservationListKeys = accounts.into();
    let metas: [AccountMeta; 3] = (&keys).into();
    let data: DeprecatedSetReservationListIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_set_reservation_list_invoke<
    'a,
    'd,
    D: Into<DeprecatedSetReservationListIxData<'d>>,
>(
    accounts: &DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = deprecated_set_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 3] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_set_reservation_list_invoke_signed<
    'a,
    'd,
    D: Into<DeprecatedSetReservationListIxData<'d>>,
>(
    accounts: &DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_set_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 3] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateReservationListAccounts<
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
    pub reservation_list: &'me AccountInfo<'a0>,
    pub payer: &'me AccountInfo<'a1>,
    pub update_authority: &'me AccountInfo<'a2>,
    pub master_edition: &'me AccountInfo<'a3>,
    pub resource: &'me AccountInfo<'a4>,
    pub metadata: &'me AccountInfo<'a5>,
    pub system_program: &'me AccountInfo<'a6>,
    pub rent: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateReservationListKeys<'me> {
    pub reservation_list: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub master_edition: &'me Pubkey,
    pub resource: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&DeprecatedCreateReservationListAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for DeprecatedCreateReservationListKeys<'me>
{
    fn from(
        accounts: &DeprecatedCreateReservationListAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            reservation_list: accounts.reservation_list.key,
            payer: accounts.payer.key,
            update_authority: accounts.update_authority.key,
            master_edition: accounts.master_edition.key,
            resource: accounts.resource.key,
            metadata: accounts.metadata.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&DeprecatedCreateReservationListKeys<'_>> for [AccountMeta; 8] {
    fn from(keys: &DeprecatedCreateReservationListKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.reservation_list, false),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.master_edition, false),
            AccountMeta::new_readonly(*keys.resource, false),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 8]
{
    fn from(
        accounts: &DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.reservation_list.clone(),
            accounts.payer.clone(),
            accounts.update_authority.clone(),
            accounts.master_edition.clone(),
            accounts.resource.clone(),
            accounts.metadata.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct DeprecatedCreateReservationListIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateReservationListIxData<'me>(
    pub &'me DeprecatedCreateReservationListIxArgs,
);
impl<'me> From<&'me DeprecatedCreateReservationListIxArgs>
    for DeprecatedCreateReservationListIxData<'me>
{
    fn from(args: &'me DeprecatedCreateReservationListIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedCreateReservationListIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[6u8])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_create_reservation_list_ix<
    'k,
    'd,
    K: Into<DeprecatedCreateReservationListKeys<'k>>,
    D: Into<DeprecatedCreateReservationListIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedCreateReservationListKeys = accounts.into();
    let metas: [AccountMeta; 8] = (&keys).into();
    let data: DeprecatedCreateReservationListIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_create_reservation_list_invoke<
    'a,
    'd,
    D: Into<DeprecatedCreateReservationListIxData<'d>>,
>(
    accounts: &DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = deprecated_create_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_create_reservation_list_invoke_signed<
    'a,
    'd,
    D: Into<DeprecatedCreateReservationListIxData<'d>>,
>(
    accounts: &DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_create_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub metadata: &'me AccountInfo<'a0>,
    pub creator: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataKeys<'me> {
    pub metadata: &'me Pubkey,
    pub creator: &'me Pubkey,
}
impl<'me> From<&SignMetadataAccounts<'me, '_, '_>> for SignMetadataKeys<'me> {
    fn from(accounts: &SignMetadataAccounts<'me, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            creator: accounts.creator.key,
        }
    }
}
impl From<&SignMetadataKeys<'_>> for [AccountMeta; 2] {
    fn from(keys: &SignMetadataKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.creator, true),
        ]
    }
}
impl<'a> From<&SignMetadataAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; 2] {
    fn from(accounts: &SignMetadataAccounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.creator.clone()]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct SignMetadataIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataIxData<'me>(pub &'me SignMetadataIxArgs);
impl<'me> From<&'me SignMetadataIxArgs> for SignMetadataIxData<'me> {
    fn from(args: &'me SignMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SignMetadataIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[7u8])?;
        self.0.serialize(writer)
    }
}
pub fn sign_metadata_ix<'k, 'd, K: Into<SignMetadataKeys<'k>>, D: Into<SignMetadataIxData<'d>>>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: SignMetadataKeys = accounts.into();
    let metas: [AccountMeta; 2] = (&keys).into();
    let data: SignMetadataIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sign_metadata_invoke<'a, 'd, D: Into<SignMetadataIxData<'d>>>(
    accounts: &SignMetadataAccounts<'_, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = sign_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sign_metadata_invoke_signed<'a, 'd, D: Into<SignMetadataIxData<'d>>>(
    accounts: &SignMetadataAccounts<'_, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sign_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenAccounts<
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
    pub destination: &'me AccountInfo<'a0>,
    pub token: &'me AccountInfo<'a1>,
    pub one_time_printing_authorization_mint: &'me AccountInfo<'a2>,
    pub printing_mint: &'me AccountInfo<'a3>,
    pub burn_authority: &'me AccountInfo<'a4>,
    pub metadata: &'me AccountInfo<'a5>,
    pub master_edition: &'me AccountInfo<'a6>,
    pub token_program: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenKeys<'me> {
    pub destination: &'me Pubkey,
    pub token: &'me Pubkey,
    pub one_time_printing_authorization_mint: &'me Pubkey,
    pub printing_mint: &'me Pubkey,
    pub burn_authority: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub master_edition: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me>
    From<&DeprecatedMintPrintingTokensViaTokenAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for DeprecatedMintPrintingTokensViaTokenKeys<'me>
{
    fn from(
        accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<
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
        >,
    ) -> Self {
        Self {
            destination: accounts.destination.key,
            token: accounts.token.key,
            one_time_printing_authorization_mint: accounts.one_time_printing_authorization_mint.key,
            printing_mint: accounts.printing_mint.key,
            burn_authority: accounts.burn_authority.key,
            metadata: accounts.metadata.key,
            master_edition: accounts.master_edition.key,
            token_program: accounts.token_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&DeprecatedMintPrintingTokensViaTokenKeys<'_>> for [AccountMeta; 9] {
    fn from(keys: &DeprecatedMintPrintingTokensViaTokenKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.destination, false),
            AccountMeta::new(*keys.token, false),
            AccountMeta::new(*keys.one_time_printing_authorization_mint, false),
            AccountMeta::new(*keys.printing_mint, false),
            AccountMeta::new_readonly(*keys.burn_authority, true),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.master_edition, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 9]
{
    fn from(
        accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<
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
        >,
    ) -> Self {
        [
            accounts.destination.clone(),
            accounts.token.clone(),
            accounts.one_time_printing_authorization_mint.clone(),
            accounts.printing_mint.clone(),
            accounts.burn_authority.clone(),
            accounts.metadata.clone(),
            accounts.master_edition.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenIxArgs {
    pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenIxData<'me>(
    pub &'me DeprecatedMintPrintingTokensViaTokenIxArgs,
);
impl<'me> From<&'me DeprecatedMintPrintingTokensViaTokenIxArgs>
    for DeprecatedMintPrintingTokensViaTokenIxData<'me>
{
    fn from(args: &'me DeprecatedMintPrintingTokensViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedMintPrintingTokensViaTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[8u8])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_mint_printing_tokens_via_token_ix<
    'k,
    'd,
    K: Into<DeprecatedMintPrintingTokensViaTokenKeys<'k>>,
    D: Into<DeprecatedMintPrintingTokensViaTokenIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedMintPrintingTokensViaTokenKeys = accounts.into();
    let metas: [AccountMeta; 9] = (&keys).into();
    let data: DeprecatedMintPrintingTokensViaTokenIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_printing_tokens_via_token_invoke<
    'a,
    'd,
    D: Into<DeprecatedMintPrintingTokensViaTokenIxData<'d>>,
>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_printing_tokens_via_token_invoke_signed<
    'a,
    'd,
    D: Into<DeprecatedMintPrintingTokensViaTokenIxData<'d>>,
>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub destination: &'me AccountInfo<'a0>,
    pub printing_mint: &'me AccountInfo<'a1>,
    pub update_authority: &'me AccountInfo<'a2>,
    pub metadata: &'me AccountInfo<'a3>,
    pub master_edition: &'me AccountInfo<'a4>,
    pub token_program: &'me AccountInfo<'a5>,
    pub rent: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensKeys<'me> {
    pub destination: &'me Pubkey,
    pub printing_mint: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub master_edition: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&DeprecatedMintPrintingTokensAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for DeprecatedMintPrintingTokensKeys<'me>
{
    fn from(
        accounts: &DeprecatedMintPrintingTokensAccounts<'me, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            destination: accounts.destination.key,
            printing_mint: accounts.printing_mint.key,
            update_authority: accounts.update_authority.key,
            metadata: accounts.metadata.key,
            master_edition: accounts.master_edition.key,
            token_program: accounts.token_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&DeprecatedMintPrintingTokensKeys<'_>> for [AccountMeta; 7] {
    fn from(keys: &DeprecatedMintPrintingTokensKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.destination, false),
            AccountMeta::new(*keys.printing_mint, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.master_edition, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 7]
{
    fn from(
        accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.destination.clone(),
            accounts.printing_mint.clone(),
            accounts.update_authority.clone(),
            accounts.metadata.clone(),
            accounts.master_edition.clone(),
            accounts.token_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensIxArgs {
    pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensIxData<'me>(pub &'me DeprecatedMintPrintingTokensIxArgs);
impl<'me> From<&'me DeprecatedMintPrintingTokensIxArgs>
    for DeprecatedMintPrintingTokensIxData<'me>
{
    fn from(args: &'me DeprecatedMintPrintingTokensIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedMintPrintingTokensIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[9u8])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_mint_printing_tokens_ix<
    'k,
    'd,
    K: Into<DeprecatedMintPrintingTokensKeys<'k>>,
    D: Into<DeprecatedMintPrintingTokensIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedMintPrintingTokensKeys = accounts.into();
    let metas: [AccountMeta; 7] = (&keys).into();
    let data: DeprecatedMintPrintingTokensIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_printing_tokens_invoke<
    'a,
    'd,
    D: Into<DeprecatedMintPrintingTokensIxData<'d>>,
>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 7] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_printing_tokens_invoke_signed<
    'a,
    'd,
    D: Into<DeprecatedMintPrintingTokensIxData<'d>>,
>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 7] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionAccounts<
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
    pub edition: &'me AccountInfo<'a0>,
    pub mint: &'me AccountInfo<'a1>,
    pub update_authority: &'me AccountInfo<'a2>,
    pub mint_authority: &'me AccountInfo<'a3>,
    pub payer: &'me AccountInfo<'a4>,
    pub metadata: &'me AccountInfo<'a5>,
    pub token_program: &'me AccountInfo<'a6>,
    pub system_program: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionKeys<'me> {
    pub edition: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub mint_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&CreateMasterEditionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMasterEditionKeys<'me>
{
    fn from(
        accounts: &CreateMasterEditionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            edition: accounts.edition.key,
            mint: accounts.mint.key,
            update_authority: accounts.update_authority.key,
            mint_authority: accounts.mint_authority.key,
            payer: accounts.payer.key,
            metadata: accounts.metadata.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&CreateMasterEditionKeys<'_>> for [AccountMeta; 9] {
    fn from(keys: &CreateMasterEditionKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.edition, false),
            AccountMeta::new(*keys.mint, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.mint_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 9]
{
    fn from(
        accounts: &CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.update_authority.clone(),
            accounts.mint_authority.clone(),
            accounts.payer.clone(),
            accounts.metadata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct CreateMasterEditionIxArgs {
    pub create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionIxData<'me>(pub &'me CreateMasterEditionIxArgs);
impl<'me> From<&'me CreateMasterEditionIxArgs> for CreateMasterEditionIxData<'me> {
    fn from(args: &'me CreateMasterEditionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMasterEditionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[10u8])?;
        self.0.serialize(writer)
    }
}
pub fn create_master_edition_ix<
    'k,
    'd,
    K: Into<CreateMasterEditionKeys<'k>>,
    D: Into<CreateMasterEditionIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: CreateMasterEditionKeys = accounts.into();
    let metas: [AccountMeta; 9] = (&keys).into();
    let data: CreateMasterEditionIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_master_edition_invoke<'a, 'd, D: Into<CreateMasterEditionIxData<'d>>>(
    accounts: &CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_master_edition_invoke_signed<'a, 'd, D: Into<CreateMasterEditionIxData<'d>>>(
    accounts: &CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenAccounts<
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
    'a11: 'me,
    'a12: 'me,
    'a13: 'me,
> {
    pub new_metadata: &'me AccountInfo<'a0>,
    pub new_edition: &'me AccountInfo<'a1>,
    pub master_edition: &'me AccountInfo<'a2>,
    pub new_mint: &'me AccountInfo<'a3>,
    pub edition_mark_pda: &'me AccountInfo<'a4>,
    pub new_mint_authority: &'me AccountInfo<'a5>,
    pub payer: &'me AccountInfo<'a6>,
    pub token_account_owner: &'me AccountInfo<'a7>,
    pub token_account: &'me AccountInfo<'a8>,
    pub new_metadata_update_authority: &'me AccountInfo<'a9>,
    pub metadata: &'me AccountInfo<'a10>,
    pub token_program: &'me AccountInfo<'a11>,
    pub system_program: &'me AccountInfo<'a12>,
    pub rent: &'me AccountInfo<'a13>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenKeys<'me> {
    pub new_metadata: &'me Pubkey,
    pub new_edition: &'me Pubkey,
    pub master_edition: &'me Pubkey,
    pub new_mint: &'me Pubkey,
    pub edition_mark_pda: &'me Pubkey,
    pub new_mint_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub token_account_owner: &'me Pubkey,
    pub token_account: &'me Pubkey,
    pub new_metadata_update_authority: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me>
    From<
        &MintNewEditionFromMasterEditionViaTokenAccounts<
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
            '_,
            '_,
            '_,
        >,
    > for MintNewEditionFromMasterEditionViaTokenKeys<'me>
{
    fn from(
        accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<
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
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            new_metadata: accounts.new_metadata.key,
            new_edition: accounts.new_edition.key,
            master_edition: accounts.master_edition.key,
            new_mint: accounts.new_mint.key,
            edition_mark_pda: accounts.edition_mark_pda.key,
            new_mint_authority: accounts.new_mint_authority.key,
            payer: accounts.payer.key,
            token_account_owner: accounts.token_account_owner.key,
            token_account: accounts.token_account.key,
            new_metadata_update_authority: accounts.new_metadata_update_authority.key,
            metadata: accounts.metadata.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&MintNewEditionFromMasterEditionViaTokenKeys<'_>> for [AccountMeta; 14] {
    fn from(keys: &MintNewEditionFromMasterEditionViaTokenKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.new_metadata, false),
            AccountMeta::new(*keys.new_edition, false),
            AccountMeta::new(*keys.master_edition, false),
            AccountMeta::new(*keys.new_mint, false),
            AccountMeta::new(*keys.edition_mark_pda, false),
            AccountMeta::new_readonly(*keys.new_mint_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.token_account_owner, true),
            AccountMeta::new_readonly(*keys.token_account, false),
            AccountMeta::new_readonly(*keys.new_metadata_update_authority, false),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a>
    From<
        &MintNewEditionFromMasterEditionViaTokenAccounts<
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
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 14]
{
    fn from(
        accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<
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
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.new_metadata.clone(),
            accounts.new_edition.clone(),
            accounts.master_edition.clone(),
            accounts.new_mint.clone(),
            accounts.edition_mark_pda.clone(),
            accounts.new_mint_authority.clone(),
            accounts.payer.clone(),
            accounts.token_account_owner.clone(),
            accounts.token_account.clone(),
            accounts.new_metadata_update_authority.clone(),
            accounts.metadata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenIxArgs {
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenIxData<'me>(
    pub &'me MintNewEditionFromMasterEditionViaTokenIxArgs,
);
impl<'me> From<&'me MintNewEditionFromMasterEditionViaTokenIxArgs>
    for MintNewEditionFromMasterEditionViaTokenIxData<'me>
{
    fn from(args: &'me MintNewEditionFromMasterEditionViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MintNewEditionFromMasterEditionViaTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[11u8])?;
        self.0.serialize(writer)
    }
}
pub fn mint_new_edition_from_master_edition_via_token_ix<
    'k,
    'd,
    K: Into<MintNewEditionFromMasterEditionViaTokenKeys<'k>>,
    D: Into<MintNewEditionFromMasterEditionViaTokenIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: MintNewEditionFromMasterEditionViaTokenKeys = accounts.into();
    let metas: [AccountMeta; 14] = (&keys).into();
    let data: MintNewEditionFromMasterEditionViaTokenIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_new_edition_from_master_edition_via_token_invoke<
    'a,
    'd,
    D: Into<MintNewEditionFromMasterEditionViaTokenIxData<'d>>,
>(
    accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: D,
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 14] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_new_edition_from_master_edition_via_token_invoke_signed<
    'a,
    'd,
    D: Into<MintNewEditionFromMasterEditionViaTokenIxData<'d>>,
>(
    accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<
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
        'a,
        'a,
        'a,
    >,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 14] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2Accounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    pub master_edition: &'me AccountInfo<'a0>,
    pub one_time_auth: &'me AccountInfo<'a1>,
    pub printing_mint: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2Keys<'me> {
    pub master_edition: &'me Pubkey,
    pub one_time_auth: &'me Pubkey,
    pub printing_mint: &'me Pubkey,
}
impl<'me> From<&ConvertMasterEditionV1ToV2Accounts<'me, '_, '_, '_>>
    for ConvertMasterEditionV1ToV2Keys<'me>
{
    fn from(accounts: &ConvertMasterEditionV1ToV2Accounts<'me, '_, '_, '_>) -> Self {
        Self {
            master_edition: accounts.master_edition.key,
            one_time_auth: accounts.one_time_auth.key,
            printing_mint: accounts.printing_mint.key,
        }
    }
}
impl From<&ConvertMasterEditionV1ToV2Keys<'_>> for [AccountMeta; 3] {
    fn from(keys: &ConvertMasterEditionV1ToV2Keys<'_>) -> Self {
        [
            AccountMeta::new(*keys.master_edition, false),
            AccountMeta::new(*keys.one_time_auth, false),
            AccountMeta::new(*keys.printing_mint, false),
        ]
    }
}
impl<'a> From<&ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>> for [AccountInfo<'a>; 3] {
    fn from(accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.master_edition.clone(),
            accounts.one_time_auth.clone(),
            accounts.printing_mint.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2IxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2IxData<'me>(pub &'me ConvertMasterEditionV1ToV2IxArgs);
impl<'me> From<&'me ConvertMasterEditionV1ToV2IxArgs> for ConvertMasterEditionV1ToV2IxData<'me> {
    fn from(args: &'me ConvertMasterEditionV1ToV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ConvertMasterEditionV1ToV2IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[12u8])?;
        self.0.serialize(writer)
    }
}
pub fn convert_master_edition_v1_to_v2_ix<
    'k,
    'd,
    K: Into<ConvertMasterEditionV1ToV2Keys<'k>>,
    D: Into<ConvertMasterEditionV1ToV2IxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: ConvertMasterEditionV1ToV2Keys = accounts.into();
    let metas: [AccountMeta; 3] = (&keys).into();
    let data: ConvertMasterEditionV1ToV2IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_master_edition_v1_to_v2_invoke<
    'a,
    'd,
    D: Into<ConvertMasterEditionV1ToV2IxData<'d>>,
>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = convert_master_edition_v1_to_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 3] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn convert_master_edition_v1_to_v2_invoke_signed<
    'a,
    'd,
    D: Into<ConvertMasterEditionV1ToV2IxData<'d>>,
>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = convert_master_edition_v1_to_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 3] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
    'a11: 'me,
    'a12: 'me,
    'a13: 'me,
    'a14: 'me,
    'a15: 'me,
    'a16: 'me,
> {
    pub new_metadata: &'me AccountInfo<'a0>,
    pub new_edition: &'me AccountInfo<'a1>,
    pub master_edition: &'me AccountInfo<'a2>,
    pub new_mint: &'me AccountInfo<'a3>,
    pub edition_mark_pda: &'me AccountInfo<'a4>,
    pub new_mint_authority: &'me AccountInfo<'a5>,
    pub payer: &'me AccountInfo<'a6>,
    pub vault_authority: &'me AccountInfo<'a7>,
    pub safety_deposit_store: &'me AccountInfo<'a8>,
    pub safety_deposit_box: &'me AccountInfo<'a9>,
    pub vault: &'me AccountInfo<'a10>,
    pub new_metadata_update_authority: &'me AccountInfo<'a11>,
    pub metadata: &'me AccountInfo<'a12>,
    pub token_program: &'me AccountInfo<'a13>,
    pub token_vault_program: &'me AccountInfo<'a14>,
    pub system_program: &'me AccountInfo<'a15>,
    pub rent: &'me AccountInfo<'a16>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyKeys<'me> {
    pub new_metadata: &'me Pubkey,
    pub new_edition: &'me Pubkey,
    pub master_edition: &'me Pubkey,
    pub new_mint: &'me Pubkey,
    pub edition_mark_pda: &'me Pubkey,
    pub new_mint_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub vault_authority: &'me Pubkey,
    pub safety_deposit_store: &'me Pubkey,
    pub safety_deposit_box: &'me Pubkey,
    pub vault: &'me Pubkey,
    pub new_metadata_update_authority: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub token_vault_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me>
    From<
        &MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    > for MintNewEditionFromMasterEditionViaVaultProxyKeys<'me>
{
    fn from(
        accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
            '_,
            '_,
            '_,
            '_,
            '_,
            '_,
        >,
    ) -> Self {
        Self {
            new_metadata: accounts.new_metadata.key,
            new_edition: accounts.new_edition.key,
            master_edition: accounts.master_edition.key,
            new_mint: accounts.new_mint.key,
            edition_mark_pda: accounts.edition_mark_pda.key,
            new_mint_authority: accounts.new_mint_authority.key,
            payer: accounts.payer.key,
            vault_authority: accounts.vault_authority.key,
            safety_deposit_store: accounts.safety_deposit_store.key,
            safety_deposit_box: accounts.safety_deposit_box.key,
            vault: accounts.vault.key,
            new_metadata_update_authority: accounts.new_metadata_update_authority.key,
            metadata: accounts.metadata.key,
            token_program: accounts.token_program.key,
            token_vault_program: accounts.token_vault_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&MintNewEditionFromMasterEditionViaVaultProxyKeys<'_>> for [AccountMeta; 17] {
    fn from(keys: &MintNewEditionFromMasterEditionViaVaultProxyKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.new_metadata, false),
            AccountMeta::new(*keys.new_edition, false),
            AccountMeta::new(*keys.master_edition, false),
            AccountMeta::new(*keys.new_mint, false),
            AccountMeta::new(*keys.edition_mark_pda, false),
            AccountMeta::new_readonly(*keys.new_mint_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.vault_authority, true),
            AccountMeta::new_readonly(*keys.safety_deposit_store, false),
            AccountMeta::new_readonly(*keys.safety_deposit_box, false),
            AccountMeta::new_readonly(*keys.vault, false),
            AccountMeta::new_readonly(*keys.new_metadata_update_authority, false),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.token_vault_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a>
    From<
        &MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    > for [AccountInfo<'a>; 17]
{
    fn from(
        accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
            'a,
            'a,
            'a,
            'a,
            'a,
            'a,
        >,
    ) -> Self {
        [
            accounts.new_metadata.clone(),
            accounts.new_edition.clone(),
            accounts.master_edition.clone(),
            accounts.new_mint.clone(),
            accounts.edition_mark_pda.clone(),
            accounts.new_mint_authority.clone(),
            accounts.payer.clone(),
            accounts.vault_authority.clone(),
            accounts.safety_deposit_store.clone(),
            accounts.safety_deposit_box.clone(),
            accounts.vault.clone(),
            accounts.new_metadata_update_authority.clone(),
            accounts.metadata.clone(),
            accounts.token_program.clone(),
            accounts.token_vault_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyIxArgs {
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyIxData<'me>(
    pub &'me MintNewEditionFromMasterEditionViaVaultProxyIxArgs,
);
impl<'me> From<&'me MintNewEditionFromMasterEditionViaVaultProxyIxArgs>
    for MintNewEditionFromMasterEditionViaVaultProxyIxData<'me>
{
    fn from(args: &'me MintNewEditionFromMasterEditionViaVaultProxyIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MintNewEditionFromMasterEditionViaVaultProxyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[13u8])?;
        self.0.serialize(writer)
    }
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_ix<
    'k,
    'd,
    K: Into<MintNewEditionFromMasterEditionViaVaultProxyKeys<'k>>,
    D: Into<MintNewEditionFromMasterEditionViaVaultProxyIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: MintNewEditionFromMasterEditionViaVaultProxyKeys = accounts.into();
    let metas: [AccountMeta; 17] = (&keys).into();
    let data: MintNewEditionFromMasterEditionViaVaultProxyIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_invoke<
    'a,
    'd,
    D: Into<MintNewEditionFromMasterEditionViaVaultProxyIxData<'d>>,
>(
    accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: D,
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_vault_proxy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 17] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_invoke_signed<
    'a,
    'd,
    D: Into<MintNewEditionFromMasterEditionViaVaultProxyIxData<'d>>,
>(
    accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<
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
        'a,
        'a,
        'a,
        'a,
        'a,
        'a,
    >,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_vault_proxy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 17] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataAccounts<'me, 'a0: 'me> {
    pub metadata: &'me AccountInfo<'a0>,
}
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataKeys<'me> {
    pub metadata: &'me Pubkey,
}
impl<'me> From<&PuffMetadataAccounts<'me, '_>> for PuffMetadataKeys<'me> {
    fn from(accounts: &PuffMetadataAccounts<'me, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
        }
    }
}
impl From<&PuffMetadataKeys<'_>> for [AccountMeta; 1] {
    fn from(keys: &PuffMetadataKeys<'_>) -> Self {
        [AccountMeta::new(*keys.metadata, false)]
    }
}
impl<'a> From<&PuffMetadataAccounts<'_, 'a>> for [AccountInfo<'a>; 1] {
    fn from(accounts: &PuffMetadataAccounts<'_, 'a>) -> Self {
        [accounts.metadata.clone()]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct PuffMetadataIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataIxData<'me>(pub &'me PuffMetadataIxArgs);
impl<'me> From<&'me PuffMetadataIxArgs> for PuffMetadataIxData<'me> {
    fn from(args: &'me PuffMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PuffMetadataIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[14u8])?;
        self.0.serialize(writer)
    }
}
pub fn puff_metadata_ix<'k, 'd, K: Into<PuffMetadataKeys<'k>>, D: Into<PuffMetadataIxData<'d>>>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: PuffMetadataKeys = accounts.into();
    let metas: [AccountMeta; 1] = (&keys).into();
    let data: PuffMetadataIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn puff_metadata_invoke<'a, 'd, D: Into<PuffMetadataIxData<'d>>>(
    accounts: &PuffMetadataAccounts<'_, 'a>,
    args: D,
) -> ProgramResult {
    let ix = puff_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 1] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn puff_metadata_invoke_signed<'a, 'd, D: Into<PuffMetadataIxData<'d>>>(
    accounts: &PuffMetadataAccounts<'_, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = puff_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 1] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2Accounts<'me, 'a0: 'me, 'a1: 'me> {
    pub metadata: &'me AccountInfo<'a0>,
    pub update_authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2Keys<'me> {
    pub metadata: &'me Pubkey,
    pub update_authority: &'me Pubkey,
}
impl<'me> From<&UpdateMetadataAccountV2Accounts<'me, '_, '_>> for UpdateMetadataAccountV2Keys<'me> {
    fn from(accounts: &UpdateMetadataAccountV2Accounts<'me, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            update_authority: accounts.update_authority.key,
        }
    }
}
impl From<&UpdateMetadataAccountV2Keys<'_>> for [AccountMeta; 2] {
    fn from(keys: &UpdateMetadataAccountV2Keys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
        ]
    }
}
impl<'a> From<&UpdateMetadataAccountV2Accounts<'_, 'a, 'a>> for [AccountInfo<'a>; 2] {
    fn from(accounts: &UpdateMetadataAccountV2Accounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.update_authority.clone()]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct UpdateMetadataAccountV2IxArgs {
    pub update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2IxData<'me>(pub &'me UpdateMetadataAccountV2IxArgs);
impl<'me> From<&'me UpdateMetadataAccountV2IxArgs> for UpdateMetadataAccountV2IxData<'me> {
    fn from(args: &'me UpdateMetadataAccountV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateMetadataAccountV2IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[15u8])?;
        self.0.serialize(writer)
    }
}
pub fn update_metadata_account_v2_ix<
    'k,
    'd,
    K: Into<UpdateMetadataAccountV2Keys<'k>>,
    D: Into<UpdateMetadataAccountV2IxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: UpdateMetadataAccountV2Keys = accounts.into();
    let metas: [AccountMeta; 2] = (&keys).into();
    let data: UpdateMetadataAccountV2IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_metadata_account_v2_invoke<'a, 'd, D: Into<UpdateMetadataAccountV2IxData<'d>>>(
    accounts: &UpdateMetadataAccountV2Accounts<'_, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = update_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_metadata_account_v2_invoke_signed<
    'a,
    'd,
    D: Into<UpdateMetadataAccountV2IxData<'d>>,
>(
    accounts: &UpdateMetadataAccountV2Accounts<'_, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountV2Accounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
    'a6: 'me,
> {
    pub metadata: &'me AccountInfo<'a0>,
    pub mint: &'me AccountInfo<'a1>,
    pub mint_authority: &'me AccountInfo<'a2>,
    pub payer: &'me AccountInfo<'a3>,
    pub update_authority: &'me AccountInfo<'a4>,
    pub system_program: &'me AccountInfo<'a5>,
    pub rent: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountV2Keys<'me> {
    pub metadata: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub mint_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&CreateMetadataAccountV2Accounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMetadataAccountV2Keys<'me>
{
    fn from(accounts: &CreateMetadataAccountV2Accounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            mint: accounts.mint.key,
            mint_authority: accounts.mint_authority.key,
            payer: accounts.payer.key,
            update_authority: accounts.update_authority.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&CreateMetadataAccountV2Keys<'_>> for [AccountMeta; 7] {
    fn from(keys: &CreateMetadataAccountV2Keys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.mint_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.update_authority, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 7]
{
    fn from(accounts: &CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.mint.clone(),
            accounts.mint_authority.clone(),
            accounts.payer.clone(),
            accounts.update_authority.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct CreateMetadataAccountV2IxArgs {
    pub create_metadata_account_args_v2: CreateMetadataAccountArgsV2,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountV2IxData<'me>(pub &'me CreateMetadataAccountV2IxArgs);
impl<'me> From<&'me CreateMetadataAccountV2IxArgs> for CreateMetadataAccountV2IxData<'me> {
    fn from(args: &'me CreateMetadataAccountV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMetadataAccountV2IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[16u8])?;
        self.0.serialize(writer)
    }
}
pub fn create_metadata_account_v2_ix<
    'k,
    'd,
    K: Into<CreateMetadataAccountV2Keys<'k>>,
    D: Into<CreateMetadataAccountV2IxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: CreateMetadataAccountV2Keys = accounts.into();
    let metas: [AccountMeta; 7] = (&keys).into();
    let data: CreateMetadataAccountV2IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_account_v2_invoke<'a, 'd, D: Into<CreateMetadataAccountV2IxData<'d>>>(
    accounts: &CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = create_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 7] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_metadata_account_v2_invoke_signed<
    'a,
    'd,
    D: Into<CreateMetadataAccountV2IxData<'d>>,
>(
    accounts: &CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 7] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionV3Accounts<
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
    pub edition: &'me AccountInfo<'a0>,
    pub mint: &'me AccountInfo<'a1>,
    pub update_authority: &'me AccountInfo<'a2>,
    pub mint_authority: &'me AccountInfo<'a3>,
    pub payer: &'me AccountInfo<'a4>,
    pub metadata: &'me AccountInfo<'a5>,
    pub token_program: &'me AccountInfo<'a6>,
    pub system_program: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionV3Keys<'me> {
    pub edition: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub mint_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&CreateMasterEditionV3Accounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMasterEditionV3Keys<'me>
{
    fn from(
        accounts: &CreateMasterEditionV3Accounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            edition: accounts.edition.key,
            mint: accounts.mint.key,
            update_authority: accounts.update_authority.key,
            mint_authority: accounts.mint_authority.key,
            payer: accounts.payer.key,
            metadata: accounts.metadata.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&CreateMasterEditionV3Keys<'_>> for [AccountMeta; 9] {
    fn from(keys: &CreateMasterEditionV3Keys<'_>) -> Self {
        [
            AccountMeta::new(*keys.edition, false),
            AccountMeta::new(*keys.mint, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.mint_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 9]
{
    fn from(
        accounts: &CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.update_authority.clone(),
            accounts.mint_authority.clone(),
            accounts.payer.clone(),
            accounts.metadata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct CreateMasterEditionV3IxArgs {
    pub create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionV3IxData<'me>(pub &'me CreateMasterEditionV3IxArgs);
impl<'me> From<&'me CreateMasterEditionV3IxArgs> for CreateMasterEditionV3IxData<'me> {
    fn from(args: &'me CreateMasterEditionV3IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMasterEditionV3IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[17u8])?;
        self.0.serialize(writer)
    }
}
pub fn create_master_edition_v3_ix<
    'k,
    'd,
    K: Into<CreateMasterEditionV3Keys<'k>>,
    D: Into<CreateMasterEditionV3IxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: CreateMasterEditionV3Keys = accounts.into();
    let metas: [AccountMeta; 9] = (&keys).into();
    let data: CreateMasterEditionV3IxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_master_edition_v3_invoke<'a, 'd, D: Into<CreateMasterEditionV3IxData<'d>>>(
    accounts: &CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = create_master_edition_v3_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_master_edition_v3_invoke_signed<'a, 'd, D: Into<CreateMasterEditionV3IxData<'d>>>(
    accounts: &CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_master_edition_v3_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me>
{
    pub metadata: &'me AccountInfo<'a0>,
    pub collection_authority: &'me AccountInfo<'a1>,
    pub payer: &'me AccountInfo<'a2>,
    pub collection_mint: &'me AccountInfo<'a3>,
    pub collection: &'me AccountInfo<'a4>,
    pub collection_master_edition_account: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionKeys<'me> {
    pub metadata: &'me Pubkey,
    pub collection_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub collection_mint: &'me Pubkey,
    pub collection: &'me Pubkey,
    pub collection_master_edition_account: &'me Pubkey,
}
impl<'me> From<&VerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for VerifyCollectionKeys<'me>
{
    fn from(accounts: &VerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            collection_authority: accounts.collection_authority.key,
            payer: accounts.payer.key,
            collection_mint: accounts.collection_mint.key,
            collection: accounts.collection.key,
            collection_master_edition_account: accounts.collection_master_edition_account.key,
        }
    }
}
impl From<&VerifyCollectionKeys<'_>> for [AccountMeta; 6] {
    fn from(keys: &VerifyCollectionKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.collection_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.collection_mint, false),
            AccountMeta::new_readonly(*keys.collection, false),
            AccountMeta::new_readonly(*keys.collection_master_edition_account, false),
        ]
    }
}
impl<'a> From<&VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 6] {
    fn from(accounts: &VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.collection_authority.clone(),
            accounts.payer.clone(),
            accounts.collection_mint.clone(),
            accounts.collection.clone(),
            accounts.collection_master_edition_account.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct VerifyCollectionIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionIxData<'me>(pub &'me VerifyCollectionIxArgs);
impl<'me> From<&'me VerifyCollectionIxArgs> for VerifyCollectionIxData<'me> {
    fn from(args: &'me VerifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for VerifyCollectionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[18u8])?;
        self.0.serialize(writer)
    }
}
pub fn verify_collection_ix<
    'k,
    'd,
    K: Into<VerifyCollectionKeys<'k>>,
    D: Into<VerifyCollectionIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: VerifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; 6] = (&keys).into();
    let data: VerifyCollectionIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn verify_collection_invoke<'a, 'd, D: Into<VerifyCollectionIxData<'d>>>(
    accounts: &VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 6] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn verify_collection_invoke_signed<'a, 'd, D: Into<VerifyCollectionIxData<'d>>>(
    accounts: &VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 6] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct UtilizeAccounts<
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
    pub metadata: &'me AccountInfo<'a0>,
    pub token_account: &'me AccountInfo<'a1>,
    pub mint: &'me AccountInfo<'a2>,
    pub use_authority: &'me AccountInfo<'a3>,
    pub owner: &'me AccountInfo<'a4>,
    pub token_program: &'me AccountInfo<'a5>,
    pub ata_program: &'me AccountInfo<'a6>,
    pub system_program: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
    pub use_authority_record: &'me AccountInfo<'a9>,
    pub burner: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct UtilizeKeys<'me> {
    pub metadata: &'me Pubkey,
    pub token_account: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub use_authority: &'me Pubkey,
    pub owner: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub ata_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
    pub use_authority_record: &'me Pubkey,
    pub burner: &'me Pubkey,
}
impl<'me> From<&UtilizeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for UtilizeKeys<'me>
{
    fn from(accounts: &UtilizeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            token_account: accounts.token_account.key,
            mint: accounts.mint.key,
            use_authority: accounts.use_authority.key,
            owner: accounts.owner.key,
            token_program: accounts.token_program.key,
            ata_program: accounts.ata_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
            use_authority_record: accounts.use_authority_record.key,
            burner: accounts.burner.key,
        }
    }
}
impl From<&UtilizeKeys<'_>> for [AccountMeta; 11] {
    fn from(keys: &UtilizeKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new(*keys.token_account, false),
            AccountMeta::new(*keys.mint, false),
            AccountMeta::new_readonly(*keys.use_authority, true),
            AccountMeta::new_readonly(*keys.owner, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.ata_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
            AccountMeta::new(*keys.use_authority_record, false),
            AccountMeta::new_readonly(*keys.burner, false),
        ]
    }
}
impl<'a> From<&UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 11]
{
    fn from(accounts: &UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.token_account.clone(),
            accounts.mint.clone(),
            accounts.use_authority.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
            accounts.ata_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
            accounts.use_authority_record.clone(),
            accounts.burner.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct UtilizeIxArgs {
    pub utilize_args: UtilizeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct UtilizeIxData<'me>(pub &'me UtilizeIxArgs);
impl<'me> From<&'me UtilizeIxArgs> for UtilizeIxData<'me> {
    fn from(args: &'me UtilizeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UtilizeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[19u8])?;
        self.0.serialize(writer)
    }
}
pub fn utilize_ix<'k, 'd, K: Into<UtilizeKeys<'k>>, D: Into<UtilizeIxData<'d>>>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: UtilizeKeys = accounts.into();
    let metas: [AccountMeta; 11] = (&keys).into();
    let data: UtilizeIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn utilize_invoke<'a, 'd, D: Into<UtilizeIxData<'d>>>(
    accounts: &UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = utilize_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 11] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn utilize_invoke_signed<'a, 'd, D: Into<UtilizeIxData<'d>>>(
    accounts: &UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = utilize_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 11] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityAccounts<
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
    pub use_authority_record: &'me AccountInfo<'a0>,
    pub owner: &'me AccountInfo<'a1>,
    pub payer: &'me AccountInfo<'a2>,
    pub user: &'me AccountInfo<'a3>,
    pub owner_token_account: &'me AccountInfo<'a4>,
    pub metadata: &'me AccountInfo<'a5>,
    pub mint: &'me AccountInfo<'a6>,
    pub burner: &'me AccountInfo<'a7>,
    pub token_program: &'me AccountInfo<'a8>,
    pub system_program: &'me AccountInfo<'a9>,
    pub rent: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityKeys<'me> {
    pub use_authority_record: &'me Pubkey,
    pub owner: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub user: &'me Pubkey,
    pub owner_token_account: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub burner: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&ApproveUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for ApproveUseAuthorityKeys<'me>
{
    fn from(
        accounts: &ApproveUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            use_authority_record: accounts.use_authority_record.key,
            owner: accounts.owner.key,
            payer: accounts.payer.key,
            user: accounts.user.key,
            owner_token_account: accounts.owner_token_account.key,
            metadata: accounts.metadata.key,
            mint: accounts.mint.key,
            burner: accounts.burner.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&ApproveUseAuthorityKeys<'_>> for [AccountMeta; 11] {
    fn from(keys: &ApproveUseAuthorityKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.use_authority_record, false),
            AccountMeta::new_readonly(*keys.owner, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.user, false),
            AccountMeta::new(*keys.owner_token_account, false),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.burner, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 11]
{
    fn from(
        accounts: &ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.use_authority_record.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.user.clone(),
            accounts.owner_token_account.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
            accounts.burner.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct ApproveUseAuthorityIxArgs {
    pub approve_use_authority_args: ApproveUseAuthorityArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityIxData<'me>(pub &'me ApproveUseAuthorityIxArgs);
impl<'me> From<&'me ApproveUseAuthorityIxArgs> for ApproveUseAuthorityIxData<'me> {
    fn from(args: &'me ApproveUseAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ApproveUseAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[20u8])?;
        self.0.serialize(writer)
    }
}
pub fn approve_use_authority_ix<
    'k,
    'd,
    K: Into<ApproveUseAuthorityKeys<'k>>,
    D: Into<ApproveUseAuthorityIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: ApproveUseAuthorityKeys = accounts.into();
    let metas: [AccountMeta; 11] = (&keys).into();
    let data: ApproveUseAuthorityIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_use_authority_invoke<'a, 'd, D: Into<ApproveUseAuthorityIxData<'d>>>(
    accounts: &ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = approve_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 11] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_use_authority_invoke_signed<'a, 'd, D: Into<ApproveUseAuthorityIxData<'d>>>(
    accounts: &ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = approve_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 11] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeUseAuthorityAccounts<
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
    pub use_authority_record: &'me AccountInfo<'a0>,
    pub owner: &'me AccountInfo<'a1>,
    pub user: &'me AccountInfo<'a2>,
    pub owner_token_account: &'me AccountInfo<'a3>,
    pub mint: &'me AccountInfo<'a4>,
    pub metadata: &'me AccountInfo<'a5>,
    pub token_program: &'me AccountInfo<'a6>,
    pub system_program: &'me AccountInfo<'a7>,
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeUseAuthorityKeys<'me> {
    pub use_authority_record: &'me Pubkey,
    pub owner: &'me Pubkey,
    pub user: &'me Pubkey,
    pub owner_token_account: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub token_program: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&RevokeUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for RevokeUseAuthorityKeys<'me>
{
    fn from(
        accounts: &RevokeUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            use_authority_record: accounts.use_authority_record.key,
            owner: accounts.owner.key,
            user: accounts.user.key,
            owner_token_account: accounts.owner_token_account.key,
            mint: accounts.mint.key,
            metadata: accounts.metadata.key,
            token_program: accounts.token_program.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&RevokeUseAuthorityKeys<'_>> for [AccountMeta; 9] {
    fn from(keys: &RevokeUseAuthorityKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.use_authority_record, false),
            AccountMeta::new_readonly(*keys.owner, true),
            AccountMeta::new_readonly(*keys.user, false),
            AccountMeta::new(*keys.owner_token_account, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.token_program, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 9]
{
    fn from(accounts: &RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.use_authority_record.clone(),
            accounts.owner.clone(),
            accounts.user.clone(),
            accounts.owner_token_account.clone(),
            accounts.mint.clone(),
            accounts.metadata.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct RevokeUseAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RevokeUseAuthorityIxData<'me>(pub &'me RevokeUseAuthorityIxArgs);
impl<'me> From<&'me RevokeUseAuthorityIxArgs> for RevokeUseAuthorityIxData<'me> {
    fn from(args: &'me RevokeUseAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RevokeUseAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[21u8])?;
        self.0.serialize(writer)
    }
}
pub fn revoke_use_authority_ix<
    'k,
    'd,
    K: Into<RevokeUseAuthorityKeys<'k>>,
    D: Into<RevokeUseAuthorityIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: RevokeUseAuthorityKeys = accounts.into();
    let metas: [AccountMeta; 9] = (&keys).into();
    let data: RevokeUseAuthorityIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_use_authority_invoke<'a, 'd, D: Into<RevokeUseAuthorityIxData<'d>>>(
    accounts: &RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = revoke_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_use_authority_invoke_signed<'a, 'd, D: Into<RevokeUseAuthorityIxData<'d>>>(
    accounts: &RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revoke_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 9] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct UnverifyCollectionAccounts<
    'me,
    'a0: 'me,
    'a1: 'me,
    'a2: 'me,
    'a3: 'me,
    'a4: 'me,
    'a5: 'me,
> {
    pub metadata: &'me AccountInfo<'a0>,
    pub collection_authority: &'me AccountInfo<'a1>,
    pub collection_mint: &'me AccountInfo<'a2>,
    pub collection: &'me AccountInfo<'a3>,
    pub collection_master_edition_account: &'me AccountInfo<'a4>,
    pub collection_authority_record: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct UnverifyCollectionKeys<'me> {
    pub metadata: &'me Pubkey,
    pub collection_authority: &'me Pubkey,
    pub collection_mint: &'me Pubkey,
    pub collection: &'me Pubkey,
    pub collection_master_edition_account: &'me Pubkey,
    pub collection_authority_record: &'me Pubkey,
}
impl<'me> From<&UnverifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for UnverifyCollectionKeys<'me>
{
    fn from(accounts: &UnverifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            collection_authority: accounts.collection_authority.key,
            collection_mint: accounts.collection_mint.key,
            collection: accounts.collection.key,
            collection_master_edition_account: accounts.collection_master_edition_account.key,
            collection_authority_record: accounts.collection_authority_record.key,
        }
    }
}
impl From<&UnverifyCollectionKeys<'_>> for [AccountMeta; 6] {
    fn from(keys: &UnverifyCollectionKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.collection_authority, true),
            AccountMeta::new_readonly(*keys.collection_mint, false),
            AccountMeta::new_readonly(*keys.collection, false),
            AccountMeta::new_readonly(*keys.collection_master_edition_account, false),
            AccountMeta::new_readonly(*keys.collection_authority_record, false),
        ]
    }
}
impl<'a> From<&UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 6] {
    fn from(accounts: &UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.collection_authority.clone(),
            accounts.collection_mint.clone(),
            accounts.collection.clone(),
            accounts.collection_master_edition_account.clone(),
            accounts.collection_authority_record.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct UnverifyCollectionIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UnverifyCollectionIxData<'me>(pub &'me UnverifyCollectionIxArgs);
impl<'me> From<&'me UnverifyCollectionIxArgs> for UnverifyCollectionIxData<'me> {
    fn from(args: &'me UnverifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UnverifyCollectionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[22u8])?;
        self.0.serialize(writer)
    }
}
pub fn unverify_collection_ix<
    'k,
    'd,
    K: Into<UnverifyCollectionKeys<'k>>,
    D: Into<UnverifyCollectionIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: UnverifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; 6] = (&keys).into();
    let data: UnverifyCollectionIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unverify_collection_invoke<'a, 'd, D: Into<UnverifyCollectionIxData<'d>>>(
    accounts: &UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = unverify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 6] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn unverify_collection_invoke_signed<'a, 'd, D: Into<UnverifyCollectionIxData<'d>>>(
    accounts: &UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = unverify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 6] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityAccounts<
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
    pub collection_authority_record: &'me AccountInfo<'a0>,
    pub new_collection_authority: &'me AccountInfo<'a1>,
    pub update_authority: &'me AccountInfo<'a2>,
    pub payer: &'me AccountInfo<'a3>,
    pub metadata: &'me AccountInfo<'a4>,
    pub mint: &'me AccountInfo<'a5>,
    pub system_program: &'me AccountInfo<'a6>,
    pub rent: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityKeys<'me> {
    pub collection_authority_record: &'me Pubkey,
    pub new_collection_authority: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub system_program: &'me Pubkey,
    pub rent: &'me Pubkey,
}
impl<'me> From<&ApproveCollectionAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for ApproveCollectionAuthorityKeys<'me>
{
    fn from(
        accounts: &ApproveCollectionAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            collection_authority_record: accounts.collection_authority_record.key,
            new_collection_authority: accounts.new_collection_authority.key,
            update_authority: accounts.update_authority.key,
            payer: accounts.payer.key,
            metadata: accounts.metadata.key,
            mint: accounts.mint.key,
            system_program: accounts.system_program.key,
            rent: accounts.rent.key,
        }
    }
}
impl From<&ApproveCollectionAuthorityKeys<'_>> for [AccountMeta; 8] {
    fn from(keys: &ApproveCollectionAuthorityKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.collection_authority_record, false),
            AccountMeta::new_readonly(*keys.new_collection_authority, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.system_program, false),
            AccountMeta::new_readonly(*keys.rent, false),
        ]
    }
}
impl<'a> From<&ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 8]
{
    fn from(
        accounts: &ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    ) -> Self {
        [
            accounts.collection_authority_record.clone(),
            accounts.new_collection_authority.clone(),
            accounts.update_authority.clone(),
            accounts.payer.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
            accounts.system_program.clone(),
            accounts.rent.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct ApproveCollectionAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityIxData<'me>(pub &'me ApproveCollectionAuthorityIxArgs);
impl<'me> From<&'me ApproveCollectionAuthorityIxArgs> for ApproveCollectionAuthorityIxData<'me> {
    fn from(args: &'me ApproveCollectionAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ApproveCollectionAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[23u8])?;
        self.0.serialize(writer)
    }
}
pub fn approve_collection_authority_ix<
    'k,
    'd,
    K: Into<ApproveCollectionAuthorityKeys<'k>>,
    D: Into<ApproveCollectionAuthorityIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: ApproveCollectionAuthorityKeys = accounts.into();
    let metas: [AccountMeta; 8] = (&keys).into();
    let data: ApproveCollectionAuthorityIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_collection_authority_invoke<
    'a,
    'd,
    D: Into<ApproveCollectionAuthorityIxData<'d>>,
>(
    accounts: &ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = approve_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_collection_authority_invoke_signed<
    'a,
    'd,
    D: Into<ApproveCollectionAuthorityIxData<'d>>,
>(
    accounts: &ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = approve_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    pub collection_authority_record: &'me AccountInfo<'a0>,
    pub update_authority: &'me AccountInfo<'a1>,
    pub metadata: &'me AccountInfo<'a2>,
    pub mint: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityKeys<'me> {
    pub collection_authority_record: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub metadata: &'me Pubkey,
    pub mint: &'me Pubkey,
}
impl<'me> From<&RevokeCollectionAuthorityAccounts<'me, '_, '_, '_, '_>>
    for RevokeCollectionAuthorityKeys<'me>
{
    fn from(accounts: &RevokeCollectionAuthorityAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            collection_authority_record: accounts.collection_authority_record.key,
            update_authority: accounts.update_authority.key,
            metadata: accounts.metadata.key,
            mint: accounts.mint.key,
        }
    }
}
impl From<&RevokeCollectionAuthorityKeys<'_>> for [AccountMeta; 4] {
    fn from(keys: &RevokeCollectionAuthorityKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.collection_authority_record, false),
            AccountMeta::new_readonly(*keys.update_authority, true),
            AccountMeta::new_readonly(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.mint, false),
        ]
    }
}
impl<'a> From<&RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 4] {
    fn from(accounts: &RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.collection_authority_record.clone(),
            accounts.update_authority.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct RevokeCollectionAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityIxData<'me>(pub &'me RevokeCollectionAuthorityIxArgs);
impl<'me> From<&'me RevokeCollectionAuthorityIxArgs> for RevokeCollectionAuthorityIxData<'me> {
    fn from(args: &'me RevokeCollectionAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RevokeCollectionAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[24u8])?;
        self.0.serialize(writer)
    }
}
pub fn revoke_collection_authority_ix<
    'k,
    'd,
    K: Into<RevokeCollectionAuthorityKeys<'k>>,
    D: Into<RevokeCollectionAuthorityIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: RevokeCollectionAuthorityKeys = accounts.into();
    let metas: [AccountMeta; 4] = (&keys).into();
    let data: RevokeCollectionAuthorityIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_collection_authority_invoke<'a, 'd, D: Into<RevokeCollectionAuthorityIxData<'d>>>(
    accounts: &RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = revoke_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 4] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_collection_authority_invoke_signed<
    'a,
    'd,
    D: Into<RevokeCollectionAuthorityIxData<'d>>,
>(
    accounts: &RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revoke_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 4] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct SetAndVerifyCollectionAccounts<
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
    pub metadata: &'me AccountInfo<'a0>,
    pub collection_authority: &'me AccountInfo<'a1>,
    pub payer: &'me AccountInfo<'a2>,
    pub update_authority: &'me AccountInfo<'a3>,
    pub collection_mint: &'me AccountInfo<'a4>,
    pub collection: &'me AccountInfo<'a5>,
    pub collection_master_edition_account: &'me AccountInfo<'a6>,
    pub collection_authority_record: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetAndVerifyCollectionKeys<'me> {
    pub metadata: &'me Pubkey,
    pub collection_authority: &'me Pubkey,
    pub payer: &'me Pubkey,
    pub update_authority: &'me Pubkey,
    pub collection_mint: &'me Pubkey,
    pub collection: &'me Pubkey,
    pub collection_master_edition_account: &'me Pubkey,
    pub collection_authority_record: &'me Pubkey,
}
impl<'me> From<&SetAndVerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for SetAndVerifyCollectionKeys<'me>
{
    fn from(
        accounts: &SetAndVerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            metadata: accounts.metadata.key,
            collection_authority: accounts.collection_authority.key,
            payer: accounts.payer.key,
            update_authority: accounts.update_authority.key,
            collection_mint: accounts.collection_mint.key,
            collection: accounts.collection.key,
            collection_master_edition_account: accounts.collection_master_edition_account.key,
            collection_authority_record: accounts.collection_authority_record.key,
        }
    }
}
impl From<&SetAndVerifyCollectionKeys<'_>> for [AccountMeta; 8] {
    fn from(keys: &SetAndVerifyCollectionKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.collection_authority, true),
            AccountMeta::new_readonly(*keys.payer, true),
            AccountMeta::new_readonly(*keys.update_authority, false),
            AccountMeta::new_readonly(*keys.collection_mint, false),
            AccountMeta::new_readonly(*keys.collection, false),
            AccountMeta::new_readonly(*keys.collection_master_edition_account, false),
            AccountMeta::new_readonly(*keys.collection_authority_record, false),
        ]
    }
}
impl<'a> From<&SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; 8]
{
    fn from(accounts: &SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.collection_authority.clone(),
            accounts.payer.clone(),
            accounts.update_authority.clone(),
            accounts.collection_mint.clone(),
            accounts.collection.clone(),
            accounts.collection_master_edition_account.clone(),
            accounts.collection_authority_record.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct SetAndVerifyCollectionIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SetAndVerifyCollectionIxData<'me>(pub &'me SetAndVerifyCollectionIxArgs);
impl<'me> From<&'me SetAndVerifyCollectionIxArgs> for SetAndVerifyCollectionIxData<'me> {
    fn from(args: &'me SetAndVerifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetAndVerifyCollectionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[25u8])?;
        self.0.serialize(writer)
    }
}
pub fn set_and_verify_collection_ix<
    'k,
    'd,
    K: Into<SetAndVerifyCollectionKeys<'k>>,
    D: Into<SetAndVerifyCollectionIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: SetAndVerifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; 8] = (&keys).into();
    let data: SetAndVerifyCollectionIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_and_verify_collection_invoke<'a, 'd, D: Into<SetAndVerifyCollectionIxData<'d>>>(
    accounts: &SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = set_and_verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_and_verify_collection_invoke_signed<
    'a,
    'd,
    D: Into<SetAndVerifyCollectionIxData<'d>>,
>(
    accounts: &SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_and_verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 8] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    pub delegate: &'me AccountInfo<'a0>,
    pub token_account: &'me AccountInfo<'a1>,
    pub edition: &'me AccountInfo<'a2>,
    pub mint: &'me AccountInfo<'a3>,
    pub token_program: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountKeys<'me> {
    pub delegate: &'me Pubkey,
    pub token_account: &'me Pubkey,
    pub edition: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub token_program: &'me Pubkey,
}
impl<'me> From<&FreezeDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>>
    for FreezeDelegatedAccountKeys<'me>
{
    fn from(accounts: &FreezeDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            delegate: accounts.delegate.key,
            token_account: accounts.token_account.key,
            edition: accounts.edition.key,
            mint: accounts.mint.key,
            token_program: accounts.token_program.key,
        }
    }
}
impl From<&FreezeDelegatedAccountKeys<'_>> for [AccountMeta; 5] {
    fn from(keys: &FreezeDelegatedAccountKeys<'_>) -> Self {
        [
            AccountMeta::new_readonly(*keys.delegate, true),
            AccountMeta::new(*keys.token_account, false),
            AccountMeta::new_readonly(*keys.edition, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.token_program, false),
        ]
    }
}
impl<'a> From<&FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 5] {
    fn from(accounts: &FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.delegate.clone(),
            accounts.token_account.clone(),
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct FreezeDelegatedAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountIxData<'me>(pub &'me FreezeDelegatedAccountIxArgs);
impl<'me> From<&'me FreezeDelegatedAccountIxArgs> for FreezeDelegatedAccountIxData<'me> {
    fn from(args: &'me FreezeDelegatedAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for FreezeDelegatedAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[26u8])?;
        self.0.serialize(writer)
    }
}
pub fn freeze_delegated_account_ix<
    'k,
    'd,
    K: Into<FreezeDelegatedAccountKeys<'k>>,
    D: Into<FreezeDelegatedAccountIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: FreezeDelegatedAccountKeys = accounts.into();
    let metas: [AccountMeta; 5] = (&keys).into();
    let data: FreezeDelegatedAccountIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn freeze_delegated_account_invoke<'a, 'd, D: Into<FreezeDelegatedAccountIxData<'d>>>(
    accounts: &FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = freeze_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn freeze_delegated_account_invoke_signed<'a, 'd, D: Into<FreezeDelegatedAccountIxData<'d>>>(
    accounts: &FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = freeze_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    pub delegate: &'me AccountInfo<'a0>,
    pub token_account: &'me AccountInfo<'a1>,
    pub edition: &'me AccountInfo<'a2>,
    pub mint: &'me AccountInfo<'a3>,
    pub token_program: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountKeys<'me> {
    pub delegate: &'me Pubkey,
    pub token_account: &'me Pubkey,
    pub edition: &'me Pubkey,
    pub mint: &'me Pubkey,
    pub token_program: &'me Pubkey,
}
impl<'me> From<&ThawDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>>
    for ThawDelegatedAccountKeys<'me>
{
    fn from(accounts: &ThawDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            delegate: accounts.delegate.key,
            token_account: accounts.token_account.key,
            edition: accounts.edition.key,
            mint: accounts.mint.key,
            token_program: accounts.token_program.key,
        }
    }
}
impl From<&ThawDelegatedAccountKeys<'_>> for [AccountMeta; 5] {
    fn from(keys: &ThawDelegatedAccountKeys<'_>) -> Self {
        [
            AccountMeta::new_readonly(*keys.delegate, true),
            AccountMeta::new(*keys.token_account, false),
            AccountMeta::new_readonly(*keys.edition, false),
            AccountMeta::new_readonly(*keys.mint, false),
            AccountMeta::new_readonly(*keys.token_program, false),
        ]
    }
}
impl<'a> From<&ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>> for [AccountInfo<'a>; 5] {
    fn from(accounts: &ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.delegate.clone(),
            accounts.token_account.clone(),
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct ThawDelegatedAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountIxData<'me>(pub &'me ThawDelegatedAccountIxArgs);
impl<'me> From<&'me ThawDelegatedAccountIxArgs> for ThawDelegatedAccountIxData<'me> {
    fn from(args: &'me ThawDelegatedAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ThawDelegatedAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[27u8])?;
        self.0.serialize(writer)
    }
}
pub fn thaw_delegated_account_ix<
    'k,
    'd,
    K: Into<ThawDelegatedAccountKeys<'k>>,
    D: Into<ThawDelegatedAccountIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: ThawDelegatedAccountKeys = accounts.into();
    let metas: [AccountMeta; 5] = (&keys).into();
    let data: ThawDelegatedAccountIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn thaw_delegated_account_invoke<'a, 'd, D: Into<ThawDelegatedAccountIxData<'d>>>(
    accounts: &ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = thaw_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn thaw_delegated_account_invoke_signed<'a, 'd, D: Into<ThawDelegatedAccountIxData<'d>>>(
    accounts: &ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = thaw_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 5] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationAccounts<'me, 'a0: 'me, 'a1: 'me> {
    pub metadata: &'me AccountInfo<'a0>,
    pub creator: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationKeys<'me> {
    pub metadata: &'me Pubkey,
    pub creator: &'me Pubkey,
}
impl<'me> From<&RemoveCreatorVerificationAccounts<'me, '_, '_>>
    for RemoveCreatorVerificationKeys<'me>
{
    fn from(accounts: &RemoveCreatorVerificationAccounts<'me, '_, '_>) -> Self {
        Self {
            metadata: accounts.metadata.key,
            creator: accounts.creator.key,
        }
    }
}
impl From<&RemoveCreatorVerificationKeys<'_>> for [AccountMeta; 2] {
    fn from(keys: &RemoveCreatorVerificationKeys<'_>) -> Self {
        [
            AccountMeta::new(*keys.metadata, false),
            AccountMeta::new_readonly(*keys.creator, true),
        ]
    }
}
impl<'a> From<&RemoveCreatorVerificationAccounts<'_, 'a, 'a>> for [AccountInfo<'a>; 2] {
    fn from(accounts: &RemoveCreatorVerificationAccounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.creator.clone()]
    }
}
#[derive(BorshSerialize, Clone, Debug)]
pub struct RemoveCreatorVerificationIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationIxData<'me>(pub &'me RemoveCreatorVerificationIxArgs);
impl<'me> From<&'me RemoveCreatorVerificationIxArgs> for RemoveCreatorVerificationIxData<'me> {
    fn from(args: &'me RemoveCreatorVerificationIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveCreatorVerificationIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write(&[28u8])?;
        self.0.serialize(writer)
    }
}
pub fn remove_creator_verification_ix<
    'k,
    'd,
    K: Into<RemoveCreatorVerificationKeys<'k>>,
    D: Into<RemoveCreatorVerificationIxData<'d>>,
>(
    accounts: K,
    args: D,
) -> std::io::Result<Instruction> {
    let keys: RemoveCreatorVerificationKeys = accounts.into();
    let metas: [AccountMeta; 2] = (&keys).into();
    let data: RemoveCreatorVerificationIxData = args.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_creator_verification_invoke<'a, 'd, D: Into<RemoveCreatorVerificationIxData<'d>>>(
    accounts: &RemoveCreatorVerificationAccounts<'_, 'a, 'a>,
    args: D,
) -> ProgramResult {
    let ix = remove_creator_verification_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_creator_verification_invoke_signed<
    'a,
    'd,
    D: Into<RemoveCreatorVerificationIxData<'d>>,
>(
    accounts: &RemoveCreatorVerificationAccounts<'_, 'a, 'a>,
    args: D,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_creator_verification_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; 2] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
