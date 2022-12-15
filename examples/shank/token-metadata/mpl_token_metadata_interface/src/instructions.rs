use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey,
};
pub const CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN: usize = 7usize;
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
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a0>,
    ///Mint of token asset
    pub mint: &'me AccountInfo<'a1>,
    ///Mint authority
    pub mint_authority: &'me AccountInfo<'a2>,
    ///payer
    pub payer: &'me AccountInfo<'a3>,
    ///update authority info
    pub update_authority: &'me AccountInfo<'a4>,
    ///System program
    pub system_program: &'me AccountInfo<'a5>,
    ///Rent info
    pub rent: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountKeys {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Mint of token asset
    pub mint: Pubkey,
    ///Mint authority
    pub mint_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///update authority info
    pub update_authority: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&CreateMetadataAccountAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMetadataAccountKeys
{
    fn from(accounts: &CreateMetadataAccountAccounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            payer: *accounts.payer.key,
            update_authority: *accounts.update_authority.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&CreateMetadataAccountKeys> for [AccountMeta; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreateMetadataAccountKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.mint_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.update_authority, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateMetadataAccountIxArgs {
    create_metadata_account_args: CreateMetadataAccountArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountIxData<'me>(pub &'me CreateMetadataAccountIxArgs);
pub const CREATE_METADATA_ACCOUNT_IX_DISCM: u8 = 0u8;
impl<'me> From<&'me CreateMetadataAccountIxArgs> for CreateMetadataAccountIxData<'me> {
    fn from(args: &'me CreateMetadataAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMetadataAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_METADATA_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn create_metadata_account_ix<
    K: Into<CreateMetadataAccountKeys>,
    A: Into<CreateMetadataAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateMetadataAccountKeys = accounts.into();
    let metas: [AccountMeta; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateMetadataAccountIxArgs = args.into();
    let data: CreateMetadataAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_account_invoke<'a, A: Into<CreateMetadataAccountIxArgs>>(
    accounts: &CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = create_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_metadata_account_invoke_signed<'a, A: Into<CreateMetadataAccountIxArgs>>(
    accounts: &CreateMetadataAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountAccounts<'me, 'a0: 'me, 'a1: 'me> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
    ///Update authority key
    pub update_authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountKeys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Update authority key
    pub update_authority: Pubkey,
}
impl<'me> From<&UpdateMetadataAccountAccounts<'me, '_, '_>> for UpdateMetadataAccountKeys {
    fn from(accounts: &UpdateMetadataAccountAccounts<'me, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            update_authority: *accounts.update_authority.key,
        }
    }
}
impl From<&UpdateMetadataAccountKeys> for [AccountMeta; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &UpdateMetadataAccountKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.update_authority, true),
        ]
    }
}
impl<'a> From<&UpdateMetadataAccountAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateMetadataAccountAccounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.update_authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateMetadataAccountIxArgs {
    update_metadata_account_args: UpdateMetadataAccountArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountIxData<'me>(pub &'me UpdateMetadataAccountIxArgs);
pub const UPDATE_METADATA_ACCOUNT_IX_DISCM: u8 = 1u8;
impl<'me> From<&'me UpdateMetadataAccountIxArgs> for UpdateMetadataAccountIxData<'me> {
    fn from(args: &'me UpdateMetadataAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateMetadataAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_METADATA_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn update_metadata_account_ix<
    K: Into<UpdateMetadataAccountKeys>,
    A: Into<UpdateMetadataAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateMetadataAccountKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateMetadataAccountIxArgs = args.into();
    let data: UpdateMetadataAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_metadata_account_invoke<'a, A: Into<UpdateMetadataAccountIxArgs>>(
    accounts: &UpdateMetadataAccountAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_metadata_account_invoke_signed<'a, A: Into<UpdateMetadataAccountIxArgs>>(
    accounts: &UpdateMetadataAccountAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN: usize = 13usize;
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
    ///Unallocated edition V1 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: &'me AccountInfo<'a0>,
    ///Metadata mint
    pub mint: &'me AccountInfo<'a1>,
    ///Printing mint - A mint you control that can mint tokens that can be exchanged for limited editions of your master edition via the MintNewEditionFromMasterEditionViaToken endpoint
    pub printing_mint: &'me AccountInfo<'a2>,
    ///One time authorization printing mint - A mint you control that prints tokens that gives the bearer permission to mint any number of tokens from the printing mint one time via an endpoint with the token-metadata program for your metadata. Also burns the token.
    pub one_time_printing_authorization_mint: &'me AccountInfo<'a3>,
    ///Current Update authority key
    pub update_authority: &'me AccountInfo<'a4>,
    ///Printing mint authority - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.
    pub printing_mint_authority: &'me AccountInfo<'a5>,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: &'me AccountInfo<'a6>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'a7>,
    ///payer
    pub payer: &'me AccountInfo<'a8>,
    ///Token program
    pub token_program: &'me AccountInfo<'a9>,
    ///System program
    pub system_program: &'me AccountInfo<'a10>,
    ///Rent info
    pub rent: &'me AccountInfo<'a11>,
    ///One time authorization printing mint authority - must be provided if using max supply. THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.
    pub one_time_printing_authorization_mint_authority: &'me AccountInfo<'a12>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionKeys {
    ///Unallocated edition V1 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: Pubkey,
    ///Metadata mint
    pub mint: Pubkey,
    ///Printing mint - A mint you control that can mint tokens that can be exchanged for limited editions of your master edition via the MintNewEditionFromMasterEditionViaToken endpoint
    pub printing_mint: Pubkey,
    ///One time authorization printing mint - A mint you control that prints tokens that gives the bearer permission to mint any number of tokens from the printing mint one time via an endpoint with the token-metadata program for your metadata. Also burns the token.
    pub one_time_printing_authorization_mint: Pubkey,
    ///Current Update authority key
    pub update_authority: Pubkey,
    ///Printing mint authority - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.
    pub printing_mint_authority: Pubkey,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: Pubkey,
    ///Metadata account
    pub metadata: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
    ///One time authorization printing mint authority - must be provided if using max supply. THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.
    pub one_time_printing_authorization_mint_authority: Pubkey,
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
    > for DeprecatedCreateMasterEditionKeys
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
            edition: *accounts.edition.key,
            mint: *accounts.mint.key,
            printing_mint: *accounts.printing_mint.key,
            one_time_printing_authorization_mint: *accounts
                .one_time_printing_authorization_mint
                .key,
            update_authority: *accounts.update_authority.key,
            printing_mint_authority: *accounts.printing_mint_authority.key,
            mint_authority: *accounts.mint_authority.key,
            metadata: *accounts.metadata.key,
            payer: *accounts.payer.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            one_time_printing_authorization_mint_authority: *accounts
                .one_time_printing_authorization_mint_authority
                .key,
        }
    }
}
impl From<&DeprecatedCreateMasterEditionKeys>
    for [AccountMeta; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeprecatedCreateMasterEditionKeys) -> Self {
        [
            AccountMeta::new(keys.edition, false),
            AccountMeta::new(keys.mint, false),
            AccountMeta::new(keys.printing_mint, false),
            AccountMeta::new(keys.one_time_printing_authorization_mint, false),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.printing_mint_authority, true),
            AccountMeta::new_readonly(keys.mint_authority, true),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new_readonly(keys.one_time_printing_authorization_mint_authority, true),
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
    > for [AccountInfo<'a>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionIxArgs {
    create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionIxData<'me>(pub &'me DeprecatedCreateMasterEditionIxArgs);
pub const DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM: u8 = 2u8;
impl<'me> From<&'me DeprecatedCreateMasterEditionIxArgs>
    for DeprecatedCreateMasterEditionIxData<'me>
{
    fn from(args: &'me DeprecatedCreateMasterEditionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedCreateMasterEditionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_create_master_edition_ix<
    K: Into<DeprecatedCreateMasterEditionKeys>,
    A: Into<DeprecatedCreateMasterEditionIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedCreateMasterEditionKeys = accounts.into();
    let metas: [AccountMeta; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeprecatedCreateMasterEditionIxArgs = args.into();
    let data: DeprecatedCreateMasterEditionIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_create_master_edition_invoke<'a, A: Into<DeprecatedCreateMasterEditionIxArgs>>(
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
    args: A,
) -> ProgramResult {
    let ix = deprecated_create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_create_master_edition_invoke_signed<
    'a,
    A: Into<DeprecatedCreateMasterEditionIxArgs>,
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
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN:
    usize = 16usize;
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
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a0>,
    ///New Edition V1 (pda of ['metadata', program id, mint id, 'edition'])
    pub edition: &'me AccountInfo<'a1>,
    ///Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a2>,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint: &'me AccountInfo<'a3>,
    ///Mint authority of new mint
    pub mint_authority: &'me AccountInfo<'a4>,
    ///Printing Mint of master record edition
    pub printing_mint: &'me AccountInfo<'a5>,
    ///Token account containing Printing mint token to be transferred
    pub master_token_account: &'me AccountInfo<'a6>,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master mint id, edition_number])
    pub edition_marker: &'me AccountInfo<'a7>,
    ///Burn authority for this token
    pub burn_authority: &'me AccountInfo<'a8>,
    ///payer
    pub payer: &'me AccountInfo<'a9>,
    ///update authority info for new metadata account
    pub master_update_authority: &'me AccountInfo<'a10>,
    ///Master record metadata account
    pub master_metadata: &'me AccountInfo<'a11>,
    ///Token program
    pub token_program: &'me AccountInfo<'a12>,
    ///System program
    pub system_program: &'me AccountInfo<'a13>,
    ///Rent info
    pub rent: &'me AccountInfo<'a14>,
    ///Reservation List - If present, and you are on this list, you can get an edition number given by your position on the list.
    pub reservation_list: &'me AccountInfo<'a15>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys {
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///New Edition V1 (pda of ['metadata', program id, mint id, 'edition'])
    pub edition: Pubkey,
    ///Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: Pubkey,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint: Pubkey,
    ///Mint authority of new mint
    pub mint_authority: Pubkey,
    ///Printing Mint of master record edition
    pub printing_mint: Pubkey,
    ///Token account containing Printing mint token to be transferred
    pub master_token_account: Pubkey,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master mint id, edition_number])
    pub edition_marker: Pubkey,
    ///Burn authority for this token
    pub burn_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///update authority info for new metadata account
    pub master_update_authority: Pubkey,
    ///Master record metadata account
    pub master_metadata: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
    ///Reservation List - If present, and you are on this list, you can get an edition number given by your position on the list.
    pub reservation_list: Pubkey,
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
    > for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys
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
            metadata: *accounts.metadata.key,
            edition: *accounts.edition.key,
            master_edition: *accounts.master_edition.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            printing_mint: *accounts.printing_mint.key,
            master_token_account: *accounts.master_token_account.key,
            edition_marker: *accounts.edition_marker.key,
            burn_authority: *accounts.burn_authority.key,
            payer: *accounts.payer.key,
            master_update_authority: *accounts.master_update_authority.key,
            master_metadata: *accounts.master_metadata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            reservation_list: *accounts.reservation_list.key,
        }
    }
}
impl From<&DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys>
    for [AccountMeta;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new(keys.edition, false),
            AccountMeta::new(keys.master_edition, false),
            AccountMeta::new(keys.mint, false),
            AccountMeta::new_readonly(keys.mint_authority, true),
            AccountMeta::new(keys.printing_mint, false),
            AccountMeta::new(keys.master_token_account, false),
            AccountMeta::new(keys.edition_marker, false),
            AccountMeta::new_readonly(keys.burn_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.master_update_authority, false),
            AccountMeta::new_readonly(keys.master_metadata, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new(keys.reservation_list, false),
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
    >
    for [AccountInfo<'a>;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'me>(
    pub &'me DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs,
);
pub const DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM: u8 = 3u8;
impl<'me> From<&'me DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'me>
{
    fn from(args: &'me DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[
            DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM,
        ])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_ix<
    K: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys>,
    A: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys = accounts.into();
    let metas: [AccountMeta;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs = args.into();
    let data: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_invoke<
    'a,
    A: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>,
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
    args: A,
) -> ProgramResult {
    let ix = deprecated_mint_new_edition_from_master_edition_via_printing_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_invoke_signed<
    'a,
    A: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>,
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
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_new_edition_from_master_edition_via_printing_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a0>,
    ///Owner on the token account
    pub owner: &'me AccountInfo<'a1>,
    ///Account containing tokens from the metadata's mint
    pub token: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenKeys {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Owner on the token account
    pub owner: Pubkey,
    ///Account containing tokens from the metadata's mint
    pub token: Pubkey,
}
impl<'me> From<&UpdatePrimarySaleHappenedViaTokenAccounts<'me, '_, '_, '_>>
    for UpdatePrimarySaleHappenedViaTokenKeys
{
    fn from(accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            owner: *accounts.owner.key,
            token: *accounts.token.key,
        }
    }
}
impl From<&UpdatePrimarySaleHappenedViaTokenKeys>
    for [AccountMeta; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdatePrimarySaleHappenedViaTokenKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.owner, true),
            AccountMeta::new_readonly(keys.token, false),
        ]
    }
}
impl<'a> From<&UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.owner.clone(),
            accounts.token.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenIxData<'me>(
    pub &'me UpdatePrimarySaleHappenedViaTokenIxArgs,
);
pub const UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM: u8 = 4u8;
impl<'me> From<&'me UpdatePrimarySaleHappenedViaTokenIxArgs>
    for UpdatePrimarySaleHappenedViaTokenIxData<'me>
{
    fn from(args: &'me UpdatePrimarySaleHappenedViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdatePrimarySaleHappenedViaTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn update_primary_sale_happened_via_token_ix<
    K: Into<UpdatePrimarySaleHappenedViaTokenKeys>,
    A: Into<UpdatePrimarySaleHappenedViaTokenIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdatePrimarySaleHappenedViaTokenKeys = accounts.into();
    let metas: [AccountMeta; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: UpdatePrimarySaleHappenedViaTokenIxArgs = args.into();
    let data: UpdatePrimarySaleHappenedViaTokenIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_primary_sale_happened_via_token_invoke<
    'a,
    A: Into<UpdatePrimarySaleHappenedViaTokenIxArgs>,
>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_primary_sale_happened_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_primary_sale_happened_via_token_invoke_signed<
    'a,
    A: Into<UpdatePrimarySaleHappenedViaTokenIxArgs>,
>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_primary_sale_happened_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a0>,
    ///PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]
    pub reservation_list: &'me AccountInfo<'a1>,
    ///The resource you tied the reservation list too
    pub resource: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListKeys {
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: Pubkey,
    ///PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]
    pub reservation_list: Pubkey,
    ///The resource you tied the reservation list too
    pub resource: Pubkey,
}
impl<'me> From<&DeprecatedSetReservationListAccounts<'me, '_, '_, '_>>
    for DeprecatedSetReservationListKeys
{
    fn from(accounts: &DeprecatedSetReservationListAccounts<'me, '_, '_, '_>) -> Self {
        Self {
            master_edition: *accounts.master_edition.key,
            reservation_list: *accounts.reservation_list.key,
            resource: *accounts.resource.key,
        }
    }
}
impl From<&DeprecatedSetReservationListKeys>
    for [AccountMeta; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeprecatedSetReservationListKeys) -> Self {
        [
            AccountMeta::new(keys.master_edition, false),
            AccountMeta::new(keys.reservation_list, false),
            AccountMeta::new_readonly(keys.resource, true),
        ]
    }
}
impl<'a> From<&DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.master_edition.clone(),
            accounts.reservation_list.clone(),
            accounts.resource.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeprecatedSetReservationListIxArgs {
    set_reservation_list_args: SetReservationListArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListIxData<'me>(pub &'me DeprecatedSetReservationListIxArgs);
pub const DEPRECATED_SET_RESERVATION_LIST_IX_DISCM: u8 = 5u8;
impl<'me> From<&'me DeprecatedSetReservationListIxArgs>
    for DeprecatedSetReservationListIxData<'me>
{
    fn from(args: &'me DeprecatedSetReservationListIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedSetReservationListIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_SET_RESERVATION_LIST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_set_reservation_list_ix<
    K: Into<DeprecatedSetReservationListKeys>,
    A: Into<DeprecatedSetReservationListIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedSetReservationListKeys = accounts.into();
    let metas: [AccountMeta; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeprecatedSetReservationListIxArgs = args.into();
    let data: DeprecatedSetReservationListIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_set_reservation_list_invoke<'a, A: Into<DeprecatedSetReservationListIxArgs>>(
    accounts: &DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_set_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_set_reservation_list_invoke_signed<
    'a,
    A: Into<DeprecatedSetReservationListIxArgs>,
>(
    accounts: &DeprecatedSetReservationListAccounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_set_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN: usize = 8usize;
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
    ///PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]
    pub reservation_list: &'me AccountInfo<'a0>,
    ///Payer
    pub payer: &'me AccountInfo<'a1>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'a2>,
    /// Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a3>,
    ///A resource you wish to tie the reservation list to. This is so your later visitors who come to redeem can derive your reservation list PDA with something they can easily get at. You choose what this should be.
    pub resource: &'me AccountInfo<'a4>,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a5>,
    ///System program
    pub system_program: &'me AccountInfo<'a6>,
    ///Rent info
    pub rent: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateReservationListKeys {
    ///PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]
    pub reservation_list: Pubkey,
    ///Payer
    pub payer: Pubkey,
    ///Update authority
    pub update_authority: Pubkey,
    /// Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: Pubkey,
    ///A resource you wish to tie the reservation list to. This is so your later visitors who come to redeem can derive your reservation list PDA with something they can easily get at. You choose what this should be.
    pub resource: Pubkey,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&DeprecatedCreateReservationListAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for DeprecatedCreateReservationListKeys
{
    fn from(
        accounts: &DeprecatedCreateReservationListAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            reservation_list: *accounts.reservation_list.key,
            payer: *accounts.payer.key,
            update_authority: *accounts.update_authority.key,
            master_edition: *accounts.master_edition.key,
            resource: *accounts.resource.key,
            metadata: *accounts.metadata.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&DeprecatedCreateReservationListKeys>
    for [AccountMeta; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeprecatedCreateReservationListKeys) -> Self {
        [
            AccountMeta::new(keys.reservation_list, false),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.master_edition, false),
            AccountMeta::new_readonly(keys.resource, false),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeprecatedCreateReservationListIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateReservationListIxData<'me>(
    pub &'me DeprecatedCreateReservationListIxArgs,
);
pub const DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM: u8 = 6u8;
impl<'me> From<&'me DeprecatedCreateReservationListIxArgs>
    for DeprecatedCreateReservationListIxData<'me>
{
    fn from(args: &'me DeprecatedCreateReservationListIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedCreateReservationListIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_create_reservation_list_ix<
    K: Into<DeprecatedCreateReservationListKeys>,
    A: Into<DeprecatedCreateReservationListIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedCreateReservationListKeys = accounts.into();
    let metas: [AccountMeta; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeprecatedCreateReservationListIxArgs = args.into();
    let data: DeprecatedCreateReservationListIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_create_reservation_list_invoke<
    'a,
    A: Into<DeprecatedCreateReservationListIxArgs>,
>(
    accounts: &DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_create_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_create_reservation_list_invoke_signed<
    'a,
    A: Into<DeprecatedCreateReservationListIxArgs>,
>(
    accounts: &DeprecatedCreateReservationListAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_create_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SIGN_METADATA_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataAccounts<'me, 'a0: 'me, 'a1: 'me> {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a0>,
    ///Creator
    pub creator: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataKeys {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Creator
    pub creator: Pubkey,
}
impl<'me> From<&SignMetadataAccounts<'me, '_, '_>> for SignMetadataKeys {
    fn from(accounts: &SignMetadataAccounts<'me, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            creator: *accounts.creator.key,
        }
    }
}
impl From<&SignMetadataKeys> for [AccountMeta; SIGN_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: &SignMetadataKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.creator, true),
        ]
    }
}
impl<'a> From<&SignMetadataAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; SIGN_METADATA_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SignMetadataAccounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.creator.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SignMetadataIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataIxData<'me>(pub &'me SignMetadataIxArgs);
pub const SIGN_METADATA_IX_DISCM: u8 = 7u8;
impl<'me> From<&'me SignMetadataIxArgs> for SignMetadataIxData<'me> {
    fn from(args: &'me SignMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SignMetadataIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SIGN_METADATA_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn sign_metadata_ix<K: Into<SignMetadataKeys>, A: Into<SignMetadataIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SignMetadataKeys = accounts.into();
    let metas: [AccountMeta; SIGN_METADATA_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SignMetadataIxArgs = args.into();
    let data: SignMetadataIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sign_metadata_invoke<'a, A: Into<SignMetadataIxArgs>>(
    accounts: &SignMetadataAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = sign_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SIGN_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sign_metadata_invoke_signed<'a, A: Into<SignMetadataIxArgs>>(
    accounts: &SignMetadataAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sign_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SIGN_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN: usize = 9usize;
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
    ///Destination account
    pub destination: &'me AccountInfo<'a0>,
    ///Token account containing one time authorization token
    pub token: &'me AccountInfo<'a1>,
    ///One time authorization mint
    pub one_time_printing_authorization_mint: &'me AccountInfo<'a2>,
    ///Printing mint
    pub printing_mint: &'me AccountInfo<'a3>,
    ///Burn authority
    pub burn_authority: &'me AccountInfo<'a4>,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a5>,
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a6>,
    ///Token program
    pub token_program: &'me AccountInfo<'a7>,
    ///Rent
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenKeys {
    ///Destination account
    pub destination: Pubkey,
    ///Token account containing one time authorization token
    pub token: Pubkey,
    ///One time authorization mint
    pub one_time_printing_authorization_mint: Pubkey,
    ///Printing mint
    pub printing_mint: Pubkey,
    ///Burn authority
    pub burn_authority: Pubkey,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///Rent
    pub rent: Pubkey,
}
impl<'me>
    From<&DeprecatedMintPrintingTokensViaTokenAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for DeprecatedMintPrintingTokensViaTokenKeys
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
            destination: *accounts.destination.key,
            token: *accounts.token.key,
            one_time_printing_authorization_mint: *accounts
                .one_time_printing_authorization_mint
                .key,
            printing_mint: *accounts.printing_mint.key,
            burn_authority: *accounts.burn_authority.key,
            metadata: *accounts.metadata.key,
            master_edition: *accounts.master_edition.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&DeprecatedMintPrintingTokensViaTokenKeys>
    for [AccountMeta; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeprecatedMintPrintingTokensViaTokenKeys) -> Self {
        [
            AccountMeta::new(keys.destination, false),
            AccountMeta::new(keys.token, false),
            AccountMeta::new(keys.one_time_printing_authorization_mint, false),
            AccountMeta::new(keys.printing_mint, false),
            AccountMeta::new_readonly(keys.burn_authority, true),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.master_edition, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenIxArgs {
    mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenIxData<'me>(
    pub &'me DeprecatedMintPrintingTokensViaTokenIxArgs,
);
pub const DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM: u8 = 8u8;
impl<'me> From<&'me DeprecatedMintPrintingTokensViaTokenIxArgs>
    for DeprecatedMintPrintingTokensViaTokenIxData<'me>
{
    fn from(args: &'me DeprecatedMintPrintingTokensViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedMintPrintingTokensViaTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_mint_printing_tokens_via_token_ix<
    K: Into<DeprecatedMintPrintingTokensViaTokenKeys>,
    A: Into<DeprecatedMintPrintingTokensViaTokenIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedMintPrintingTokensViaTokenKeys = accounts.into();
    let metas: [AccountMeta; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: DeprecatedMintPrintingTokensViaTokenIxArgs = args.into();
    let data: DeprecatedMintPrintingTokensViaTokenIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_printing_tokens_via_token_invoke<
    'a,
    A: Into<DeprecatedMintPrintingTokensViaTokenIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_printing_tokens_via_token_invoke_signed<
    'a,
    A: Into<DeprecatedMintPrintingTokensViaTokenIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN: usize = 7usize;
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
    ///Destination account
    pub destination: &'me AccountInfo<'a0>,
    ///Printing mint
    pub printing_mint: &'me AccountInfo<'a1>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'a2>,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a3>,
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a4>,
    ///Token program
    pub token_program: &'me AccountInfo<'a5>,
    ///Rent
    pub rent: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensKeys {
    ///Destination account
    pub destination: Pubkey,
    ///Printing mint
    pub printing_mint: Pubkey,
    ///Update authority
    pub update_authority: Pubkey,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///Rent
    pub rent: Pubkey,
}
impl<'me> From<&DeprecatedMintPrintingTokensAccounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for DeprecatedMintPrintingTokensKeys
{
    fn from(
        accounts: &DeprecatedMintPrintingTokensAccounts<'me, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            destination: *accounts.destination.key,
            printing_mint: *accounts.printing_mint.key,
            update_authority: *accounts.update_authority.key,
            metadata: *accounts.metadata.key,
            master_edition: *accounts.master_edition.key,
            token_program: *accounts.token_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&DeprecatedMintPrintingTokensKeys>
    for [AccountMeta; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN]
{
    fn from(keys: &DeprecatedMintPrintingTokensKeys) -> Self {
        [
            AccountMeta::new(keys.destination, false),
            AccountMeta::new(keys.printing_mint, false),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.master_edition, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensIxArgs {
    mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensIxData<'me>(pub &'me DeprecatedMintPrintingTokensIxArgs);
pub const DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM: u8 = 9u8;
impl<'me> From<&'me DeprecatedMintPrintingTokensIxArgs>
    for DeprecatedMintPrintingTokensIxData<'me>
{
    fn from(args: &'me DeprecatedMintPrintingTokensIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for DeprecatedMintPrintingTokensIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn deprecated_mint_printing_tokens_ix<
    K: Into<DeprecatedMintPrintingTokensKeys>,
    A: Into<DeprecatedMintPrintingTokensIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: DeprecatedMintPrintingTokensKeys = accounts.into();
    let metas: [AccountMeta; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: DeprecatedMintPrintingTokensIxArgs = args.into();
    let data: DeprecatedMintPrintingTokensIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_printing_tokens_invoke<'a, A: Into<DeprecatedMintPrintingTokensIxArgs>>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_printing_tokens_invoke_signed<
    'a,
    A: Into<DeprecatedMintPrintingTokensIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN: usize = 9usize;
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
    ///Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: &'me AccountInfo<'a0>,
    ///Metadata mint
    pub mint: &'me AccountInfo<'a1>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'a2>,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: &'me AccountInfo<'a3>,
    ///payer
    pub payer: &'me AccountInfo<'a4>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'a5>,
    ///Token program
    pub token_program: &'me AccountInfo<'a6>,
    ///System program
    pub system_program: &'me AccountInfo<'a7>,
    ///Rent info
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionKeys {
    ///Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: Pubkey,
    ///Metadata mint
    pub mint: Pubkey,
    ///Update authority
    pub update_authority: Pubkey,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///Metadata account
    pub metadata: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&CreateMasterEditionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMasterEditionKeys
{
    fn from(
        accounts: &CreateMasterEditionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            edition: *accounts.edition.key,
            mint: *accounts.mint.key,
            update_authority: *accounts.update_authority.key,
            mint_authority: *accounts.mint_authority.key,
            payer: *accounts.payer.key,
            metadata: *accounts.metadata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&CreateMasterEditionKeys> for [AccountMeta; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreateMasterEditionKeys) -> Self {
        [
            AccountMeta::new(keys.edition, false),
            AccountMeta::new(keys.mint, false),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.mint_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateMasterEditionIxArgs {
    create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionIxData<'me>(pub &'me CreateMasterEditionIxArgs);
pub const CREATE_MASTER_EDITION_IX_DISCM: u8 = 10u8;
impl<'me> From<&'me CreateMasterEditionIxArgs> for CreateMasterEditionIxData<'me> {
    fn from(args: &'me CreateMasterEditionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMasterEditionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_MASTER_EDITION_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn create_master_edition_ix<
    K: Into<CreateMasterEditionKeys>,
    A: Into<CreateMasterEditionIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateMasterEditionKeys = accounts.into();
    let metas: [AccountMeta; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateMasterEditionIxArgs = args.into();
    let data: CreateMasterEditionIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_master_edition_invoke<'a, A: Into<CreateMasterEditionIxArgs>>(
    accounts: &CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_master_edition_invoke_signed<'a, A: Into<CreateMasterEditionIxArgs>>(
    accounts: &CreateMasterEditionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN: usize = 14usize;
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
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata: &'me AccountInfo<'a0>,
    ///New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: &'me AccountInfo<'a1>,
    ///Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a2>,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_mint: &'me AccountInfo<'a3>,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_mark_pda: &'me AccountInfo<'a4>,
    ///Mint authority of new mint
    pub new_mint_authority: &'me AccountInfo<'a5>,
    ///payer
    pub payer: &'me AccountInfo<'a6>,
    ///owner of token account containing master token (#8)
    pub token_account_owner: &'me AccountInfo<'a7>,
    ///token account containing token from master metadata mint
    pub token_account: &'me AccountInfo<'a8>,
    ///Update authority info for new metadata
    pub new_metadata_update_authority: &'me AccountInfo<'a9>,
    ///Master record metadata account
    pub metadata: &'me AccountInfo<'a10>,
    ///Token program
    pub token_program: &'me AccountInfo<'a11>,
    ///System program
    pub system_program: &'me AccountInfo<'a12>,
    ///Rent info
    pub rent: &'me AccountInfo<'a13>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenKeys {
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata: Pubkey,
    ///New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: Pubkey,
    ///Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: Pubkey,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_mint: Pubkey,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_mark_pda: Pubkey,
    ///Mint authority of new mint
    pub new_mint_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///owner of token account containing master token (#8)
    pub token_account_owner: Pubkey,
    ///token account containing token from master metadata mint
    pub token_account: Pubkey,
    ///Update authority info for new metadata
    pub new_metadata_update_authority: Pubkey,
    ///Master record metadata account
    pub metadata: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
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
    > for MintNewEditionFromMasterEditionViaTokenKeys
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
            new_metadata: *accounts.new_metadata.key,
            new_edition: *accounts.new_edition.key,
            master_edition: *accounts.master_edition.key,
            new_mint: *accounts.new_mint.key,
            edition_mark_pda: *accounts.edition_mark_pda.key,
            new_mint_authority: *accounts.new_mint_authority.key,
            payer: *accounts.payer.key,
            token_account_owner: *accounts.token_account_owner.key,
            token_account: *accounts.token_account.key,
            new_metadata_update_authority: *accounts.new_metadata_update_authority.key,
            metadata: *accounts.metadata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&MintNewEditionFromMasterEditionViaTokenKeys>
    for [AccountMeta; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(keys: &MintNewEditionFromMasterEditionViaTokenKeys) -> Self {
        [
            AccountMeta::new(keys.new_metadata, false),
            AccountMeta::new(keys.new_edition, false),
            AccountMeta::new(keys.master_edition, false),
            AccountMeta::new(keys.new_mint, false),
            AccountMeta::new(keys.edition_mark_pda, false),
            AccountMeta::new_readonly(keys.new_mint_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.token_account_owner, true),
            AccountMeta::new_readonly(keys.token_account, false),
            AccountMeta::new_readonly(keys.new_metadata_update_authority, false),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
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
    > for [AccountInfo<'a>; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenIxArgs {
    mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenIxData<'me>(
    pub &'me MintNewEditionFromMasterEditionViaTokenIxArgs,
);
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM: u8 = 11u8;
impl<'me> From<&'me MintNewEditionFromMasterEditionViaTokenIxArgs>
    for MintNewEditionFromMasterEditionViaTokenIxData<'me>
{
    fn from(args: &'me MintNewEditionFromMasterEditionViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MintNewEditionFromMasterEditionViaTokenIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn mint_new_edition_from_master_edition_via_token_ix<
    K: Into<MintNewEditionFromMasterEditionViaTokenKeys>,
    A: Into<MintNewEditionFromMasterEditionViaTokenIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MintNewEditionFromMasterEditionViaTokenKeys = accounts.into();
    let metas: [AccountMeta; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: MintNewEditionFromMasterEditionViaTokenIxArgs = args.into();
    let data: MintNewEditionFromMasterEditionViaTokenIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_new_edition_from_master_edition_via_token_invoke<
    'a,
    A: Into<MintNewEditionFromMasterEditionViaTokenIxArgs>,
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
    args: A,
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_new_edition_from_master_edition_via_token_invoke_signed<
    'a,
    A: Into<MintNewEditionFromMasterEditionViaTokenIxArgs>,
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
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN: usize = 3usize;
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2Accounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me> {
    ///Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'a0>,
    ///One time authorization mint
    pub one_time_auth: &'me AccountInfo<'a1>,
    ///Printing mint
    pub printing_mint: &'me AccountInfo<'a2>,
}
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2Keys {
    ///Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: Pubkey,
    ///One time authorization mint
    pub one_time_auth: Pubkey,
    ///Printing mint
    pub printing_mint: Pubkey,
}
impl<'me> From<&ConvertMasterEditionV1ToV2Accounts<'me, '_, '_, '_>>
    for ConvertMasterEditionV1ToV2Keys
{
    fn from(accounts: &ConvertMasterEditionV1ToV2Accounts<'me, '_, '_, '_>) -> Self {
        Self {
            master_edition: *accounts.master_edition.key,
            one_time_auth: *accounts.one_time_auth.key,
            printing_mint: *accounts.printing_mint.key,
        }
    }
}
impl From<&ConvertMasterEditionV1ToV2Keys>
    for [AccountMeta; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN]
{
    fn from(keys: &ConvertMasterEditionV1ToV2Keys) -> Self {
        [
            AccountMeta::new(keys.master_edition, false),
            AccountMeta::new(keys.one_time_auth, false),
            AccountMeta::new(keys.printing_mint, false),
        ]
    }
}
impl<'a> From<&ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>) -> Self {
        [
            accounts.master_edition.clone(),
            accounts.one_time_auth.clone(),
            accounts.printing_mint.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2IxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2IxData<'me>(pub &'me ConvertMasterEditionV1ToV2IxArgs);
pub const CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM: u8 = 12u8;
impl<'me> From<&'me ConvertMasterEditionV1ToV2IxArgs> for ConvertMasterEditionV1ToV2IxData<'me> {
    fn from(args: &'me ConvertMasterEditionV1ToV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ConvertMasterEditionV1ToV2IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn convert_master_edition_v1_to_v2_ix<
    K: Into<ConvertMasterEditionV1ToV2Keys>,
    A: Into<ConvertMasterEditionV1ToV2IxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ConvertMasterEditionV1ToV2Keys = accounts.into();
    let metas: [AccountMeta; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ConvertMasterEditionV1ToV2IxArgs = args.into();
    let data: ConvertMasterEditionV1ToV2IxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_master_edition_v1_to_v2_invoke<'a, A: Into<ConvertMasterEditionV1ToV2IxArgs>>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = convert_master_edition_v1_to_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn convert_master_edition_v1_to_v2_invoke_signed<
    'a,
    A: Into<ConvertMasterEditionV1ToV2IxArgs>,
>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = convert_master_edition_v1_to_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN: usize = 17usize;
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
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata: &'me AccountInfo<'a0>,
    ///New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: &'me AccountInfo<'a1>,
    ///Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition']
    pub master_edition: &'me AccountInfo<'a2>,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_mint: &'me AccountInfo<'a3>,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_mark_pda: &'me AccountInfo<'a4>,
    ///Mint authority of new mint
    pub new_mint_authority: &'me AccountInfo<'a5>,
    ///payer
    pub payer: &'me AccountInfo<'a6>,
    ///Vault authority
    pub vault_authority: &'me AccountInfo<'a7>,
    ///Safety deposit token store account
    pub safety_deposit_store: &'me AccountInfo<'a8>,
    ///Safety deposit box
    pub safety_deposit_box: &'me AccountInfo<'a9>,
    ///Vault
    pub vault: &'me AccountInfo<'a10>,
    ///Update authority info for new metadata
    pub new_metadata_update_authority: &'me AccountInfo<'a11>,
    ///Master record metadata account
    pub metadata: &'me AccountInfo<'a12>,
    ///Token program
    pub token_program: &'me AccountInfo<'a13>,
    ///Token vault program
    pub token_vault_program: &'me AccountInfo<'a14>,
    ///System program
    pub system_program: &'me AccountInfo<'a15>,
    ///Rent info
    pub rent: &'me AccountInfo<'a16>,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyKeys {
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata: Pubkey,
    ///New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: Pubkey,
    ///Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition']
    pub master_edition: Pubkey,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_mint: Pubkey,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_mark_pda: Pubkey,
    ///Mint authority of new mint
    pub new_mint_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///Vault authority
    pub vault_authority: Pubkey,
    ///Safety deposit token store account
    pub safety_deposit_store: Pubkey,
    ///Safety deposit box
    pub safety_deposit_box: Pubkey,
    ///Vault
    pub vault: Pubkey,
    ///Update authority info for new metadata
    pub new_metadata_update_authority: Pubkey,
    ///Master record metadata account
    pub metadata: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///Token vault program
    pub token_vault_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
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
    > for MintNewEditionFromMasterEditionViaVaultProxyKeys
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
            new_metadata: *accounts.new_metadata.key,
            new_edition: *accounts.new_edition.key,
            master_edition: *accounts.master_edition.key,
            new_mint: *accounts.new_mint.key,
            edition_mark_pda: *accounts.edition_mark_pda.key,
            new_mint_authority: *accounts.new_mint_authority.key,
            payer: *accounts.payer.key,
            vault_authority: *accounts.vault_authority.key,
            safety_deposit_store: *accounts.safety_deposit_store.key,
            safety_deposit_box: *accounts.safety_deposit_box.key,
            vault: *accounts.vault.key,
            new_metadata_update_authority: *accounts.new_metadata_update_authority.key,
            metadata: *accounts.metadata.key,
            token_program: *accounts.token_program.key,
            token_vault_program: *accounts.token_vault_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&MintNewEditionFromMasterEditionViaVaultProxyKeys>
    for [AccountMeta; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &MintNewEditionFromMasterEditionViaVaultProxyKeys) -> Self {
        [
            AccountMeta::new(keys.new_metadata, false),
            AccountMeta::new(keys.new_edition, false),
            AccountMeta::new(keys.master_edition, false),
            AccountMeta::new(keys.new_mint, false),
            AccountMeta::new(keys.edition_mark_pda, false),
            AccountMeta::new_readonly(keys.new_mint_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.vault_authority, true),
            AccountMeta::new_readonly(keys.safety_deposit_store, false),
            AccountMeta::new_readonly(keys.safety_deposit_box, false),
            AccountMeta::new_readonly(keys.vault, false),
            AccountMeta::new_readonly(keys.new_metadata_update_authority, false),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.token_vault_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
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
    > for [AccountInfo<'a>; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyIxArgs {
    mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyIxData<'me>(
    pub &'me MintNewEditionFromMasterEditionViaVaultProxyIxArgs,
);
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM: u8 = 13u8;
impl<'me> From<&'me MintNewEditionFromMasterEditionViaVaultProxyIxArgs>
    for MintNewEditionFromMasterEditionViaVaultProxyIxData<'me>
{
    fn from(args: &'me MintNewEditionFromMasterEditionViaVaultProxyIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for MintNewEditionFromMasterEditionViaVaultProxyIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_ix<
    K: Into<MintNewEditionFromMasterEditionViaVaultProxyKeys>,
    A: Into<MintNewEditionFromMasterEditionViaVaultProxyIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: MintNewEditionFromMasterEditionViaVaultProxyKeys = accounts.into();
    let metas: [AccountMeta; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN] =
        (&keys).into();
    let args_full: MintNewEditionFromMasterEditionViaVaultProxyIxArgs = args.into();
    let data: MintNewEditionFromMasterEditionViaVaultProxyIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_invoke<
    'a,
    A: Into<MintNewEditionFromMasterEditionViaVaultProxyIxArgs>,
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
    args: A,
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_vault_proxy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_invoke_signed<
    'a,
    A: Into<MintNewEditionFromMasterEditionViaVaultProxyIxArgs>,
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
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_vault_proxy_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const PUFF_METADATA_IX_ACCOUNTS_LEN: usize = 1usize;
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataAccounts<'me, 'a0: 'me> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
}
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataKeys {
    ///Metadata account
    pub metadata: Pubkey,
}
impl<'me> From<&PuffMetadataAccounts<'me, '_>> for PuffMetadataKeys {
    fn from(accounts: &PuffMetadataAccounts<'me, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
        }
    }
}
impl From<&PuffMetadataKeys> for [AccountMeta; PUFF_METADATA_IX_ACCOUNTS_LEN] {
    fn from(keys: &PuffMetadataKeys) -> Self {
        [AccountMeta::new(keys.metadata, false)]
    }
}
impl<'a> From<&PuffMetadataAccounts<'_, 'a>> for [AccountInfo<'a>; PUFF_METADATA_IX_ACCOUNTS_LEN] {
    fn from(accounts: &PuffMetadataAccounts<'_, 'a>) -> Self {
        [accounts.metadata.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct PuffMetadataIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataIxData<'me>(pub &'me PuffMetadataIxArgs);
pub const PUFF_METADATA_IX_DISCM: u8 = 14u8;
impl<'me> From<&'me PuffMetadataIxArgs> for PuffMetadataIxData<'me> {
    fn from(args: &'me PuffMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for PuffMetadataIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[PUFF_METADATA_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn puff_metadata_ix<K: Into<PuffMetadataKeys>, A: Into<PuffMetadataIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PuffMetadataKeys = accounts.into();
    let metas: [AccountMeta; PUFF_METADATA_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PuffMetadataIxArgs = args.into();
    let data: PuffMetadataIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn puff_metadata_invoke<'a, A: Into<PuffMetadataIxArgs>>(
    accounts: &PuffMetadataAccounts<'_, 'a>,
    args: A,
) -> ProgramResult {
    let ix = puff_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PUFF_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn puff_metadata_invoke_signed<'a, A: Into<PuffMetadataIxArgs>>(
    accounts: &PuffMetadataAccounts<'_, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = puff_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; PUFF_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2Accounts<'me, 'a0: 'me, 'a1: 'me> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
    ///Update authority key
    pub update_authority: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2Keys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Update authority key
    pub update_authority: Pubkey,
}
impl<'me> From<&UpdateMetadataAccountV2Accounts<'me, '_, '_>> for UpdateMetadataAccountV2Keys {
    fn from(accounts: &UpdateMetadataAccountV2Accounts<'me, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            update_authority: *accounts.update_authority.key,
        }
    }
}
impl From<&UpdateMetadataAccountV2Keys>
    for [AccountMeta; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]
{
    fn from(keys: &UpdateMetadataAccountV2Keys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.update_authority, true),
        ]
    }
}
impl<'a> From<&UpdateMetadataAccountV2Accounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateMetadataAccountV2Accounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.update_authority.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UpdateMetadataAccountV2IxArgs {
    update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2IxData<'me>(pub &'me UpdateMetadataAccountV2IxArgs);
pub const UPDATE_METADATA_ACCOUNT_V2_IX_DISCM: u8 = 15u8;
impl<'me> From<&'me UpdateMetadataAccountV2IxArgs> for UpdateMetadataAccountV2IxData<'me> {
    fn from(args: &'me UpdateMetadataAccountV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UpdateMetadataAccountV2IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_METADATA_ACCOUNT_V2_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn update_metadata_account_v2_ix<
    K: Into<UpdateMetadataAccountV2Keys>,
    A: Into<UpdateMetadataAccountV2IxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UpdateMetadataAccountV2Keys = accounts.into();
    let metas: [AccountMeta; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UpdateMetadataAccountV2IxArgs = args.into();
    let data: UpdateMetadataAccountV2IxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_metadata_account_v2_invoke<'a, A: Into<UpdateMetadataAccountV2IxArgs>>(
    accounts: &UpdateMetadataAccountV2Accounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = update_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_metadata_account_v2_invoke_signed<'a, A: Into<UpdateMetadataAccountV2IxArgs>>(
    accounts: &UpdateMetadataAccountV2Accounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN: usize = 7usize;
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
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a0>,
    ///Mint of token asset
    pub mint: &'me AccountInfo<'a1>,
    ///Mint authority
    pub mint_authority: &'me AccountInfo<'a2>,
    ///payer
    pub payer: &'me AccountInfo<'a3>,
    ///update authority info
    pub update_authority: &'me AccountInfo<'a4>,
    ///System program
    pub system_program: &'me AccountInfo<'a5>,
    ///Rent info
    pub rent: &'me AccountInfo<'a6>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountV2Keys {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Mint of token asset
    pub mint: Pubkey,
    ///Mint authority
    pub mint_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///update authority info
    pub update_authority: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&CreateMetadataAccountV2Accounts<'me, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMetadataAccountV2Keys
{
    fn from(accounts: &CreateMetadataAccountV2Accounts<'me, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
            mint_authority: *accounts.mint_authority.key,
            payer: *accounts.payer.key,
            update_authority: *accounts.update_authority.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&CreateMetadataAccountV2Keys>
    for [AccountMeta; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]
{
    fn from(keys: &CreateMetadataAccountV2Keys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.mint_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.update_authority, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateMetadataAccountV2IxArgs {
    create_metadata_account_args_v2: CreateMetadataAccountArgsV2,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountV2IxData<'me>(pub &'me CreateMetadataAccountV2IxArgs);
pub const CREATE_METADATA_ACCOUNT_V2_IX_DISCM: u8 = 16u8;
impl<'me> From<&'me CreateMetadataAccountV2IxArgs> for CreateMetadataAccountV2IxData<'me> {
    fn from(args: &'me CreateMetadataAccountV2IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMetadataAccountV2IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_METADATA_ACCOUNT_V2_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn create_metadata_account_v2_ix<
    K: Into<CreateMetadataAccountV2Keys>,
    A: Into<CreateMetadataAccountV2IxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateMetadataAccountV2Keys = accounts.into();
    let metas: [AccountMeta; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateMetadataAccountV2IxArgs = args.into();
    let data: CreateMetadataAccountV2IxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_account_v2_invoke<'a, A: Into<CreateMetadataAccountV2IxArgs>>(
    accounts: &CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = create_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_metadata_account_v2_invoke_signed<'a, A: Into<CreateMetadataAccountV2IxArgs>>(
    accounts: &CreateMetadataAccountV2Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN: usize = 9usize;
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
    ///Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: &'me AccountInfo<'a0>,
    ///Metadata mint
    pub mint: &'me AccountInfo<'a1>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'a2>,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: &'me AccountInfo<'a3>,
    ///payer
    pub payer: &'me AccountInfo<'a4>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'a5>,
    ///Token program
    pub token_program: &'me AccountInfo<'a6>,
    ///System program
    pub system_program: &'me AccountInfo<'a7>,
    ///Rent info
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionV3Keys {
    ///Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: Pubkey,
    ///Metadata mint
    pub mint: Pubkey,
    ///Update authority
    pub update_authority: Pubkey,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///Metadata account
    pub metadata: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&CreateMasterEditionV3Accounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for CreateMasterEditionV3Keys
{
    fn from(
        accounts: &CreateMasterEditionV3Accounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            edition: *accounts.edition.key,
            mint: *accounts.mint.key,
            update_authority: *accounts.update_authority.key,
            mint_authority: *accounts.mint_authority.key,
            payer: *accounts.payer.key,
            metadata: *accounts.metadata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&CreateMasterEditionV3Keys> for [AccountMeta; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN] {
    fn from(keys: &CreateMasterEditionV3Keys) -> Self {
        [
            AccountMeta::new(keys.edition, false),
            AccountMeta::new(keys.mint, false),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.mint_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct CreateMasterEditionV3IxArgs {
    create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionV3IxData<'me>(pub &'me CreateMasterEditionV3IxArgs);
pub const CREATE_MASTER_EDITION_V3_IX_DISCM: u8 = 17u8;
impl<'me> From<&'me CreateMasterEditionV3IxArgs> for CreateMasterEditionV3IxData<'me> {
    fn from(args: &'me CreateMasterEditionV3IxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for CreateMasterEditionV3IxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_MASTER_EDITION_V3_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn create_master_edition_v3_ix<
    K: Into<CreateMasterEditionV3Keys>,
    A: Into<CreateMasterEditionV3IxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: CreateMasterEditionV3Keys = accounts.into();
    let metas: [AccountMeta; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: CreateMasterEditionV3IxArgs = args.into();
    let data: CreateMasterEditionV3IxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_master_edition_v3_invoke<'a, A: Into<CreateMasterEditionV3IxArgs>>(
    accounts: &CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = create_master_edition_v3_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_master_edition_v3_invoke_signed<'a, A: Into<CreateMasterEditionV3IxArgs>>(
    accounts: &CreateMasterEditionV3Accounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_master_edition_v3_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const VERIFY_COLLECTION_IX_ACCOUNTS_LEN: usize = 6usize;
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me, 'a5: 'me>
{
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
    ///Collection Update authority
    pub collection_authority: &'me AccountInfo<'a1>,
    ///payer
    pub payer: &'me AccountInfo<'a2>,
    ///Mint of the Collection
    pub collection_mint: &'me AccountInfo<'a3>,
    ///Metadata Account of the Collection
    pub collection: &'me AccountInfo<'a4>,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionKeys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Collection Update authority
    pub collection_authority: Pubkey,
    ///payer
    pub payer: Pubkey,
    ///Mint of the Collection
    pub collection_mint: Pubkey,
    ///Metadata Account of the Collection
    pub collection: Pubkey,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: Pubkey,
}
impl<'me> From<&VerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>> for VerifyCollectionKeys {
    fn from(accounts: &VerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            collection_authority: *accounts.collection_authority.key,
            payer: *accounts.payer.key,
            collection_mint: *accounts.collection_mint.key,
            collection: *accounts.collection.key,
            collection_master_edition_account: *accounts.collection_master_edition_account.key,
        }
    }
}
impl From<&VerifyCollectionKeys> for [AccountMeta; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] {
    fn from(keys: &VerifyCollectionKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.collection_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.collection_mint, false),
            AccountMeta::new_readonly(keys.collection, false),
            AccountMeta::new_readonly(keys.collection_master_edition_account, false),
        ]
    }
}
impl<'a> From<&VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN]
{
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct VerifyCollectionIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionIxData<'me>(pub &'me VerifyCollectionIxArgs);
pub const VERIFY_COLLECTION_IX_DISCM: u8 = 18u8;
impl<'me> From<&'me VerifyCollectionIxArgs> for VerifyCollectionIxData<'me> {
    fn from(args: &'me VerifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for VerifyCollectionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[VERIFY_COLLECTION_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn verify_collection_ix<K: Into<VerifyCollectionKeys>, A: Into<VerifyCollectionIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: VerifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: VerifyCollectionIxArgs = args.into();
    let data: VerifyCollectionIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn verify_collection_invoke<'a, A: Into<VerifyCollectionIxArgs>>(
    accounts: &VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn verify_collection_invoke_signed<'a, A: Into<VerifyCollectionIxArgs>>(
    accounts: &VerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UTILIZE_IX_ACCOUNTS_LEN: usize = 11usize;
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
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
    ///Token Account Of NFT
    pub token_account: &'me AccountInfo<'a1>,
    ///Mint of the Metadata
    pub mint: &'me AccountInfo<'a2>,
    ///A Use Authority / Can be the current Owner of the NFT
    pub use_authority: &'me AccountInfo<'a3>,
    ///Owner
    pub owner: &'me AccountInfo<'a4>,
    ///Token program
    pub token_program: &'me AccountInfo<'a5>,
    ///Associated Token program
    pub ata_program: &'me AccountInfo<'a6>,
    ///System program
    pub system_program: &'me AccountInfo<'a7>,
    ///Rent info
    pub rent: &'me AccountInfo<'a8>,
    ///Use Authority Record PDA If present the program Assumes a delegated use authority
    pub use_authority_record: &'me AccountInfo<'a9>,
    ///Program As Signer (Burner)
    pub burner: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct UtilizeKeys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Token Account Of NFT
    pub token_account: Pubkey,
    ///Mint of the Metadata
    pub mint: Pubkey,
    ///A Use Authority / Can be the current Owner of the NFT
    pub use_authority: Pubkey,
    ///Owner
    pub owner: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///Associated Token program
    pub ata_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
    ///Use Authority Record PDA If present the program Assumes a delegated use authority
    pub use_authority_record: Pubkey,
    ///Program As Signer (Burner)
    pub burner: Pubkey,
}
impl<'me> From<&UtilizeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>> for UtilizeKeys {
    fn from(accounts: &UtilizeAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            token_account: *accounts.token_account.key,
            mint: *accounts.mint.key,
            use_authority: *accounts.use_authority.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
            ata_program: *accounts.ata_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
            use_authority_record: *accounts.use_authority_record.key,
            burner: *accounts.burner.key,
        }
    }
}
impl From<&UtilizeKeys> for [AccountMeta; UTILIZE_IX_ACCOUNTS_LEN] {
    fn from(keys: &UtilizeKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new(keys.token_account, false),
            AccountMeta::new(keys.mint, false),
            AccountMeta::new_readonly(keys.use_authority, true),
            AccountMeta::new_readonly(keys.owner, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.ata_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
            AccountMeta::new(keys.use_authority_record, false),
            AccountMeta::new_readonly(keys.burner, false),
        ]
    }
}
impl<'a> From<&UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UTILIZE_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UtilizeIxArgs {
    utilize_args: UtilizeArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct UtilizeIxData<'me>(pub &'me UtilizeIxArgs);
pub const UTILIZE_IX_DISCM: u8 = 19u8;
impl<'me> From<&'me UtilizeIxArgs> for UtilizeIxData<'me> {
    fn from(args: &'me UtilizeIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UtilizeIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UTILIZE_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn utilize_ix<K: Into<UtilizeKeys>, A: Into<UtilizeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UtilizeKeys = accounts.into();
    let metas: [AccountMeta; UTILIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UtilizeIxArgs = args.into();
    let data: UtilizeIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn utilize_invoke<'a, A: Into<UtilizeIxArgs>>(
    accounts: &UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = utilize_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UTILIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn utilize_invoke_signed<'a, A: Into<UtilizeIxArgs>>(
    accounts: &UtilizeAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = utilize_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UTILIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 11usize;
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
    ///Use Authority Record PDA
    pub use_authority_record: &'me AccountInfo<'a0>,
    ///Owner
    pub owner: &'me AccountInfo<'a1>,
    ///Payer
    pub payer: &'me AccountInfo<'a2>,
    ///A Use Authority
    pub user: &'me AccountInfo<'a3>,
    ///Owned Token Account Of Mint
    pub owner_token_account: &'me AccountInfo<'a4>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'a5>,
    ///Mint of Metadata
    pub mint: &'me AccountInfo<'a6>,
    ///Program As Signer (Burner)
    pub burner: &'me AccountInfo<'a7>,
    ///Token program
    pub token_program: &'me AccountInfo<'a8>,
    ///System program
    pub system_program: &'me AccountInfo<'a9>,
    ///Rent info
    pub rent: &'me AccountInfo<'a10>,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityKeys {
    ///Use Authority Record PDA
    pub use_authority_record: Pubkey,
    ///Owner
    pub owner: Pubkey,
    ///Payer
    pub payer: Pubkey,
    ///A Use Authority
    pub user: Pubkey,
    ///Owned Token Account Of Mint
    pub owner_token_account: Pubkey,
    ///Metadata account
    pub metadata: Pubkey,
    ///Mint of Metadata
    pub mint: Pubkey,
    ///Program As Signer (Burner)
    pub burner: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&ApproveUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for ApproveUseAuthorityKeys
{
    fn from(
        accounts: &ApproveUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            use_authority_record: *accounts.use_authority_record.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            user: *accounts.user.key,
            owner_token_account: *accounts.owner_token_account.key,
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
            burner: *accounts.burner.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&ApproveUseAuthorityKeys> for [AccountMeta; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &ApproveUseAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.use_authority_record, false),
            AccountMeta::new_readonly(keys.owner, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.user, false),
            AccountMeta::new(keys.owner_token_account, false),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.burner, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ApproveUseAuthorityIxArgs {
    approve_use_authority_args: ApproveUseAuthorityArgs,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityIxData<'me>(pub &'me ApproveUseAuthorityIxArgs);
pub const APPROVE_USE_AUTHORITY_IX_DISCM: u8 = 20u8;
impl<'me> From<&'me ApproveUseAuthorityIxArgs> for ApproveUseAuthorityIxData<'me> {
    fn from(args: &'me ApproveUseAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ApproveUseAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[APPROVE_USE_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn approve_use_authority_ix<
    K: Into<ApproveUseAuthorityKeys>,
    A: Into<ApproveUseAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ApproveUseAuthorityKeys = accounts.into();
    let metas: [AccountMeta; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ApproveUseAuthorityIxArgs = args.into();
    let data: ApproveUseAuthorityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_use_authority_invoke<'a, A: Into<ApproveUseAuthorityIxArgs>>(
    accounts: &ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = approve_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_use_authority_invoke_signed<'a, A: Into<ApproveUseAuthorityIxArgs>>(
    accounts: &ApproveUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = approve_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 9usize;
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
    ///Use Authority Record PDA
    pub use_authority_record: &'me AccountInfo<'a0>,
    ///Owner
    pub owner: &'me AccountInfo<'a1>,
    ///A Use Authority
    pub user: &'me AccountInfo<'a2>,
    ///Owned Token Account Of Mint
    pub owner_token_account: &'me AccountInfo<'a3>,
    ///Mint of Metadata
    pub mint: &'me AccountInfo<'a4>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'a5>,
    ///Token program
    pub token_program: &'me AccountInfo<'a6>,
    ///System program
    pub system_program: &'me AccountInfo<'a7>,
    ///Rent info
    pub rent: &'me AccountInfo<'a8>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeUseAuthorityKeys {
    ///Use Authority Record PDA
    pub use_authority_record: Pubkey,
    ///Owner
    pub owner: Pubkey,
    ///A Use Authority
    pub user: Pubkey,
    ///Owned Token Account Of Mint
    pub owner_token_account: Pubkey,
    ///Mint of Metadata
    pub mint: Pubkey,
    ///Metadata account
    pub metadata: Pubkey,
    ///Token program
    pub token_program: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&RevokeUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>>
    for RevokeUseAuthorityKeys
{
    fn from(
        accounts: &RevokeUseAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            use_authority_record: *accounts.use_authority_record.key,
            owner: *accounts.owner.key,
            user: *accounts.user.key,
            owner_token_account: *accounts.owner_token_account.key,
            mint: *accounts.mint.key,
            metadata: *accounts.metadata.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&RevokeUseAuthorityKeys> for [AccountMeta; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN] {
    fn from(keys: &RevokeUseAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.use_authority_record, false),
            AccountMeta::new_readonly(keys.owner, true),
            AccountMeta::new_readonly(keys.user, false),
            AccountMeta::new(keys.owner_token_account, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.token_program, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RevokeUseAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RevokeUseAuthorityIxData<'me>(pub &'me RevokeUseAuthorityIxArgs);
pub const REVOKE_USE_AUTHORITY_IX_DISCM: u8 = 21u8;
impl<'me> From<&'me RevokeUseAuthorityIxArgs> for RevokeUseAuthorityIxData<'me> {
    fn from(args: &'me RevokeUseAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RevokeUseAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REVOKE_USE_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn revoke_use_authority_ix<
    K: Into<RevokeUseAuthorityKeys>,
    A: Into<RevokeUseAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RevokeUseAuthorityKeys = accounts.into();
    let metas: [AccountMeta; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RevokeUseAuthorityIxArgs = args.into();
    let data: RevokeUseAuthorityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_use_authority_invoke<'a, A: Into<RevokeUseAuthorityIxArgs>>(
    accounts: &RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = revoke_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_use_authority_invoke_signed<'a, A: Into<RevokeUseAuthorityIxArgs>>(
    accounts: &RevokeUseAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revoke_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN: usize = 6usize;
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
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
    ///Collection Authority
    pub collection_authority: &'me AccountInfo<'a1>,
    ///Mint of the Collection
    pub collection_mint: &'me AccountInfo<'a2>,
    ///Metadata Account of the Collection
    pub collection: &'me AccountInfo<'a3>,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'me AccountInfo<'a4>,
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'a5>,
}
#[derive(Copy, Clone, Debug)]
pub struct UnverifyCollectionKeys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Collection Authority
    pub collection_authority: Pubkey,
    ///Mint of the Collection
    pub collection_mint: Pubkey,
    ///Metadata Account of the Collection
    pub collection: Pubkey,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: Pubkey,
    ///Collection Authority Record PDA
    pub collection_authority_record: Pubkey,
}
impl<'me> From<&UnverifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>>
    for UnverifyCollectionKeys
{
    fn from(accounts: &UnverifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            collection_authority: *accounts.collection_authority.key,
            collection_mint: *accounts.collection_mint.key,
            collection: *accounts.collection.key,
            collection_master_edition_account: *accounts.collection_master_edition_account.key,
            collection_authority_record: *accounts.collection_authority_record.key,
        }
    }
}
impl From<&UnverifyCollectionKeys> for [AccountMeta; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN] {
    fn from(keys: &UnverifyCollectionKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.collection_authority, true),
            AccountMeta::new_readonly(keys.collection_mint, false),
            AccountMeta::new_readonly(keys.collection, false),
            AccountMeta::new_readonly(keys.collection_master_edition_account, false),
            AccountMeta::new_readonly(keys.collection_authority_record, false),
        ]
    }
}
impl<'a> From<&UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN]
{
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct UnverifyCollectionIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct UnverifyCollectionIxData<'me>(pub &'me UnverifyCollectionIxArgs);
pub const UNVERIFY_COLLECTION_IX_DISCM: u8 = 22u8;
impl<'me> From<&'me UnverifyCollectionIxArgs> for UnverifyCollectionIxData<'me> {
    fn from(args: &'me UnverifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for UnverifyCollectionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[UNVERIFY_COLLECTION_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn unverify_collection_ix<
    K: Into<UnverifyCollectionKeys>,
    A: Into<UnverifyCollectionIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UnverifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UnverifyCollectionIxArgs = args.into();
    let data: UnverifyCollectionIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unverify_collection_invoke<'a, A: Into<UnverifyCollectionIxArgs>>(
    accounts: &UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = unverify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn unverify_collection_invoke_signed<'a, A: Into<UnverifyCollectionIxArgs>>(
    accounts: &UnverifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = unverify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN: usize = 8usize;
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
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'a0>,
    ///A Collection Authority
    pub new_collection_authority: &'me AccountInfo<'a1>,
    ///Update Authority of Collection NFT
    pub update_authority: &'me AccountInfo<'a2>,
    ///Payer
    pub payer: &'me AccountInfo<'a3>,
    ///Collection Metadata account
    pub metadata: &'me AccountInfo<'a4>,
    ///Mint of Collection Metadata
    pub mint: &'me AccountInfo<'a5>,
    ///System program
    pub system_program: &'me AccountInfo<'a6>,
    ///Rent info
    pub rent: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityKeys {
    ///Collection Authority Record PDA
    pub collection_authority_record: Pubkey,
    ///A Collection Authority
    pub new_collection_authority: Pubkey,
    ///Update Authority of Collection NFT
    pub update_authority: Pubkey,
    ///Payer
    pub payer: Pubkey,
    ///Collection Metadata account
    pub metadata: Pubkey,
    ///Mint of Collection Metadata
    pub mint: Pubkey,
    ///System program
    pub system_program: Pubkey,
    ///Rent info
    pub rent: Pubkey,
}
impl<'me> From<&ApproveCollectionAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for ApproveCollectionAuthorityKeys
{
    fn from(
        accounts: &ApproveCollectionAuthorityAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            collection_authority_record: *accounts.collection_authority_record.key,
            new_collection_authority: *accounts.new_collection_authority.key,
            update_authority: *accounts.update_authority.key,
            payer: *accounts.payer.key,
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
            system_program: *accounts.system_program.key,
            rent: *accounts.rent.key,
        }
    }
}
impl From<&ApproveCollectionAuthorityKeys>
    for [AccountMeta; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &ApproveCollectionAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.collection_authority_record, false),
            AccountMeta::new_readonly(keys.new_collection_authority, false),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.system_program, false),
            AccountMeta::new_readonly(keys.rent, false),
        ]
    }
}
impl<'a> From<&ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ApproveCollectionAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityIxData<'me>(pub &'me ApproveCollectionAuthorityIxArgs);
pub const APPROVE_COLLECTION_AUTHORITY_IX_DISCM: u8 = 23u8;
impl<'me> From<&'me ApproveCollectionAuthorityIxArgs> for ApproveCollectionAuthorityIxData<'me> {
    fn from(args: &'me ApproveCollectionAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ApproveCollectionAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[APPROVE_COLLECTION_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn approve_collection_authority_ix<
    K: Into<ApproveCollectionAuthorityKeys>,
    A: Into<ApproveCollectionAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ApproveCollectionAuthorityKeys = accounts.into();
    let metas: [AccountMeta; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ApproveCollectionAuthorityIxArgs = args.into();
    let data: ApproveCollectionAuthorityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_collection_authority_invoke<'a, A: Into<ApproveCollectionAuthorityIxArgs>>(
    accounts: &ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = approve_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_collection_authority_invoke_signed<'a, A: Into<ApproveCollectionAuthorityIxArgs>>(
    accounts: &ApproveCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = approve_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4usize;
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me> {
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'a0>,
    ///Update Authority of Collection NFT
    pub update_authority: &'me AccountInfo<'a1>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'a2>,
    ///Mint of Metadata
    pub mint: &'me AccountInfo<'a3>,
}
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityKeys {
    ///Collection Authority Record PDA
    pub collection_authority_record: Pubkey,
    ///Update Authority of Collection NFT
    pub update_authority: Pubkey,
    ///Metadata account
    pub metadata: Pubkey,
    ///Mint of Metadata
    pub mint: Pubkey,
}
impl<'me> From<&RevokeCollectionAuthorityAccounts<'me, '_, '_, '_, '_>>
    for RevokeCollectionAuthorityKeys
{
    fn from(accounts: &RevokeCollectionAuthorityAccounts<'me, '_, '_, '_, '_>) -> Self {
        Self {
            collection_authority_record: *accounts.collection_authority_record.key,
            update_authority: *accounts.update_authority.key,
            metadata: *accounts.metadata.key,
            mint: *accounts.mint.key,
        }
    }
}
impl From<&RevokeCollectionAuthorityKeys>
    for [AccountMeta; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RevokeCollectionAuthorityKeys) -> Self {
        [
            AccountMeta::new(keys.collection_authority_record, false),
            AccountMeta::new_readonly(keys.update_authority, true),
            AccountMeta::new_readonly(keys.metadata, false),
            AccountMeta::new_readonly(keys.mint, false),
        ]
    }
}
impl<'a> From<&RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>) -> Self {
        [
            accounts.collection_authority_record.clone(),
            accounts.update_authority.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
        ]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RevokeCollectionAuthorityIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityIxData<'me>(pub &'me RevokeCollectionAuthorityIxArgs);
pub const REVOKE_COLLECTION_AUTHORITY_IX_DISCM: u8 = 24u8;
impl<'me> From<&'me RevokeCollectionAuthorityIxArgs> for RevokeCollectionAuthorityIxData<'me> {
    fn from(args: &'me RevokeCollectionAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RevokeCollectionAuthorityIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REVOKE_COLLECTION_AUTHORITY_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn revoke_collection_authority_ix<
    K: Into<RevokeCollectionAuthorityKeys>,
    A: Into<RevokeCollectionAuthorityIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RevokeCollectionAuthorityKeys = accounts.into();
    let metas: [AccountMeta; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RevokeCollectionAuthorityIxArgs = args.into();
    let data: RevokeCollectionAuthorityIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_collection_authority_invoke<'a, A: Into<RevokeCollectionAuthorityIxArgs>>(
    accounts: &RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = revoke_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_collection_authority_invoke_signed<'a, A: Into<RevokeCollectionAuthorityIxArgs>>(
    accounts: &RevokeCollectionAuthorityAccounts<'_, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revoke_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN: usize = 8usize;
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
    ///Metadata account
    pub metadata: &'me AccountInfo<'a0>,
    ///Collection Update authority
    pub collection_authority: &'me AccountInfo<'a1>,
    ///Payer
    pub payer: &'me AccountInfo<'a2>,
    ///Update Authority of Collection NFT and NFT
    pub update_authority: &'me AccountInfo<'a3>,
    ///Mint of the Collection
    pub collection_mint: &'me AccountInfo<'a4>,
    ///Metadata Account of the Collection
    pub collection: &'me AccountInfo<'a5>,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'me AccountInfo<'a6>,
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'a7>,
}
#[derive(Copy, Clone, Debug)]
pub struct SetAndVerifyCollectionKeys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Collection Update authority
    pub collection_authority: Pubkey,
    ///Payer
    pub payer: Pubkey,
    ///Update Authority of Collection NFT and NFT
    pub update_authority: Pubkey,
    ///Mint of the Collection
    pub collection_mint: Pubkey,
    ///Metadata Account of the Collection
    pub collection: Pubkey,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: Pubkey,
    ///Collection Authority Record PDA
    pub collection_authority_record: Pubkey,
}
impl<'me> From<&SetAndVerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>>
    for SetAndVerifyCollectionKeys
{
    fn from(
        accounts: &SetAndVerifyCollectionAccounts<'me, '_, '_, '_, '_, '_, '_, '_, '_>,
    ) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            collection_authority: *accounts.collection_authority.key,
            payer: *accounts.payer.key,
            update_authority: *accounts.update_authority.key,
            collection_mint: *accounts.collection_mint.key,
            collection: *accounts.collection.key,
            collection_master_edition_account: *accounts.collection_master_edition_account.key,
            collection_authority_record: *accounts.collection_authority_record.key,
        }
    }
}
impl From<&SetAndVerifyCollectionKeys>
    for [AccountMeta; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &SetAndVerifyCollectionKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.collection_authority, true),
            AccountMeta::new_readonly(keys.payer, true),
            AccountMeta::new_readonly(keys.update_authority, false),
            AccountMeta::new_readonly(keys.collection_mint, false),
            AccountMeta::new_readonly(keys.collection, false),
            AccountMeta::new_readonly(keys.collection_master_edition_account, false),
            AccountMeta::new_readonly(keys.collection_authority_record, false),
        ]
    }
}
impl<'a> From<&SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct SetAndVerifyCollectionIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct SetAndVerifyCollectionIxData<'me>(pub &'me SetAndVerifyCollectionIxArgs);
pub const SET_AND_VERIFY_COLLECTION_IX_DISCM: u8 = 25u8;
impl<'me> From<&'me SetAndVerifyCollectionIxArgs> for SetAndVerifyCollectionIxData<'me> {
    fn from(args: &'me SetAndVerifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for SetAndVerifyCollectionIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[SET_AND_VERIFY_COLLECTION_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn set_and_verify_collection_ix<
    K: Into<SetAndVerifyCollectionKeys>,
    A: Into<SetAndVerifyCollectionIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SetAndVerifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SetAndVerifyCollectionIxArgs = args.into();
    let data: SetAndVerifyCollectionIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_and_verify_collection_invoke<'a, A: Into<SetAndVerifyCollectionIxArgs>>(
    accounts: &SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = set_and_verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_and_verify_collection_invoke_signed<'a, A: Into<SetAndVerifyCollectionIxArgs>>(
    accounts: &SetAndVerifyCollectionAccounts<'_, 'a, 'a, 'a, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_and_verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5usize;
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    ///Delegate
    pub delegate: &'me AccountInfo<'a0>,
    ///Token account to freeze
    pub token_account: &'me AccountInfo<'a1>,
    ///Edition
    pub edition: &'me AccountInfo<'a2>,
    ///Token mint
    pub mint: &'me AccountInfo<'a3>,
    ///Token Program
    pub token_program: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountKeys {
    ///Delegate
    pub delegate: Pubkey,
    ///Token account to freeze
    pub token_account: Pubkey,
    ///Edition
    pub edition: Pubkey,
    ///Token mint
    pub mint: Pubkey,
    ///Token Program
    pub token_program: Pubkey,
}
impl<'me> From<&FreezeDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>>
    for FreezeDelegatedAccountKeys
{
    fn from(accounts: &FreezeDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            delegate: *accounts.delegate.key,
            token_account: *accounts.token_account.key,
            edition: *accounts.edition.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&FreezeDelegatedAccountKeys> for [AccountMeta; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &FreezeDelegatedAccountKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.delegate, true),
            AccountMeta::new(keys.token_account, false),
            AccountMeta::new_readonly(keys.edition, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]
{
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct FreezeDelegatedAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountIxData<'me>(pub &'me FreezeDelegatedAccountIxArgs);
pub const FREEZE_DELEGATED_ACCOUNT_IX_DISCM: u8 = 26u8;
impl<'me> From<&'me FreezeDelegatedAccountIxArgs> for FreezeDelegatedAccountIxData<'me> {
    fn from(args: &'me FreezeDelegatedAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for FreezeDelegatedAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[FREEZE_DELEGATED_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn freeze_delegated_account_ix<
    K: Into<FreezeDelegatedAccountKeys>,
    A: Into<FreezeDelegatedAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: FreezeDelegatedAccountKeys = accounts.into();
    let metas: [AccountMeta; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: FreezeDelegatedAccountIxArgs = args.into();
    let data: FreezeDelegatedAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn freeze_delegated_account_invoke<'a, A: Into<FreezeDelegatedAccountIxArgs>>(
    accounts: &FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = freeze_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn freeze_delegated_account_invoke_signed<'a, A: Into<FreezeDelegatedAccountIxArgs>>(
    accounts: &FreezeDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = freeze_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5usize;
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountAccounts<'me, 'a0: 'me, 'a1: 'me, 'a2: 'me, 'a3: 'me, 'a4: 'me> {
    ///Delegate
    pub delegate: &'me AccountInfo<'a0>,
    ///Token account to thaw
    pub token_account: &'me AccountInfo<'a1>,
    ///Edition
    pub edition: &'me AccountInfo<'a2>,
    ///Token mint
    pub mint: &'me AccountInfo<'a3>,
    ///Token Program
    pub token_program: &'me AccountInfo<'a4>,
}
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountKeys {
    ///Delegate
    pub delegate: Pubkey,
    ///Token account to thaw
    pub token_account: Pubkey,
    ///Edition
    pub edition: Pubkey,
    ///Token mint
    pub mint: Pubkey,
    ///Token Program
    pub token_program: Pubkey,
}
impl<'me> From<&ThawDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>>
    for ThawDelegatedAccountKeys
{
    fn from(accounts: &ThawDelegatedAccountAccounts<'me, '_, '_, '_, '_, '_>) -> Self {
        Self {
            delegate: *accounts.delegate.key,
            token_account: *accounts.token_account.key,
            edition: *accounts.edition.key,
            mint: *accounts.mint.key,
            token_program: *accounts.token_program.key,
        }
    }
}
impl From<&ThawDelegatedAccountKeys> for [AccountMeta; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] {
    fn from(keys: &ThawDelegatedAccountKeys) -> Self {
        [
            AccountMeta::new_readonly(keys.delegate, true),
            AccountMeta::new(keys.token_account, false),
            AccountMeta::new_readonly(keys.edition, false),
            AccountMeta::new_readonly(keys.mint, false),
            AccountMeta::new_readonly(keys.token_program, false),
        ]
    }
}
impl<'a> From<&ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>>
    for [AccountInfo<'a>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]
{
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
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct ThawDelegatedAccountIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountIxData<'me>(pub &'me ThawDelegatedAccountIxArgs);
pub const THAW_DELEGATED_ACCOUNT_IX_DISCM: u8 = 27u8;
impl<'me> From<&'me ThawDelegatedAccountIxArgs> for ThawDelegatedAccountIxData<'me> {
    fn from(args: &'me ThawDelegatedAccountIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for ThawDelegatedAccountIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[THAW_DELEGATED_ACCOUNT_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn thaw_delegated_account_ix<
    K: Into<ThawDelegatedAccountKeys>,
    A: Into<ThawDelegatedAccountIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: ThawDelegatedAccountKeys = accounts.into();
    let metas: [AccountMeta; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: ThawDelegatedAccountIxArgs = args.into();
    let data: ThawDelegatedAccountIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn thaw_delegated_account_invoke<'a, A: Into<ThawDelegatedAccountIxArgs>>(
    accounts: &ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = thaw_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn thaw_delegated_account_invoke_signed<'a, A: Into<ThawDelegatedAccountIxArgs>>(
    accounts: &ThawDelegatedAccountAccounts<'_, 'a, 'a, 'a, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = thaw_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub const REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN: usize = 2usize;
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationAccounts<'me, 'a0: 'me, 'a1: 'me> {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'a0>,
    ///Creator
    pub creator: &'me AccountInfo<'a1>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationKeys {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Creator
    pub creator: Pubkey,
}
impl<'me> From<&RemoveCreatorVerificationAccounts<'me, '_, '_>> for RemoveCreatorVerificationKeys {
    fn from(accounts: &RemoveCreatorVerificationAccounts<'me, '_, '_>) -> Self {
        Self {
            metadata: *accounts.metadata.key,
            creator: *accounts.creator.key,
        }
    }
}
impl From<&RemoveCreatorVerificationKeys>
    for [AccountMeta; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]
{
    fn from(keys: &RemoveCreatorVerificationKeys) -> Self {
        [
            AccountMeta::new(keys.metadata, false),
            AccountMeta::new_readonly(keys.creator, true),
        ]
    }
}
impl<'a> From<&RemoveCreatorVerificationAccounts<'_, 'a, 'a>>
    for [AccountInfo<'a>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveCreatorVerificationAccounts<'_, 'a, 'a>) -> Self {
        [accounts.metadata.clone(), accounts.creator.clone()]
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug)]
pub struct RemoveCreatorVerificationIxArgs {}
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationIxData<'me>(pub &'me RemoveCreatorVerificationIxArgs);
pub const REMOVE_CREATOR_VERIFICATION_IX_DISCM: u8 = 28u8;
impl<'me> From<&'me RemoveCreatorVerificationIxArgs> for RemoveCreatorVerificationIxData<'me> {
    fn from(args: &'me RemoveCreatorVerificationIxArgs) -> Self {
        Self(args)
    }
}
impl BorshSerialize for RemoveCreatorVerificationIxData<'_> {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_CREATOR_VERIFICATION_IX_DISCM])?;
        self.0.serialize(writer)
    }
}
pub fn remove_creator_verification_ix<
    K: Into<RemoveCreatorVerificationKeys>,
    A: Into<RemoveCreatorVerificationIxArgs>,
>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: RemoveCreatorVerificationKeys = accounts.into();
    let metas: [AccountMeta; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: RemoveCreatorVerificationIxArgs = args.into();
    let data: RemoveCreatorVerificationIxData = (&args_full).into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_creator_verification_invoke<'a, A: Into<RemoveCreatorVerificationIxArgs>>(
    accounts: &RemoveCreatorVerificationAccounts<'_, 'a, 'a>,
    args: A,
) -> ProgramResult {
    let ix = remove_creator_verification_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_creator_verification_invoke_signed<'a, A: Into<RemoveCreatorVerificationIxArgs>>(
    accounts: &RemoveCreatorVerificationAccounts<'_, 'a, 'a>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_creator_verification_ix(accounts, args)?;
    let account_info: [AccountInfo<'a>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
