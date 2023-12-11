use crate::*;
use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo,
    entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    program_error::ProgramError,
    pubkey::Pubkey,
};
#[derive(Clone, Debug, PartialEq)]
pub enum MplTokenMetadataProgramIx {
    CreateMetadataAccount(CreateMetadataAccountIxArgs),
    UpdateMetadataAccount(UpdateMetadataAccountIxArgs),
    DeprecatedCreateMasterEdition(DeprecatedCreateMasterEditionIxArgs),
    DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(
        DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs,
    ),
    UpdatePrimarySaleHappenedViaToken(UpdatePrimarySaleHappenedViaTokenIxArgs),
    DeprecatedSetReservationList(DeprecatedSetReservationListIxArgs),
    DeprecatedCreateReservationList(DeprecatedCreateReservationListIxArgs),
    SignMetadata(SignMetadataIxArgs),
    DeprecatedMintPrintingTokensViaToken(DeprecatedMintPrintingTokensViaTokenIxArgs),
    DeprecatedMintPrintingTokens(DeprecatedMintPrintingTokensIxArgs),
    CreateMasterEdition(CreateMasterEditionIxArgs),
    MintNewEditionFromMasterEditionViaToken(MintNewEditionFromMasterEditionViaTokenIxArgs),
    ConvertMasterEditionV1ToV2(ConvertMasterEditionV1ToV2IxArgs),
    MintNewEditionFromMasterEditionViaVaultProxy(
        MintNewEditionFromMasterEditionViaVaultProxyIxArgs,
    ),
    PuffMetadata(PuffMetadataIxArgs),
    UpdateMetadataAccountV2(UpdateMetadataAccountV2IxArgs),
    CreateMetadataAccountV2(CreateMetadataAccountV2IxArgs),
    CreateMasterEditionV3(CreateMasterEditionV3IxArgs),
    VerifyCollection(VerifyCollectionIxArgs),
    Utilize(UtilizeIxArgs),
    ApproveUseAuthority(ApproveUseAuthorityIxArgs),
    RevokeUseAuthority(RevokeUseAuthorityIxArgs),
    UnverifyCollection(UnverifyCollectionIxArgs),
    ApproveCollectionAuthority(ApproveCollectionAuthorityIxArgs),
    RevokeCollectionAuthority(RevokeCollectionAuthorityIxArgs),
    SetAndVerifyCollection(SetAndVerifyCollectionIxArgs),
    FreezeDelegatedAccount(FreezeDelegatedAccountIxArgs),
    ThawDelegatedAccount(ThawDelegatedAccountIxArgs),
    RemoveCreatorVerification(RemoveCreatorVerificationIxArgs),
}
impl MplTokenMetadataProgramIx {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        match maybe_discm {
            CREATE_METADATA_ACCOUNT_IX_DISCM => Ok(Self::CreateMetadataAccount(
                CreateMetadataAccountIxArgs::deserialize(buf)?,
            )),
            UPDATE_METADATA_ACCOUNT_IX_DISCM => Ok(Self::UpdateMetadataAccount(
                UpdateMetadataAccountIxArgs::deserialize(buf)?,
            )),
            DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM => Ok(Self::DeprecatedCreateMasterEdition(
                DeprecatedCreateMasterEditionIxArgs::deserialize(buf)?,
            )),
            DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM => Ok(
                Self::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(
                    DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs::deserialize(
                        buf,
                    )?,
                ),
            ),
            UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM => {
                Ok(Self::UpdatePrimarySaleHappenedViaToken(
                    UpdatePrimarySaleHappenedViaTokenIxArgs::deserialize(buf)?,
                ))
            }
            DEPRECATED_SET_RESERVATION_LIST_IX_DISCM => Ok(Self::DeprecatedSetReservationList(
                DeprecatedSetReservationListIxArgs::deserialize(buf)?,
            )),
            DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM => {
                Ok(Self::DeprecatedCreateReservationList(
                    DeprecatedCreateReservationListIxArgs::deserialize(buf)?,
                ))
            }
            SIGN_METADATA_IX_DISCM => Ok(Self::SignMetadata(SignMetadataIxArgs::deserialize(buf)?)),
            DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM => {
                Ok(Self::DeprecatedMintPrintingTokensViaToken(
                    DeprecatedMintPrintingTokensViaTokenIxArgs::deserialize(buf)?,
                ))
            }
            DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM => Ok(Self::DeprecatedMintPrintingTokens(
                DeprecatedMintPrintingTokensIxArgs::deserialize(buf)?,
            )),
            CREATE_MASTER_EDITION_IX_DISCM => Ok(Self::CreateMasterEdition(
                CreateMasterEditionIxArgs::deserialize(buf)?,
            )),
            MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM => {
                Ok(Self::MintNewEditionFromMasterEditionViaToken(
                    MintNewEditionFromMasterEditionViaTokenIxArgs::deserialize(buf)?,
                ))
            }
            CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM => Ok(Self::ConvertMasterEditionV1ToV2(
                ConvertMasterEditionV1ToV2IxArgs::deserialize(buf)?,
            )),
            MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM => {
                Ok(Self::MintNewEditionFromMasterEditionViaVaultProxy(
                    MintNewEditionFromMasterEditionViaVaultProxyIxArgs::deserialize(buf)?,
                ))
            }
            PUFF_METADATA_IX_DISCM => Ok(Self::PuffMetadata(PuffMetadataIxArgs::deserialize(buf)?)),
            UPDATE_METADATA_ACCOUNT_V2_IX_DISCM => Ok(Self::UpdateMetadataAccountV2(
                UpdateMetadataAccountV2IxArgs::deserialize(buf)?,
            )),
            CREATE_METADATA_ACCOUNT_V2_IX_DISCM => Ok(Self::CreateMetadataAccountV2(
                CreateMetadataAccountV2IxArgs::deserialize(buf)?,
            )),
            CREATE_MASTER_EDITION_V3_IX_DISCM => Ok(Self::CreateMasterEditionV3(
                CreateMasterEditionV3IxArgs::deserialize(buf)?,
            )),
            VERIFY_COLLECTION_IX_DISCM => Ok(Self::VerifyCollection(
                VerifyCollectionIxArgs::deserialize(buf)?,
            )),
            UTILIZE_IX_DISCM => Ok(Self::Utilize(UtilizeIxArgs::deserialize(buf)?)),
            APPROVE_USE_AUTHORITY_IX_DISCM => Ok(Self::ApproveUseAuthority(
                ApproveUseAuthorityIxArgs::deserialize(buf)?,
            )),
            REVOKE_USE_AUTHORITY_IX_DISCM => Ok(Self::RevokeUseAuthority(
                RevokeUseAuthorityIxArgs::deserialize(buf)?,
            )),
            UNVERIFY_COLLECTION_IX_DISCM => Ok(Self::UnverifyCollection(
                UnverifyCollectionIxArgs::deserialize(buf)?,
            )),
            APPROVE_COLLECTION_AUTHORITY_IX_DISCM => Ok(Self::ApproveCollectionAuthority(
                ApproveCollectionAuthorityIxArgs::deserialize(buf)?,
            )),
            REVOKE_COLLECTION_AUTHORITY_IX_DISCM => Ok(Self::RevokeCollectionAuthority(
                RevokeCollectionAuthorityIxArgs::deserialize(buf)?,
            )),
            SET_AND_VERIFY_COLLECTION_IX_DISCM => Ok(Self::SetAndVerifyCollection(
                SetAndVerifyCollectionIxArgs::deserialize(buf)?,
            )),
            FREEZE_DELEGATED_ACCOUNT_IX_DISCM => Ok(Self::FreezeDelegatedAccount(
                FreezeDelegatedAccountIxArgs::deserialize(buf)?,
            )),
            THAW_DELEGATED_ACCOUNT_IX_DISCM => Ok(Self::ThawDelegatedAccount(
                ThawDelegatedAccountIxArgs::deserialize(buf)?,
            )),
            REMOVE_CREATOR_VERIFICATION_IX_DISCM => Ok(Self::RemoveCreatorVerification(
                RemoveCreatorVerificationIxArgs::deserialize(buf)?,
            )),
            _ => Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("discm {:?} not found", maybe_discm),
            )),
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::CreateMetadataAccount(args) => {
                CREATE_METADATA_ACCOUNT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateMetadataAccount(args) => {
                UPDATE_METADATA_ACCOUNT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeprecatedCreateMasterEdition(args) => {
                DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeprecatedMintNewEditionFromMasterEditionViaPrintingToken(args) => {
                DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM
                    .serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdatePrimarySaleHappenedViaToken(args) => {
                UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeprecatedSetReservationList(args) => {
                DEPRECATED_SET_RESERVATION_LIST_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeprecatedCreateReservationList(args) => {
                DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SignMetadata(args) => {
                SIGN_METADATA_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeprecatedMintPrintingTokensViaToken(args) => {
                DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::DeprecatedMintPrintingTokens(args) => {
                DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CreateMasterEdition(args) => {
                CREATE_MASTER_EDITION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::MintNewEditionFromMasterEditionViaToken(args) => {
                MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ConvertMasterEditionV1ToV2(args) => {
                CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::MintNewEditionFromMasterEditionViaVaultProxy(args) => {
                MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM
                    .serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::PuffMetadata(args) => {
                PUFF_METADATA_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UpdateMetadataAccountV2(args) => {
                UPDATE_METADATA_ACCOUNT_V2_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CreateMetadataAccountV2(args) => {
                CREATE_METADATA_ACCOUNT_V2_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::CreateMasterEditionV3(args) => {
                CREATE_MASTER_EDITION_V3_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::VerifyCollection(args) => {
                VERIFY_COLLECTION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::Utilize(args) => {
                UTILIZE_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ApproveUseAuthority(args) => {
                APPROVE_USE_AUTHORITY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RevokeUseAuthority(args) => {
                REVOKE_USE_AUTHORITY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::UnverifyCollection(args) => {
                UNVERIFY_COLLECTION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ApproveCollectionAuthority(args) => {
                APPROVE_COLLECTION_AUTHORITY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RevokeCollectionAuthority(args) => {
                REVOKE_COLLECTION_AUTHORITY_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::SetAndVerifyCollection(args) => {
                SET_AND_VERIFY_COLLECTION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::FreezeDelegatedAccount(args) => {
                FREEZE_DELEGATED_ACCOUNT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::ThawDelegatedAccount(args) => {
                THAW_DELEGATED_ACCOUNT_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
            Self::RemoveCreatorVerification(args) => {
                REMOVE_CREATOR_VERIFICATION_IX_DISCM.serialize(&mut writer)?;
                args.serialize(&mut writer)
            }
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountAccounts<'me, 'info> {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Mint of token asset
    pub mint: &'me AccountInfo<'info>,
    ///Mint authority
    pub mint_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///update authority info
    pub update_authority: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&CreateMetadataAccountAccounts<'_, '_>> for CreateMetadataAccountKeys {
    fn from(accounts: &CreateMetadataAccountAccounts) -> Self {
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
impl From<[Pubkey; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]> for CreateMetadataAccountKeys {
    fn from(pubkeys: [Pubkey; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            mint: pubkeys[1],
            mint_authority: pubkeys[2],
            payer: pubkeys[3],
            update_authority: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<&CreateMetadataAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreateMetadataAccountAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]>
    for CreateMetadataAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            mint: &arr[1],
            mint_authority: &arr[2],
            payer: &arr[3],
            update_authority: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMetadataAccountIxArgs {
    pub create_metadata_account_args: CreateMetadataAccountArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMetadataAccountIxData(pub CreateMetadataAccountIxArgs);
pub const CREATE_METADATA_ACCOUNT_IX_DISCM: u8 = 0u8;
impl From<CreateMetadataAccountIxArgs> for CreateMetadataAccountIxData {
    fn from(args: CreateMetadataAccountIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMetadataAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_METADATA_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_METADATA_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateMetadataAccountIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_METADATA_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: CreateMetadataAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_account_invoke<'info, A: Into<CreateMetadataAccountIxArgs>>(
    accounts: &CreateMetadataAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_metadata_account_invoke_signed<'info, A: Into<CreateMetadataAccountIxArgs>>(
    accounts: &CreateMetadataAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_metadata_account_verify_account_keys(
    accounts: &CreateMetadataAccountAccounts<'_, '_>,
    keys: &CreateMetadataAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.mint.key, &keys.mint),
        (accounts.mint_authority.key, &keys.mint_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_metadata_account_verify_account_privileges<'me, 'info>(
    accounts: &CreateMetadataAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.mint_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountAccounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Update authority key
    pub update_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountKeys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Update authority key
    pub update_authority: Pubkey,
}
impl From<&UpdateMetadataAccountAccounts<'_, '_>> for UpdateMetadataAccountKeys {
    fn from(accounts: &UpdateMetadataAccountAccounts) -> Self {
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
impl From<[Pubkey; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]> for UpdateMetadataAccountKeys {
    fn from(pubkeys: [Pubkey; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            update_authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateMetadataAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateMetadataAccountAccounts<'_, 'info>) -> Self {
        [accounts.metadata.clone(), accounts.update_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]>
    for UpdateMetadataAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            update_authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateMetadataAccountIxArgs {
    pub update_metadata_account_args: UpdateMetadataAccountArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMetadataAccountIxData(pub UpdateMetadataAccountIxArgs);
pub const UPDATE_METADATA_ACCOUNT_IX_DISCM: u8 = 1u8;
impl From<UpdateMetadataAccountIxArgs> for UpdateMetadataAccountIxData {
    fn from(args: UpdateMetadataAccountIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMetadataAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != UPDATE_METADATA_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_METADATA_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateMetadataAccountIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_METADATA_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: UpdateMetadataAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_metadata_account_invoke<'info, A: Into<UpdateMetadataAccountIxArgs>>(
    accounts: &UpdateMetadataAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_metadata_account_invoke_signed<'info, A: Into<UpdateMetadataAccountIxArgs>>(
    accounts: &UpdateMetadataAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_metadata_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_metadata_account_verify_account_keys(
    accounts: &UpdateMetadataAccountAccounts<'_, '_>,
    keys: &UpdateMetadataAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.update_authority.key, &keys.update_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_metadata_account_verify_account_privileges<'me, 'info>(
    accounts: &UpdateMetadataAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.update_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN: usize = 13;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateMasterEditionAccounts<'me, 'info> {
    ///Unallocated edition V1 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: &'me AccountInfo<'info>,
    ///Metadata mint
    pub mint: &'me AccountInfo<'info>,
    ///Printing mint - A mint you control that can mint tokens that can be exchanged for limited editions of your master edition via the MintNewEditionFromMasterEditionViaToken endpoint
    pub printing_mint: &'me AccountInfo<'info>,
    ///One time authorization printing mint - A mint you control that prints tokens that gives the bearer permission to mint any number of tokens from the printing mint one time via an endpoint with the token-metadata program for your metadata. Also burns the token.
    pub one_time_printing_authorization_mint: &'me AccountInfo<'info>,
    ///Current Update authority key
    pub update_authority: &'me AccountInfo<'info>,
    ///Printing mint authority - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.
    pub printing_mint_authority: &'me AccountInfo<'info>,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: &'me AccountInfo<'info>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
    ///One time authorization printing mint authority - must be provided if using max supply. THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY.
    pub one_time_printing_authorization_mint_authority: &'me AccountInfo<'info>,
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
impl From<&DeprecatedCreateMasterEditionAccounts<'_, '_>> for DeprecatedCreateMasterEditionKeys {
    fn from(accounts: &DeprecatedCreateMasterEditionAccounts) -> Self {
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
impl From<[Pubkey; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]>
    for DeprecatedCreateMasterEditionKeys
{
    fn from(pubkeys: [Pubkey; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            edition: pubkeys[0],
            mint: pubkeys[1],
            printing_mint: pubkeys[2],
            one_time_printing_authorization_mint: pubkeys[3],
            update_authority: pubkeys[4],
            printing_mint_authority: pubkeys[5],
            mint_authority: pubkeys[6],
            metadata: pubkeys[7],
            payer: pubkeys[8],
            token_program: pubkeys[9],
            system_program: pubkeys[10],
            rent: pubkeys[11],
            one_time_printing_authorization_mint_authority: pubkeys[12],
        }
    }
}
impl<'info> From<&DeprecatedCreateMasterEditionAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeprecatedCreateMasterEditionAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]>
    for DeprecatedCreateMasterEditionAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            edition: &arr[0],
            mint: &arr[1],
            printing_mint: &arr[2],
            one_time_printing_authorization_mint: &arr[3],
            update_authority: &arr[4],
            printing_mint_authority: &arr[5],
            mint_authority: &arr[6],
            metadata: &arr[7],
            payer: &arr[8],
            token_program: &arr[9],
            system_program: &arr[10],
            rent: &arr[11],
            one_time_printing_authorization_mint_authority: &arr[12],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedCreateMasterEditionIxArgs {
    pub create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecatedCreateMasterEditionIxData(pub DeprecatedCreateMasterEditionIxArgs);
pub const DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM: u8 = 2u8;
impl From<DeprecatedCreateMasterEditionIxArgs> for DeprecatedCreateMasterEditionIxData {
    fn from(args: DeprecatedCreateMasterEditionIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecatedCreateMasterEditionIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeprecatedCreateMasterEditionIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_CREATE_MASTER_EDITION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: DeprecatedCreateMasterEditionIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_create_master_edition_invoke<
    'info,
    A: Into<DeprecatedCreateMasterEditionIxArgs>,
>(
    accounts: &DeprecatedCreateMasterEditionAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_create_master_edition_invoke_signed<
    'info,
    A: Into<DeprecatedCreateMasterEditionIxArgs>,
>(
    accounts: &DeprecatedCreateMasterEditionAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deprecated_create_master_edition_verify_account_keys(
    accounts: &DeprecatedCreateMasterEditionAccounts<'_, '_>,
    keys: &DeprecatedCreateMasterEditionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.edition.key, &keys.edition),
        (accounts.mint.key, &keys.mint),
        (accounts.printing_mint.key, &keys.printing_mint),
        (
            accounts.one_time_printing_authorization_mint.key,
            &keys.one_time_printing_authorization_mint,
        ),
        (accounts.update_authority.key, &keys.update_authority),
        (
            accounts.printing_mint_authority.key,
            &keys.printing_mint_authority,
        ),
        (accounts.mint_authority.key, &keys.mint_authority),
        (accounts.metadata.key, &keys.metadata),
        (accounts.payer.key, &keys.payer),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (
            accounts.one_time_printing_authorization_mint_authority.key,
            &keys.one_time_printing_authorization_mint_authority,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deprecated_create_master_edition_verify_account_privileges<'me, 'info>(
    accounts: &DeprecatedCreateMasterEditionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.edition,
        accounts.mint,
        accounts.printing_mint,
        accounts.one_time_printing_authorization_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.update_authority,
        accounts.printing_mint_authority,
        accounts.mint_authority,
        accounts.payer,
        accounts.one_time_printing_authorization_mint_authority,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN:
    usize = 16;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'me, 'info> {
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///New Edition V1 (pda of ['metadata', program id, mint id, 'edition'])
    pub edition: &'me AccountInfo<'info>,
    ///Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint: &'me AccountInfo<'info>,
    ///Mint authority of new mint
    pub mint_authority: &'me AccountInfo<'info>,
    ///Printing Mint of master record edition
    pub printing_mint: &'me AccountInfo<'info>,
    ///Token account containing Printing mint token to be transferred
    pub master_token_account: &'me AccountInfo<'info>,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master mint id, edition_number])
    pub edition_marker: &'me AccountInfo<'info>,
    ///Burn authority for this token
    pub burn_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///update authority info for new metadata account
    pub master_update_authority: &'me AccountInfo<'info>,
    ///Master record metadata account
    pub master_metadata: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
    ///Reservation List - If present, and you are on this list, you can get an edition number given by your position on the list.
    pub reservation_list: &'me AccountInfo<'info>,
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
impl From<&DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'_, '_>>
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys
{
    fn from(accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts) -> Self {
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
impl
    From<
        [Pubkey;
            DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN],
    > for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys
{
    fn from(
        pubkeys: [Pubkey; DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            metadata: pubkeys[0],
            edition: pubkeys[1],
            master_edition: pubkeys[2],
            mint: pubkeys[3],
            mint_authority: pubkeys[4],
            printing_mint: pubkeys[5],
            master_token_account: pubkeys[6],
            edition_marker: pubkeys[7],
            burn_authority: pubkeys[8],
            payer: pubkeys[9],
            master_update_authority: pubkeys[10],
            master_metadata: pubkeys[11],
            token_program: pubkeys[12],
            system_program: pubkeys[13],
            rent: pubkeys[14],
            reservation_list: pubkeys[15],
        }
    }
}
impl<'info> From<&DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(
        accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'_, 'info>,
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
impl<'me, 'info>
    From<
        &'me [AccountInfo<'info>;
                 DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN],
    > for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<
            'info,
        >; DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            metadata: &arr[0],
            edition: &arr[1],
            master_edition: &arr[2],
            mint: &arr[3],
            mint_authority: &arr[4],
            printing_mint: &arr[5],
            master_token_account: &arr[6],
            edition_marker: &arr[7],
            burn_authority: &arr[8],
            payer: &arr[9],
            master_update_authority: &arr[10],
            master_metadata: &arr[11],
            token_program: &arr[12],
            system_program: &arr[13],
            rent: &arr[14],
            reservation_list: &arr[15],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData(
    pub DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs,
);
pub const DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM: u8 = 3u8;
impl From<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>
    for DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData
{
    fn from(args: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm
            != DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM
        {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM,
                    maybe_discm
                ),
            ));
        }
        Ok(Self(
            DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs::deserialize(buf)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[
            DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_DISCM,
        ])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_invoke<
    'info,
    A: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>,
>(
    accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_mint_new_edition_from_master_edition_via_printing_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_invoke_signed<
    'info,
    A: Into<DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenIxArgs>,
>(
    accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_new_edition_from_master_edition_via_printing_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        DEPRECATED_MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_PRINTING_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_verify_account_keys(
    accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'_, '_>,
    keys: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.edition.key, &keys.edition),
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.mint.key, &keys.mint),
        (accounts.mint_authority.key, &keys.mint_authority),
        (accounts.printing_mint.key, &keys.printing_mint),
        (
            accounts.master_token_account.key,
            &keys.master_token_account,
        ),
        (accounts.edition_marker.key, &keys.edition_marker),
        (accounts.burn_authority.key, &keys.burn_authority),
        (accounts.payer.key, &keys.payer),
        (
            accounts.master_update_authority.key,
            &keys.master_update_authority,
        ),
        (accounts.master_metadata.key, &keys.master_metadata),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (accounts.reservation_list.key, &keys.reservation_list),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deprecated_mint_new_edition_from_master_edition_via_printing_token_verify_account_privileges<
    'me,
    'info,
>(
    accounts: &DeprecatedMintNewEditionFromMasterEditionViaPrintingTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.metadata,
        accounts.edition,
        accounts.master_edition,
        accounts.mint,
        accounts.printing_mint,
        accounts.master_token_account,
        accounts.edition_marker,
        accounts.reservation_list,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.mint_authority,
        accounts.burn_authority,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct UpdatePrimarySaleHappenedViaTokenAccounts<'me, 'info> {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Owner on the token account
    pub owner: &'me AccountInfo<'info>,
    ///Account containing tokens from the metadata's mint
    pub token: &'me AccountInfo<'info>,
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
impl From<&UpdatePrimarySaleHappenedViaTokenAccounts<'_, '_>>
    for UpdatePrimarySaleHappenedViaTokenKeys
{
    fn from(accounts: &UpdatePrimarySaleHappenedViaTokenAccounts) -> Self {
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
impl From<[Pubkey; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN]>
    for UpdatePrimarySaleHappenedViaTokenKeys
{
    fn from(pubkeys: [Pubkey; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            owner: pubkeys[1],
            token: pubkeys[2],
        }
    }
}
impl<'info> From<&UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'info>) -> Self {
        [
            accounts.metadata.clone(),
            accounts.owner.clone(),
            accounts.token.clone(),
        ]
    }
}
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN]>
    for UpdatePrimarySaleHappenedViaTokenAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            metadata: &arr[0],
            owner: &arr[1],
            token: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatePrimarySaleHappenedViaTokenIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdatePrimarySaleHappenedViaTokenIxData(pub UpdatePrimarySaleHappenedViaTokenIxArgs);
pub const UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM: u8 = 4u8;
impl From<UpdatePrimarySaleHappenedViaTokenIxArgs> for UpdatePrimarySaleHappenedViaTokenIxData {
    fn from(args: UpdatePrimarySaleHappenedViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl UpdatePrimarySaleHappenedViaTokenIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdatePrimarySaleHappenedViaTokenIxArgs::deserialize(
            buf,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: UpdatePrimarySaleHappenedViaTokenIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_primary_sale_happened_via_token_invoke<
    'info,
    A: Into<UpdatePrimarySaleHappenedViaTokenIxArgs>,
>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_primary_sale_happened_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_primary_sale_happened_via_token_invoke_signed<
    'info,
    A: Into<UpdatePrimarySaleHappenedViaTokenIxArgs>,
>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_primary_sale_happened_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_PRIMARY_SALE_HAPPENED_VIA_TOKEN_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_primary_sale_happened_via_token_verify_account_keys(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'_, '_>,
    keys: &UpdatePrimarySaleHappenedViaTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.owner.key, &keys.owner),
        (accounts.token.key, &keys.token),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_primary_sale_happened_via_token_verify_account_privileges<'me, 'info>(
    accounts: &UpdatePrimarySaleHappenedViaTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedSetReservationListAccounts<'me, 'info> {
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]
    pub reservation_list: &'me AccountInfo<'info>,
    ///The resource you tied the reservation list too
    pub resource: &'me AccountInfo<'info>,
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
impl From<&DeprecatedSetReservationListAccounts<'_, '_>> for DeprecatedSetReservationListKeys {
    fn from(accounts: &DeprecatedSetReservationListAccounts) -> Self {
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
impl From<[Pubkey; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN]>
    for DeprecatedSetReservationListKeys
{
    fn from(pubkeys: [Pubkey; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            master_edition: pubkeys[0],
            reservation_list: pubkeys[1],
            resource: pubkeys[2],
        }
    }
}
impl<'info> From<&DeprecatedSetReservationListAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeprecatedSetReservationListAccounts<'_, 'info>) -> Self {
        [
            accounts.master_edition.clone(),
            accounts.reservation_list.clone(),
            accounts.resource.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN]>
    for DeprecatedSetReservationListAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            master_edition: &arr[0],
            reservation_list: &arr[1],
            resource: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedSetReservationListIxArgs {
    pub set_reservation_list_args: SetReservationListArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecatedSetReservationListIxData(pub DeprecatedSetReservationListIxArgs);
pub const DEPRECATED_SET_RESERVATION_LIST_IX_DISCM: u8 = 5u8;
impl From<DeprecatedSetReservationListIxArgs> for DeprecatedSetReservationListIxData {
    fn from(args: DeprecatedSetReservationListIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecatedSetReservationListIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPRECATED_SET_RESERVATION_LIST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPRECATED_SET_RESERVATION_LIST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeprecatedSetReservationListIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_SET_RESERVATION_LIST_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: DeprecatedSetReservationListIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_set_reservation_list_invoke<
    'info,
    A: Into<DeprecatedSetReservationListIxArgs>,
>(
    accounts: &DeprecatedSetReservationListAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_set_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_set_reservation_list_invoke_signed<
    'info,
    A: Into<DeprecatedSetReservationListIxArgs>,
>(
    accounts: &DeprecatedSetReservationListAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_set_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_SET_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deprecated_set_reservation_list_verify_account_keys(
    accounts: &DeprecatedSetReservationListAccounts<'_, '_>,
    keys: &DeprecatedSetReservationListKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.reservation_list.key, &keys.reservation_list),
        (accounts.resource.key, &keys.resource),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deprecated_set_reservation_list_verify_account_privileges<'me, 'info>(
    accounts: &DeprecatedSetReservationListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.master_edition, accounts.reservation_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.resource] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedCreateReservationListAccounts<'me, 'info> {
    ///PDA for ReservationList of ['metadata', program id, master edition key, 'reservation', resource-key]
    pub reservation_list: &'me AccountInfo<'info>,
    ///Payer
    pub payer: &'me AccountInfo<'info>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'info>,
    /// Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///A resource you wish to tie the reservation list to. This is so your later visitors who come to redeem can derive your reservation list PDA with something they can easily get at. You choose what this should be.
    pub resource: &'me AccountInfo<'info>,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&DeprecatedCreateReservationListAccounts<'_, '_>>
    for DeprecatedCreateReservationListKeys
{
    fn from(accounts: &DeprecatedCreateReservationListAccounts) -> Self {
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
impl From<[Pubkey; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN]>
    for DeprecatedCreateReservationListKeys
{
    fn from(pubkeys: [Pubkey; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            reservation_list: pubkeys[0],
            payer: pubkeys[1],
            update_authority: pubkeys[2],
            master_edition: pubkeys[3],
            resource: pubkeys[4],
            metadata: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<&DeprecatedCreateReservationListAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeprecatedCreateReservationListAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN]>
    for DeprecatedCreateReservationListAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            reservation_list: &arr[0],
            payer: &arr[1],
            update_authority: &arr[2],
            master_edition: &arr[3],
            resource: &arr[4],
            metadata: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedCreateReservationListIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecatedCreateReservationListIxData(pub DeprecatedCreateReservationListIxArgs);
pub const DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM: u8 = 6u8;
impl From<DeprecatedCreateReservationListIxArgs> for DeprecatedCreateReservationListIxData {
    fn from(args: DeprecatedCreateReservationListIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecatedCreateReservationListIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeprecatedCreateReservationListIxArgs::deserialize(
            buf,
        )?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_CREATE_RESERVATION_LIST_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: DeprecatedCreateReservationListIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_create_reservation_list_invoke<
    'info,
    A: Into<DeprecatedCreateReservationListIxArgs>,
>(
    accounts: &DeprecatedCreateReservationListAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_create_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_create_reservation_list_invoke_signed<
    'info,
    A: Into<DeprecatedCreateReservationListIxArgs>,
>(
    accounts: &DeprecatedCreateReservationListAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_create_reservation_list_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_CREATE_RESERVATION_LIST_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deprecated_create_reservation_list_verify_account_keys(
    accounts: &DeprecatedCreateReservationListAccounts<'_, '_>,
    keys: &DeprecatedCreateReservationListKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.reservation_list.key, &keys.reservation_list),
        (accounts.payer.key, &keys.payer),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.resource.key, &keys.resource),
        (accounts.metadata.key, &keys.metadata),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deprecated_create_reservation_list_verify_account_privileges<'me, 'info>(
    accounts: &DeprecatedCreateReservationListAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.reservation_list] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.payer, accounts.update_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SIGN_METADATA_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataAccounts<'me, 'info> {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Creator
    pub creator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct SignMetadataKeys {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Creator
    pub creator: Pubkey,
}
impl From<&SignMetadataAccounts<'_, '_>> for SignMetadataKeys {
    fn from(accounts: &SignMetadataAccounts) -> Self {
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
impl From<[Pubkey; SIGN_METADATA_IX_ACCOUNTS_LEN]> for SignMetadataKeys {
    fn from(pubkeys: [Pubkey; SIGN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            creator: pubkeys[1],
        }
    }
}
impl<'info> From<&SignMetadataAccounts<'_, 'info>>
    for [AccountInfo<'info>; SIGN_METADATA_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SignMetadataAccounts<'_, 'info>) -> Self {
        [accounts.metadata.clone(), accounts.creator.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; SIGN_METADATA_IX_ACCOUNTS_LEN]>
    for SignMetadataAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SIGN_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            creator: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SignMetadataIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SignMetadataIxData(pub SignMetadataIxArgs);
pub const SIGN_METADATA_IX_DISCM: u8 = 7u8;
impl From<SignMetadataIxArgs> for SignMetadataIxData {
    fn from(args: SignMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl SignMetadataIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SIGN_METADATA_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SIGN_METADATA_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SignMetadataIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SIGN_METADATA_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn sign_metadata_ix<K: Into<SignMetadataKeys>, A: Into<SignMetadataIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: SignMetadataKeys = accounts.into();
    let metas: [AccountMeta; SIGN_METADATA_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: SignMetadataIxArgs = args.into();
    let data: SignMetadataIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn sign_metadata_invoke<'info, A: Into<SignMetadataIxArgs>>(
    accounts: &SignMetadataAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = sign_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SIGN_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn sign_metadata_invoke_signed<'info, A: Into<SignMetadataIxArgs>>(
    accounts: &SignMetadataAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = sign_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SIGN_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn sign_metadata_verify_account_keys(
    accounts: &SignMetadataAccounts<'_, '_>,
    keys: &SignMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.creator.key, &keys.creator),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn sign_metadata_verify_account_privileges<'me, 'info>(
    accounts: &SignMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensViaTokenAccounts<'me, 'info> {
    ///Destination account
    pub destination: &'me AccountInfo<'info>,
    ///Token account containing one time authorization token
    pub token: &'me AccountInfo<'info>,
    ///One time authorization mint
    pub one_time_printing_authorization_mint: &'me AccountInfo<'info>,
    ///Printing mint
    pub printing_mint: &'me AccountInfo<'info>,
    ///Burn authority
    pub burn_authority: &'me AccountInfo<'info>,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///Rent
    pub rent: &'me AccountInfo<'info>,
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
impl From<&DeprecatedMintPrintingTokensViaTokenAccounts<'_, '_>>
    for DeprecatedMintPrintingTokensViaTokenKeys
{
    fn from(accounts: &DeprecatedMintPrintingTokensViaTokenAccounts) -> Self {
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
impl From<[Pubkey; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN]>
    for DeprecatedMintPrintingTokensViaTokenKeys
{
    fn from(pubkeys: [Pubkey; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            destination: pubkeys[0],
            token: pubkeys[1],
            one_time_printing_authorization_mint: pubkeys[2],
            printing_mint: pubkeys[3],
            burn_authority: pubkeys[4],
            metadata: pubkeys[5],
            master_edition: pubkeys[6],
            token_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<&DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN]>
    for DeprecatedMintPrintingTokensViaTokenAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            destination: &arr[0],
            token: &arr[1],
            one_time_printing_authorization_mint: &arr[2],
            printing_mint: &arr[3],
            burn_authority: &arr[4],
            metadata: &arr[5],
            master_edition: &arr[6],
            token_program: &arr[7],
            rent: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedMintPrintingTokensViaTokenIxArgs {
    pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecatedMintPrintingTokensViaTokenIxData(
    pub DeprecatedMintPrintingTokensViaTokenIxArgs,
);
pub const DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM: u8 = 8u8;
impl From<DeprecatedMintPrintingTokensViaTokenIxArgs>
    for DeprecatedMintPrintingTokensViaTokenIxData
{
    fn from(args: DeprecatedMintPrintingTokensViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecatedMintPrintingTokensViaTokenIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            DeprecatedMintPrintingTokensViaTokenIxArgs::deserialize(buf)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: DeprecatedMintPrintingTokensViaTokenIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_printing_tokens_via_token_invoke<
    'info,
    A: Into<DeprecatedMintPrintingTokensViaTokenIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_printing_tokens_via_token_invoke_signed<
    'info,
    A: Into<DeprecatedMintPrintingTokensViaTokenIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        DEPRECATED_MINT_PRINTING_TOKENS_VIA_TOKEN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deprecated_mint_printing_tokens_via_token_verify_account_keys(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'_, '_>,
    keys: &DeprecatedMintPrintingTokensViaTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.destination.key, &keys.destination),
        (accounts.token.key, &keys.token),
        (
            accounts.one_time_printing_authorization_mint.key,
            &keys.one_time_printing_authorization_mint,
        ),
        (accounts.printing_mint.key, &keys.printing_mint),
        (accounts.burn_authority.key, &keys.burn_authority),
        (accounts.metadata.key, &keys.metadata),
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.token_program.key, &keys.token_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deprecated_mint_printing_tokens_via_token_verify_account_privileges<'me, 'info>(
    accounts: &DeprecatedMintPrintingTokensViaTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.destination,
        accounts.token,
        accounts.one_time_printing_authorization_mint,
        accounts.printing_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.burn_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct DeprecatedMintPrintingTokensAccounts<'me, 'info> {
    ///Destination account
    pub destination: &'me AccountInfo<'info>,
    ///Printing mint
    pub printing_mint: &'me AccountInfo<'info>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'info>,
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Master Edition V1 key (pda of ['metadata', program id, mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///Rent
    pub rent: &'me AccountInfo<'info>,
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
impl From<&DeprecatedMintPrintingTokensAccounts<'_, '_>> for DeprecatedMintPrintingTokensKeys {
    fn from(accounts: &DeprecatedMintPrintingTokensAccounts) -> Self {
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
impl From<[Pubkey; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN]>
    for DeprecatedMintPrintingTokensKeys
{
    fn from(pubkeys: [Pubkey; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            destination: pubkeys[0],
            printing_mint: pubkeys[1],
            update_authority: pubkeys[2],
            metadata: pubkeys[3],
            master_edition: pubkeys[4],
            token_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<&DeprecatedMintPrintingTokensAccounts<'_, 'info>>
    for [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN]>
    for DeprecatedMintPrintingTokensAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            destination: &arr[0],
            printing_mint: &arr[1],
            update_authority: &arr[2],
            metadata: &arr[3],
            master_edition: &arr[4],
            token_program: &arr[5],
            rent: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeprecatedMintPrintingTokensIxArgs {
    pub mint_printing_tokens_via_token_args: MintPrintingTokensViaTokenArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct DeprecatedMintPrintingTokensIxData(pub DeprecatedMintPrintingTokensIxArgs);
pub const DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM: u8 = 9u8;
impl From<DeprecatedMintPrintingTokensIxArgs> for DeprecatedMintPrintingTokensIxData {
    fn from(args: DeprecatedMintPrintingTokensIxArgs) -> Self {
        Self(args)
    }
}
impl DeprecatedMintPrintingTokensIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(DeprecatedMintPrintingTokensIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[DEPRECATED_MINT_PRINTING_TOKENS_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: DeprecatedMintPrintingTokensIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn deprecated_mint_printing_tokens_invoke<
    'info,
    A: Into<DeprecatedMintPrintingTokensIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn deprecated_mint_printing_tokens_invoke_signed<
    'info,
    A: Into<DeprecatedMintPrintingTokensIxArgs>,
>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = deprecated_mint_printing_tokens_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; DEPRECATED_MINT_PRINTING_TOKENS_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn deprecated_mint_printing_tokens_verify_account_keys(
    accounts: &DeprecatedMintPrintingTokensAccounts<'_, '_>,
    keys: &DeprecatedMintPrintingTokensKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.destination.key, &keys.destination),
        (accounts.printing_mint.key, &keys.printing_mint),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.metadata.key, &keys.metadata),
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.token_program.key, &keys.token_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn deprecated_mint_printing_tokens_verify_account_privileges<'me, 'info>(
    accounts: &DeprecatedMintPrintingTokensAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.destination, accounts.printing_mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.update_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionAccounts<'me, 'info> {
    ///Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: &'me AccountInfo<'info>,
    ///Metadata mint
    pub mint: &'me AccountInfo<'info>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'info>,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&CreateMasterEditionAccounts<'_, '_>> for CreateMasterEditionKeys {
    fn from(accounts: &CreateMasterEditionAccounts) -> Self {
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
impl From<[Pubkey; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]> for CreateMasterEditionKeys {
    fn from(pubkeys: [Pubkey; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            edition: pubkeys[0],
            mint: pubkeys[1],
            update_authority: pubkeys[2],
            mint_authority: pubkeys[3],
            payer: pubkeys[4],
            metadata: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<&CreateMasterEditionAccounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreateMasterEditionAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]>
    for CreateMasterEditionAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            edition: &arr[0],
            mint: &arr[1],
            update_authority: &arr[2],
            mint_authority: &arr[3],
            payer: &arr[4],
            metadata: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMasterEditionIxArgs {
    pub create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMasterEditionIxData(pub CreateMasterEditionIxArgs);
pub const CREATE_MASTER_EDITION_IX_DISCM: u8 = 10u8;
impl From<CreateMasterEditionIxArgs> for CreateMasterEditionIxData {
    fn from(args: CreateMasterEditionIxArgs) -> Self {
        Self(args)
    }
}
impl CreateMasterEditionIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_MASTER_EDITION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_MASTER_EDITION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateMasterEditionIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_MASTER_EDITION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: CreateMasterEditionIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_master_edition_invoke<'info, A: Into<CreateMasterEditionIxArgs>>(
    accounts: &CreateMasterEditionAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_master_edition_invoke_signed<'info, A: Into<CreateMasterEditionIxArgs>>(
    accounts: &CreateMasterEditionAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_master_edition_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_MASTER_EDITION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_master_edition_verify_account_keys(
    accounts: &CreateMasterEditionAccounts<'_, '_>,
    keys: &CreateMasterEditionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.edition.key, &keys.edition),
        (accounts.mint.key, &keys.mint),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.mint_authority.key, &keys.mint_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.metadata.key, &keys.metadata),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_master_edition_verify_account_privileges<'me, 'info>(
    accounts: &CreateMasterEditionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.edition, accounts.mint] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.update_authority,
        accounts.mint_authority,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN: usize = 14;
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaTokenAccounts<'me, 'info> {
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata: &'me AccountInfo<'info>,
    ///New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: &'me AccountInfo<'info>,
    ///Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_mint: &'me AccountInfo<'info>,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_mark_pda: &'me AccountInfo<'info>,
    ///Mint authority of new mint
    pub new_mint_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///owner of token account containing master token (#8)
    pub token_account_owner: &'me AccountInfo<'info>,
    ///token account containing token from master metadata mint
    pub token_account: &'me AccountInfo<'info>,
    ///Update authority info for new metadata
    pub new_metadata_update_authority: &'me AccountInfo<'info>,
    ///Master record metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&MintNewEditionFromMasterEditionViaTokenAccounts<'_, '_>>
    for MintNewEditionFromMasterEditionViaTokenKeys
{
    fn from(accounts: &MintNewEditionFromMasterEditionViaTokenAccounts) -> Self {
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
impl From<[Pubkey; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN]>
    for MintNewEditionFromMasterEditionViaTokenKeys
{
    fn from(
        pubkeys: [Pubkey; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_metadata: pubkeys[0],
            new_edition: pubkeys[1],
            master_edition: pubkeys[2],
            new_mint: pubkeys[3],
            edition_mark_pda: pubkeys[4],
            new_mint_authority: pubkeys[5],
            payer: pubkeys[6],
            token_account_owner: pubkeys[7],
            token_account: pubkeys[8],
            new_metadata_update_authority: pubkeys[9],
            metadata: pubkeys[10],
            token_program: pubkeys[11],
            system_program: pubkeys[12],
            rent: pubkeys[13],
        }
    }
}
impl<'info> From<&MintNewEditionFromMasterEditionViaTokenAccounts<'_, 'info>>
    for [AccountInfo<'info>; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info>
    From<&'me [AccountInfo<'info>; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN]>
    for MintNewEditionFromMasterEditionViaTokenAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>;
                 MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_metadata: &arr[0],
            new_edition: &arr[1],
            master_edition: &arr[2],
            new_mint: &arr[3],
            edition_mark_pda: &arr[4],
            new_mint_authority: &arr[5],
            payer: &arr[6],
            token_account_owner: &arr[7],
            token_account: &arr[8],
            new_metadata_update_authority: &arr[9],
            metadata: &arr[10],
            token_program: &arr[11],
            system_program: &arr[12],
            rent: &arr[13],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintNewEditionFromMasterEditionViaTokenIxArgs {
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintNewEditionFromMasterEditionViaTokenIxData(
    pub MintNewEditionFromMasterEditionViaTokenIxArgs,
);
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM: u8 = 11u8;
impl From<MintNewEditionFromMasterEditionViaTokenIxArgs>
    for MintNewEditionFromMasterEditionViaTokenIxData
{
    fn from(args: MintNewEditionFromMasterEditionViaTokenIxArgs) -> Self {
        Self(args)
    }
}
impl MintNewEditionFromMasterEditionViaTokenIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            MintNewEditionFromMasterEditionViaTokenIxArgs::deserialize(buf)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: MintNewEditionFromMasterEditionViaTokenIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_new_edition_from_master_edition_via_token_invoke<
    'info,
    A: Into<MintNewEditionFromMasterEditionViaTokenIxArgs>,
>(
    accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_new_edition_from_master_edition_via_token_invoke_signed<
    'info,
    A: Into<MintNewEditionFromMasterEditionViaTokenIxArgs>,
>(
    accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_token_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_TOKEN_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn mint_new_edition_from_master_edition_via_token_verify_account_keys(
    accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<'_, '_>,
    keys: &MintNewEditionFromMasterEditionViaTokenKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.new_metadata.key, &keys.new_metadata),
        (accounts.new_edition.key, &keys.new_edition),
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.new_mint.key, &keys.new_mint),
        (accounts.edition_mark_pda.key, &keys.edition_mark_pda),
        (accounts.new_mint_authority.key, &keys.new_mint_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.token_account_owner.key, &keys.token_account_owner),
        (accounts.token_account.key, &keys.token_account),
        (
            accounts.new_metadata_update_authority.key,
            &keys.new_metadata_update_authority,
        ),
        (accounts.metadata.key, &keys.metadata),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn mint_new_edition_from_master_edition_via_token_verify_account_privileges<'me, 'info>(
    accounts: &MintNewEditionFromMasterEditionViaTokenAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.new_metadata,
        accounts.new_edition,
        accounts.master_edition,
        accounts.new_mint,
        accounts.edition_mark_pda,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.new_mint_authority,
        accounts.payer,
        accounts.token_account_owner,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN: usize = 3;
#[derive(Copy, Clone, Debug)]
pub struct ConvertMasterEditionV1ToV2Accounts<'me, 'info> {
    ///Master Record Edition V1 (pda of ['metadata', program id, master metadata mint id, 'edition'])
    pub master_edition: &'me AccountInfo<'info>,
    ///One time authorization mint
    pub one_time_auth: &'me AccountInfo<'info>,
    ///Printing mint
    pub printing_mint: &'me AccountInfo<'info>,
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
impl From<&ConvertMasterEditionV1ToV2Accounts<'_, '_>> for ConvertMasterEditionV1ToV2Keys {
    fn from(accounts: &ConvertMasterEditionV1ToV2Accounts) -> Self {
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
impl From<[Pubkey; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN]>
    for ConvertMasterEditionV1ToV2Keys
{
    fn from(pubkeys: [Pubkey; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            master_edition: pubkeys[0],
            one_time_auth: pubkeys[1],
            printing_mint: pubkeys[2],
        }
    }
}
impl<'info> From<&ConvertMasterEditionV1ToV2Accounts<'_, 'info>>
    for [AccountInfo<'info>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'info>) -> Self {
        [
            accounts.master_edition.clone(),
            accounts.one_time_auth.clone(),
            accounts.printing_mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN]>
    for ConvertMasterEditionV1ToV2Accounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            master_edition: &arr[0],
            one_time_auth: &arr[1],
            printing_mint: &arr[2],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ConvertMasterEditionV1ToV2IxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ConvertMasterEditionV1ToV2IxData(pub ConvertMasterEditionV1ToV2IxArgs);
pub const CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM: u8 = 12u8;
impl From<ConvertMasterEditionV1ToV2IxArgs> for ConvertMasterEditionV1ToV2IxData {
    fn from(args: ConvertMasterEditionV1ToV2IxArgs) -> Self {
        Self(args)
    }
}
impl ConvertMasterEditionV1ToV2IxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ConvertMasterEditionV1ToV2IxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CONVERT_MASTER_EDITION_V1_TO_V2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: ConvertMasterEditionV1ToV2IxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn convert_master_edition_v1_to_v2_invoke<'info, A: Into<ConvertMasterEditionV1ToV2IxArgs>>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = convert_master_edition_v1_to_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn convert_master_edition_v1_to_v2_invoke_signed<
    'info,
    A: Into<ConvertMasterEditionV1ToV2IxArgs>,
>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = convert_master_edition_v1_to_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CONVERT_MASTER_EDITION_V1_TO_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn convert_master_edition_v1_to_v2_verify_account_keys(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'_, '_>,
    keys: &ConvertMasterEditionV1ToV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.one_time_auth.key, &keys.one_time_auth),
        (accounts.printing_mint.key, &keys.printing_mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn convert_master_edition_v1_to_v2_verify_account_privileges<'me, 'info>(
    accounts: &ConvertMasterEditionV1ToV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.master_edition,
        accounts.one_time_auth,
        accounts.printing_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN: usize = 17;
#[derive(Copy, Clone, Debug)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyAccounts<'me, 'info> {
    ///New Metadata key (pda of ['metadata', program id, mint id])
    pub new_metadata: &'me AccountInfo<'info>,
    ///New Edition (pda of ['metadata', program id, mint id, 'edition'])
    pub new_edition: &'me AccountInfo<'info>,
    ///Master Record Edition V2 (pda of ['metadata', program id, master metadata mint id, 'edition']
    pub master_edition: &'me AccountInfo<'info>,
    ///Mint of new token - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub new_mint: &'me AccountInfo<'info>,
    ///Edition pda to mark creation - will be checked for pre-existence. (pda of ['metadata', program id, master metadata mint id, 'edition', edition_number]) where edition_number is NOT the edition number you pass in args but actually edition_number = floor(edition/EDITION_MARKER_BIT_SIZE).
    pub edition_mark_pda: &'me AccountInfo<'info>,
    ///Mint authority of new mint
    pub new_mint_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///Vault authority
    pub vault_authority: &'me AccountInfo<'info>,
    ///Safety deposit token store account
    pub safety_deposit_store: &'me AccountInfo<'info>,
    ///Safety deposit box
    pub safety_deposit_box: &'me AccountInfo<'info>,
    ///Vault
    pub vault: &'me AccountInfo<'info>,
    ///Update authority info for new metadata
    pub new_metadata_update_authority: &'me AccountInfo<'info>,
    ///Master record metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///Token vault program
    pub token_vault_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&MintNewEditionFromMasterEditionViaVaultProxyAccounts<'_, '_>>
    for MintNewEditionFromMasterEditionViaVaultProxyKeys
{
    fn from(accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts) -> Self {
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
impl From<[Pubkey; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN]>
    for MintNewEditionFromMasterEditionViaVaultProxyKeys
{
    fn from(
        pubkeys: [Pubkey; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_metadata: pubkeys[0],
            new_edition: pubkeys[1],
            master_edition: pubkeys[2],
            new_mint: pubkeys[3],
            edition_mark_pda: pubkeys[4],
            new_mint_authority: pubkeys[5],
            payer: pubkeys[6],
            vault_authority: pubkeys[7],
            safety_deposit_store: pubkeys[8],
            safety_deposit_box: pubkeys[9],
            vault: pubkeys[10],
            new_metadata_update_authority: pubkeys[11],
            metadata: pubkeys[12],
            token_program: pubkeys[13],
            token_vault_program: pubkeys[14],
            system_program: pubkeys[15],
            rent: pubkeys[16],
        }
    }
}
impl<'info> From<&MintNewEditionFromMasterEditionViaVaultProxyAccounts<'_, 'info>>
    for [AccountInfo<'info>; MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info>
    From<
        &'me [AccountInfo<'info>;
                 MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN],
    > for MintNewEditionFromMasterEditionViaVaultProxyAccounts<'me, 'info>
{
    fn from(
        arr: &'me [AccountInfo<'info>;
                 MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            new_metadata: &arr[0],
            new_edition: &arr[1],
            master_edition: &arr[2],
            new_mint: &arr[3],
            edition_mark_pda: &arr[4],
            new_mint_authority: &arr[5],
            payer: &arr[6],
            vault_authority: &arr[7],
            safety_deposit_store: &arr[8],
            safety_deposit_box: &arr[9],
            vault: &arr[10],
            new_metadata_update_authority: &arr[11],
            metadata: &arr[12],
            token_program: &arr[13],
            token_vault_program: &arr[14],
            system_program: &arr[15],
            rent: &arr[16],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MintNewEditionFromMasterEditionViaVaultProxyIxArgs {
    pub mint_new_edition_from_master_edition_via_token_args:
        MintNewEditionFromMasterEditionViaTokenArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct MintNewEditionFromMasterEditionViaVaultProxyIxData(
    pub MintNewEditionFromMasterEditionViaVaultProxyIxArgs,
);
pub const MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM: u8 = 13u8;
impl From<MintNewEditionFromMasterEditionViaVaultProxyIxArgs>
    for MintNewEditionFromMasterEditionViaVaultProxyIxData
{
    fn from(args: MintNewEditionFromMasterEditionViaVaultProxyIxArgs) -> Self {
        Self(args)
    }
}
impl MintNewEditionFromMasterEditionViaVaultProxyIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(
            MintNewEditionFromMasterEditionViaVaultProxyIxArgs::deserialize(buf)?,
        ))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: MintNewEditionFromMasterEditionViaVaultProxyIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_invoke<
    'info,
    A: Into<MintNewEditionFromMasterEditionViaVaultProxyIxArgs>,
>(
    accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_vault_proxy_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_invoke_signed<
    'info,
    A: Into<MintNewEditionFromMasterEditionViaVaultProxyIxArgs>,
>(
    accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = mint_new_edition_from_master_edition_via_vault_proxy_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>;
        MINT_NEW_EDITION_FROM_MASTER_EDITION_VIA_VAULT_PROXY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_verify_account_keys(
    accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<'_, '_>,
    keys: &MintNewEditionFromMasterEditionViaVaultProxyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.new_metadata.key, &keys.new_metadata),
        (accounts.new_edition.key, &keys.new_edition),
        (accounts.master_edition.key, &keys.master_edition),
        (accounts.new_mint.key, &keys.new_mint),
        (accounts.edition_mark_pda.key, &keys.edition_mark_pda),
        (accounts.new_mint_authority.key, &keys.new_mint_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.vault_authority.key, &keys.vault_authority),
        (
            accounts.safety_deposit_store.key,
            &keys.safety_deposit_store,
        ),
        (accounts.safety_deposit_box.key, &keys.safety_deposit_box),
        (accounts.vault.key, &keys.vault),
        (
            accounts.new_metadata_update_authority.key,
            &keys.new_metadata_update_authority,
        ),
        (accounts.metadata.key, &keys.metadata),
        (accounts.token_program.key, &keys.token_program),
        (accounts.token_vault_program.key, &keys.token_vault_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn mint_new_edition_from_master_edition_via_vault_proxy_verify_account_privileges<
    'me,
    'info,
>(
    accounts: &MintNewEditionFromMasterEditionViaVaultProxyAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.new_metadata,
        accounts.new_edition,
        accounts.master_edition,
        accounts.new_mint,
        accounts.edition_mark_pda,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.new_mint_authority,
        accounts.payer,
        accounts.vault_authority,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const PUFF_METADATA_IX_ACCOUNTS_LEN: usize = 1;
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataAccounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct PuffMetadataKeys {
    ///Metadata account
    pub metadata: Pubkey,
}
impl From<&PuffMetadataAccounts<'_, '_>> for PuffMetadataKeys {
    fn from(accounts: &PuffMetadataAccounts) -> Self {
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
impl From<[Pubkey; PUFF_METADATA_IX_ACCOUNTS_LEN]> for PuffMetadataKeys {
    fn from(pubkeys: [Pubkey; PUFF_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
        }
    }
}
impl<'info> From<&PuffMetadataAccounts<'_, 'info>>
    for [AccountInfo<'info>; PUFF_METADATA_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &PuffMetadataAccounts<'_, 'info>) -> Self {
        [accounts.metadata.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; PUFF_METADATA_IX_ACCOUNTS_LEN]>
    for PuffMetadataAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; PUFF_METADATA_IX_ACCOUNTS_LEN]) -> Self {
        Self { metadata: &arr[0] }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PuffMetadataIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct PuffMetadataIxData(pub PuffMetadataIxArgs);
pub const PUFF_METADATA_IX_DISCM: u8 = 14u8;
impl From<PuffMetadataIxArgs> for PuffMetadataIxData {
    fn from(args: PuffMetadataIxArgs) -> Self {
        Self(args)
    }
}
impl PuffMetadataIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != PUFF_METADATA_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    PUFF_METADATA_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(PuffMetadataIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[PUFF_METADATA_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn puff_metadata_ix<K: Into<PuffMetadataKeys>, A: Into<PuffMetadataIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: PuffMetadataKeys = accounts.into();
    let metas: [AccountMeta; PUFF_METADATA_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: PuffMetadataIxArgs = args.into();
    let data: PuffMetadataIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn puff_metadata_invoke<'info, A: Into<PuffMetadataIxArgs>>(
    accounts: &PuffMetadataAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = puff_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PUFF_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn puff_metadata_invoke_signed<'info, A: Into<PuffMetadataIxArgs>>(
    accounts: &PuffMetadataAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = puff_metadata_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; PUFF_METADATA_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn puff_metadata_verify_account_keys(
    accounts: &PuffMetadataAccounts<'_, '_>,
    keys: &PuffMetadataKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [(accounts.metadata.key, &keys.metadata)] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn puff_metadata_verify_account_privileges<'me, 'info>(
    accounts: &PuffMetadataAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub const UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2Accounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Update authority key
    pub update_authority: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct UpdateMetadataAccountV2Keys {
    ///Metadata account
    pub metadata: Pubkey,
    ///Update authority key
    pub update_authority: Pubkey,
}
impl From<&UpdateMetadataAccountV2Accounts<'_, '_>> for UpdateMetadataAccountV2Keys {
    fn from(accounts: &UpdateMetadataAccountV2Accounts) -> Self {
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
impl From<[Pubkey; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]> for UpdateMetadataAccountV2Keys {
    fn from(pubkeys: [Pubkey; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            update_authority: pubkeys[1],
        }
    }
}
impl<'info> From<&UpdateMetadataAccountV2Accounts<'_, 'info>>
    for [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UpdateMetadataAccountV2Accounts<'_, 'info>) -> Self {
        [accounts.metadata.clone(), accounts.update_authority.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]>
    for UpdateMetadataAccountV2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            update_authority: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateMetadataAccountV2IxArgs {
    pub update_metadata_account_args_v2: UpdateMetadataAccountArgsV2,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateMetadataAccountV2IxData(pub UpdateMetadataAccountV2IxArgs);
pub const UPDATE_METADATA_ACCOUNT_V2_IX_DISCM: u8 = 15u8;
impl From<UpdateMetadataAccountV2IxArgs> for UpdateMetadataAccountV2IxData {
    fn from(args: UpdateMetadataAccountV2IxArgs) -> Self {
        Self(args)
    }
}
impl UpdateMetadataAccountV2IxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != UPDATE_METADATA_ACCOUNT_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UPDATE_METADATA_ACCOUNT_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UpdateMetadataAccountV2IxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UPDATE_METADATA_ACCOUNT_V2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: UpdateMetadataAccountV2IxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_metadata_account_v2_invoke<'info, A: Into<UpdateMetadataAccountV2IxArgs>>(
    accounts: &UpdateMetadataAccountV2Accounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = update_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn update_metadata_account_v2_invoke_signed<'info, A: Into<UpdateMetadataAccountV2IxArgs>>(
    accounts: &UpdateMetadataAccountV2Accounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = update_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UPDATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn update_metadata_account_v2_verify_account_keys(
    accounts: &UpdateMetadataAccountV2Accounts<'_, '_>,
    keys: &UpdateMetadataAccountV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.update_authority.key, &keys.update_authority),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn update_metadata_account_v2_verify_account_privileges<'me, 'info>(
    accounts: &UpdateMetadataAccountV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.update_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN: usize = 7;
#[derive(Copy, Clone, Debug)]
pub struct CreateMetadataAccountV2Accounts<'me, 'info> {
    ///Metadata key (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Mint of token asset
    pub mint: &'me AccountInfo<'info>,
    ///Mint authority
    pub mint_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///update authority info
    pub update_authority: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&CreateMetadataAccountV2Accounts<'_, '_>> for CreateMetadataAccountV2Keys {
    fn from(accounts: &CreateMetadataAccountV2Accounts) -> Self {
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
impl From<[Pubkey; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]> for CreateMetadataAccountV2Keys {
    fn from(pubkeys: [Pubkey; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            mint: pubkeys[1],
            mint_authority: pubkeys[2],
            payer: pubkeys[3],
            update_authority: pubkeys[4],
            system_program: pubkeys[5],
            rent: pubkeys[6],
        }
    }
}
impl<'info> From<&CreateMetadataAccountV2Accounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreateMetadataAccountV2Accounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]>
    for CreateMetadataAccountV2Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            mint: &arr[1],
            mint_authority: &arr[2],
            payer: &arr[3],
            update_authority: &arr[4],
            system_program: &arr[5],
            rent: &arr[6],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMetadataAccountV2IxArgs {
    pub create_metadata_account_args_v2: CreateMetadataAccountArgsV2,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMetadataAccountV2IxData(pub CreateMetadataAccountV2IxArgs);
pub const CREATE_METADATA_ACCOUNT_V2_IX_DISCM: u8 = 16u8;
impl From<CreateMetadataAccountV2IxArgs> for CreateMetadataAccountV2IxData {
    fn from(args: CreateMetadataAccountV2IxArgs) -> Self {
        Self(args)
    }
}
impl CreateMetadataAccountV2IxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_METADATA_ACCOUNT_V2_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_METADATA_ACCOUNT_V2_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateMetadataAccountV2IxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_METADATA_ACCOUNT_V2_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: CreateMetadataAccountV2IxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_metadata_account_v2_invoke<'info, A: Into<CreateMetadataAccountV2IxArgs>>(
    accounts: &CreateMetadataAccountV2Accounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_metadata_account_v2_invoke_signed<'info, A: Into<CreateMetadataAccountV2IxArgs>>(
    accounts: &CreateMetadataAccountV2Accounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_metadata_account_v2_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_METADATA_ACCOUNT_V2_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_metadata_account_v2_verify_account_keys(
    accounts: &CreateMetadataAccountV2Accounts<'_, '_>,
    keys: &CreateMetadataAccountV2Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.mint.key, &keys.mint),
        (accounts.mint_authority.key, &keys.mint_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_metadata_account_v2_verify_account_privileges<'me, 'info>(
    accounts: &CreateMetadataAccountV2Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.mint_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct CreateMasterEditionV3Accounts<'me, 'info> {
    ///Unallocated edition V2 account with address as pda of ['metadata', program id, mint, 'edition']
    pub edition: &'me AccountInfo<'info>,
    ///Metadata mint
    pub mint: &'me AccountInfo<'info>,
    ///Update authority
    pub update_authority: &'me AccountInfo<'info>,
    ///Mint authority on the metadata's mint - THIS WILL TRANSFER AUTHORITY AWAY FROM THIS KEY
    pub mint_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&CreateMasterEditionV3Accounts<'_, '_>> for CreateMasterEditionV3Keys {
    fn from(accounts: &CreateMasterEditionV3Accounts) -> Self {
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
impl From<[Pubkey; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN]> for CreateMasterEditionV3Keys {
    fn from(pubkeys: [Pubkey; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            edition: pubkeys[0],
            mint: pubkeys[1],
            update_authority: pubkeys[2],
            mint_authority: pubkeys[3],
            payer: pubkeys[4],
            metadata: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<&CreateMasterEditionV3Accounts<'_, 'info>>
    for [AccountInfo<'info>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &CreateMasterEditionV3Accounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN]>
    for CreateMasterEditionV3Accounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            edition: &arr[0],
            mint: &arr[1],
            update_authority: &arr[2],
            mint_authority: &arr[3],
            payer: &arr[4],
            metadata: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateMasterEditionV3IxArgs {
    pub create_master_edition_args: CreateMasterEditionArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CreateMasterEditionV3IxData(pub CreateMasterEditionV3IxArgs);
pub const CREATE_MASTER_EDITION_V3_IX_DISCM: u8 = 17u8;
impl From<CreateMasterEditionV3IxArgs> for CreateMasterEditionV3IxData {
    fn from(args: CreateMasterEditionV3IxArgs) -> Self {
        Self(args)
    }
}
impl CreateMasterEditionV3IxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != CREATE_MASTER_EDITION_V3_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    CREATE_MASTER_EDITION_V3_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(CreateMasterEditionV3IxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[CREATE_MASTER_EDITION_V3_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: CreateMasterEditionV3IxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn create_master_edition_v3_invoke<'info, A: Into<CreateMasterEditionV3IxArgs>>(
    accounts: &CreateMasterEditionV3Accounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = create_master_edition_v3_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn create_master_edition_v3_invoke_signed<'info, A: Into<CreateMasterEditionV3IxArgs>>(
    accounts: &CreateMasterEditionV3Accounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = create_master_edition_v3_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; CREATE_MASTER_EDITION_V3_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn create_master_edition_v3_verify_account_keys(
    accounts: &CreateMasterEditionV3Accounts<'_, '_>,
    keys: &CreateMasterEditionV3Keys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.edition.key, &keys.edition),
        (accounts.mint.key, &keys.mint),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.mint_authority.key, &keys.mint_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.metadata.key, &keys.metadata),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn create_master_edition_v3_verify_account_privileges<'me, 'info>(
    accounts: &CreateMasterEditionV3Accounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.edition, accounts.mint, accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [
        accounts.update_authority,
        accounts.mint_authority,
        accounts.payer,
    ] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const VERIFY_COLLECTION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct VerifyCollectionAccounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Collection Update authority
    pub collection_authority: &'me AccountInfo<'info>,
    ///payer
    pub payer: &'me AccountInfo<'info>,
    ///Mint of the Collection
    pub collection_mint: &'me AccountInfo<'info>,
    ///Metadata Account of the Collection
    pub collection: &'me AccountInfo<'info>,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'me AccountInfo<'info>,
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
impl From<&VerifyCollectionAccounts<'_, '_>> for VerifyCollectionKeys {
    fn from(accounts: &VerifyCollectionAccounts) -> Self {
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
impl From<[Pubkey; VERIFY_COLLECTION_IX_ACCOUNTS_LEN]> for VerifyCollectionKeys {
    fn from(pubkeys: [Pubkey; VERIFY_COLLECTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            collection_authority: pubkeys[1],
            payer: pubkeys[2],
            collection_mint: pubkeys[3],
            collection: pubkeys[4],
            collection_master_edition_account: pubkeys[5],
        }
    }
}
impl<'info> From<&VerifyCollectionAccounts<'_, 'info>>
    for [AccountInfo<'info>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &VerifyCollectionAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN]>
    for VerifyCollectionAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            collection_authority: &arr[1],
            payer: &arr[2],
            collection_mint: &arr[3],
            collection: &arr[4],
            collection_master_edition_account: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct VerifyCollectionIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct VerifyCollectionIxData(pub VerifyCollectionIxArgs);
pub const VERIFY_COLLECTION_IX_DISCM: u8 = 18u8;
impl From<VerifyCollectionIxArgs> for VerifyCollectionIxData {
    fn from(args: VerifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl VerifyCollectionIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != VERIFY_COLLECTION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    VERIFY_COLLECTION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(VerifyCollectionIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[VERIFY_COLLECTION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn verify_collection_ix<K: Into<VerifyCollectionKeys>, A: Into<VerifyCollectionIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: VerifyCollectionKeys = accounts.into();
    let metas: [AccountMeta; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: VerifyCollectionIxArgs = args.into();
    let data: VerifyCollectionIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn verify_collection_invoke<'info, A: Into<VerifyCollectionIxArgs>>(
    accounts: &VerifyCollectionAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn verify_collection_invoke_signed<'info, A: Into<VerifyCollectionIxArgs>>(
    accounts: &VerifyCollectionAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; VERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn verify_collection_verify_account_keys(
    accounts: &VerifyCollectionAccounts<'_, '_>,
    keys: &VerifyCollectionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (
            accounts.collection_authority.key,
            &keys.collection_authority,
        ),
        (accounts.payer.key, &keys.payer),
        (accounts.collection_mint.key, &keys.collection_mint),
        (accounts.collection.key, &keys.collection),
        (
            accounts.collection_master_edition_account.key,
            &keys.collection_master_edition_account,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn verify_collection_verify_account_privileges<'me, 'info>(
    accounts: &VerifyCollectionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.collection_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UTILIZE_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct UtilizeAccounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Token Account Of NFT
    pub token_account: &'me AccountInfo<'info>,
    ///Mint of the Metadata
    pub mint: &'me AccountInfo<'info>,
    ///A Use Authority / Can be the current Owner of the NFT
    pub use_authority: &'me AccountInfo<'info>,
    ///Owner
    pub owner: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///Associated Token program
    pub ata_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
    ///Use Authority Record PDA If present the program Assumes a delegated use authority
    pub use_authority_record: &'me AccountInfo<'info>,
    ///Program As Signer (Burner)
    pub burner: &'me AccountInfo<'info>,
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
impl From<&UtilizeAccounts<'_, '_>> for UtilizeKeys {
    fn from(accounts: &UtilizeAccounts) -> Self {
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
impl From<[Pubkey; UTILIZE_IX_ACCOUNTS_LEN]> for UtilizeKeys {
    fn from(pubkeys: [Pubkey; UTILIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            token_account: pubkeys[1],
            mint: pubkeys[2],
            use_authority: pubkeys[3],
            owner: pubkeys[4],
            token_program: pubkeys[5],
            ata_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
            use_authority_record: pubkeys[9],
            burner: pubkeys[10],
        }
    }
}
impl<'info> From<&UtilizeAccounts<'_, 'info>> for [AccountInfo<'info>; UTILIZE_IX_ACCOUNTS_LEN] {
    fn from(accounts: &UtilizeAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; UTILIZE_IX_ACCOUNTS_LEN]>
    for UtilizeAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UTILIZE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            token_account: &arr[1],
            mint: &arr[2],
            use_authority: &arr[3],
            owner: &arr[4],
            token_program: &arr[5],
            ata_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
            use_authority_record: &arr[9],
            burner: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UtilizeIxArgs {
    pub utilize_args: UtilizeArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UtilizeIxData(pub UtilizeIxArgs);
pub const UTILIZE_IX_DISCM: u8 = 19u8;
impl From<UtilizeIxArgs> for UtilizeIxData {
    fn from(args: UtilizeIxArgs) -> Self {
        Self(args)
    }
}
impl UtilizeIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != UTILIZE_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UTILIZE_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UtilizeIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UTILIZE_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn utilize_ix<K: Into<UtilizeKeys>, A: Into<UtilizeIxArgs>>(
    accounts: K,
    args: A,
) -> std::io::Result<Instruction> {
    let keys: UtilizeKeys = accounts.into();
    let metas: [AccountMeta; UTILIZE_IX_ACCOUNTS_LEN] = (&keys).into();
    let args_full: UtilizeIxArgs = args.into();
    let data: UtilizeIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn utilize_invoke<'info, A: Into<UtilizeIxArgs>>(
    accounts: &UtilizeAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = utilize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UTILIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn utilize_invoke_signed<'info, A: Into<UtilizeIxArgs>>(
    accounts: &UtilizeAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = utilize_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UTILIZE_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn utilize_verify_account_keys(
    accounts: &UtilizeAccounts<'_, '_>,
    keys: &UtilizeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.token_account.key, &keys.token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.use_authority.key, &keys.use_authority),
        (accounts.owner.key, &keys.owner),
        (accounts.token_program.key, &keys.token_program),
        (accounts.ata_program.key, &keys.ata_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
        (
            accounts.use_authority_record.key,
            &keys.use_authority_record,
        ),
        (accounts.burner.key, &keys.burner),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn utilize_verify_account_privileges<'me, 'info>(
    accounts: &UtilizeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.metadata,
        accounts.token_account,
        accounts.mint,
        accounts.use_authority_record,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.use_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 11;
#[derive(Copy, Clone, Debug)]
pub struct ApproveUseAuthorityAccounts<'me, 'info> {
    ///Use Authority Record PDA
    pub use_authority_record: &'me AccountInfo<'info>,
    ///Owner
    pub owner: &'me AccountInfo<'info>,
    ///Payer
    pub payer: &'me AccountInfo<'info>,
    ///A Use Authority
    pub user: &'me AccountInfo<'info>,
    ///Owned Token Account Of Mint
    pub owner_token_account: &'me AccountInfo<'info>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Mint of Metadata
    pub mint: &'me AccountInfo<'info>,
    ///Program As Signer (Burner)
    pub burner: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&ApproveUseAuthorityAccounts<'_, '_>> for ApproveUseAuthorityKeys {
    fn from(accounts: &ApproveUseAuthorityAccounts) -> Self {
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
impl From<[Pubkey; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN]> for ApproveUseAuthorityKeys {
    fn from(pubkeys: [Pubkey; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            use_authority_record: pubkeys[0],
            owner: pubkeys[1],
            payer: pubkeys[2],
            user: pubkeys[3],
            owner_token_account: pubkeys[4],
            metadata: pubkeys[5],
            mint: pubkeys[6],
            burner: pubkeys[7],
            token_program: pubkeys[8],
            system_program: pubkeys[9],
            rent: pubkeys[10],
        }
    }
}
impl<'info> From<&ApproveUseAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ApproveUseAuthorityAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for ApproveUseAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            use_authority_record: &arr[0],
            owner: &arr[1],
            payer: &arr[2],
            user: &arr[3],
            owner_token_account: &arr[4],
            metadata: &arr[5],
            mint: &arr[6],
            burner: &arr[7],
            token_program: &arr[8],
            system_program: &arr[9],
            rent: &arr[10],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApproveUseAuthorityIxArgs {
    pub approve_use_authority_args: ApproveUseAuthorityArgs,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveUseAuthorityIxData(pub ApproveUseAuthorityIxArgs);
pub const APPROVE_USE_AUTHORITY_IX_DISCM: u8 = 20u8;
impl From<ApproveUseAuthorityIxArgs> for ApproveUseAuthorityIxData {
    fn from(args: ApproveUseAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl ApproveUseAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != APPROVE_USE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    APPROVE_USE_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ApproveUseAuthorityIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[APPROVE_USE_AUTHORITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: ApproveUseAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_use_authority_invoke<'info, A: Into<ApproveUseAuthorityIxArgs>>(
    accounts: &ApproveUseAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = approve_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_use_authority_invoke_signed<'info, A: Into<ApproveUseAuthorityIxArgs>>(
    accounts: &ApproveUseAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = approve_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn approve_use_authority_verify_account_keys(
    accounts: &ApproveUseAuthorityAccounts<'_, '_>,
    keys: &ApproveUseAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.use_authority_record.key,
            &keys.use_authority_record,
        ),
        (accounts.owner.key, &keys.owner),
        (accounts.payer.key, &keys.payer),
        (accounts.user.key, &keys.user),
        (accounts.owner_token_account.key, &keys.owner_token_account),
        (accounts.metadata.key, &keys.metadata),
        (accounts.mint.key, &keys.mint),
        (accounts.burner.key, &keys.burner),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn approve_use_authority_verify_account_privileges<'me, 'info>(
    accounts: &ApproveUseAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.use_authority_record, accounts.owner_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.owner, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct RevokeUseAuthorityAccounts<'me, 'info> {
    ///Use Authority Record PDA
    pub use_authority_record: &'me AccountInfo<'info>,
    ///Owner
    pub owner: &'me AccountInfo<'info>,
    ///A Use Authority
    pub user: &'me AccountInfo<'info>,
    ///Owned Token Account Of Mint
    pub owner_token_account: &'me AccountInfo<'info>,
    ///Mint of Metadata
    pub mint: &'me AccountInfo<'info>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Token program
    pub token_program: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&RevokeUseAuthorityAccounts<'_, '_>> for RevokeUseAuthorityKeys {
    fn from(accounts: &RevokeUseAuthorityAccounts) -> Self {
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
impl From<[Pubkey; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN]> for RevokeUseAuthorityKeys {
    fn from(pubkeys: [Pubkey; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            use_authority_record: pubkeys[0],
            owner: pubkeys[1],
            user: pubkeys[2],
            owner_token_account: pubkeys[3],
            mint: pubkeys[4],
            metadata: pubkeys[5],
            token_program: pubkeys[6],
            system_program: pubkeys[7],
            rent: pubkeys[8],
        }
    }
}
impl<'info> From<&RevokeUseAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RevokeUseAuthorityAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN]>
    for RevokeUseAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            use_authority_record: &arr[0],
            owner: &arr[1],
            user: &arr[2],
            owner_token_account: &arr[3],
            mint: &arr[4],
            metadata: &arr[5],
            token_program: &arr[6],
            system_program: &arr[7],
            rent: &arr[8],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RevokeUseAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeUseAuthorityIxData(pub RevokeUseAuthorityIxArgs);
pub const REVOKE_USE_AUTHORITY_IX_DISCM: u8 = 21u8;
impl From<RevokeUseAuthorityIxArgs> for RevokeUseAuthorityIxData {
    fn from(args: RevokeUseAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl RevokeUseAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REVOKE_USE_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REVOKE_USE_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RevokeUseAuthorityIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REVOKE_USE_AUTHORITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: RevokeUseAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_use_authority_invoke<'info, A: Into<RevokeUseAuthorityIxArgs>>(
    accounts: &RevokeUseAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = revoke_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_use_authority_invoke_signed<'info, A: Into<RevokeUseAuthorityIxArgs>>(
    accounts: &RevokeUseAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revoke_use_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REVOKE_USE_AUTHORITY_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn revoke_use_authority_verify_account_keys(
    accounts: &RevokeUseAuthorityAccounts<'_, '_>,
    keys: &RevokeUseAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.use_authority_record.key,
            &keys.use_authority_record,
        ),
        (accounts.owner.key, &keys.owner),
        (accounts.user.key, &keys.user),
        (accounts.owner_token_account.key, &keys.owner_token_account),
        (accounts.mint.key, &keys.mint),
        (accounts.metadata.key, &keys.metadata),
        (accounts.token_program.key, &keys.token_program),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn revoke_use_authority_verify_account_privileges<'me, 'info>(
    accounts: &RevokeUseAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.use_authority_record, accounts.owner_token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN: usize = 6;
#[derive(Copy, Clone, Debug)]
pub struct UnverifyCollectionAccounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Collection Authority
    pub collection_authority: &'me AccountInfo<'info>,
    ///Mint of the Collection
    pub collection_mint: &'me AccountInfo<'info>,
    ///Metadata Account of the Collection
    pub collection: &'me AccountInfo<'info>,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'me AccountInfo<'info>,
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'info>,
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
impl From<&UnverifyCollectionAccounts<'_, '_>> for UnverifyCollectionKeys {
    fn from(accounts: &UnverifyCollectionAccounts) -> Self {
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
impl From<[Pubkey; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN]> for UnverifyCollectionKeys {
    fn from(pubkeys: [Pubkey; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            collection_authority: pubkeys[1],
            collection_mint: pubkeys[2],
            collection: pubkeys[3],
            collection_master_edition_account: pubkeys[4],
            collection_authority_record: pubkeys[5],
        }
    }
}
impl<'info> From<&UnverifyCollectionAccounts<'_, 'info>>
    for [AccountInfo<'info>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &UnverifyCollectionAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN]>
    for UnverifyCollectionAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            collection_authority: &arr[1],
            collection_mint: &arr[2],
            collection: &arr[3],
            collection_master_edition_account: &arr[4],
            collection_authority_record: &arr[5],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UnverifyCollectionIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct UnverifyCollectionIxData(pub UnverifyCollectionIxArgs);
pub const UNVERIFY_COLLECTION_IX_DISCM: u8 = 22u8;
impl From<UnverifyCollectionIxArgs> for UnverifyCollectionIxData {
    fn from(args: UnverifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl UnverifyCollectionIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != UNVERIFY_COLLECTION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    UNVERIFY_COLLECTION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(UnverifyCollectionIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[UNVERIFY_COLLECTION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: UnverifyCollectionIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn unverify_collection_invoke<'info, A: Into<UnverifyCollectionIxArgs>>(
    accounts: &UnverifyCollectionAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = unverify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke(&ix, &account_info)
}
pub fn unverify_collection_invoke_signed<'info, A: Into<UnverifyCollectionIxArgs>>(
    accounts: &UnverifyCollectionAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = unverify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; UNVERIFY_COLLECTION_IX_ACCOUNTS_LEN] = accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn unverify_collection_verify_account_keys(
    accounts: &UnverifyCollectionAccounts<'_, '_>,
    keys: &UnverifyCollectionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (
            accounts.collection_authority.key,
            &keys.collection_authority,
        ),
        (accounts.collection_mint.key, &keys.collection_mint),
        (accounts.collection.key, &keys.collection),
        (
            accounts.collection_master_edition_account.key,
            &keys.collection_master_edition_account,
        ),
        (
            accounts.collection_authority_record.key,
            &keys.collection_authority_record,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn unverify_collection_verify_account_privileges<'me, 'info>(
    accounts: &UnverifyCollectionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.collection_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct ApproveCollectionAuthorityAccounts<'me, 'info> {
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'info>,
    ///A Collection Authority
    pub new_collection_authority: &'me AccountInfo<'info>,
    ///Update Authority of Collection NFT
    pub update_authority: &'me AccountInfo<'info>,
    ///Payer
    pub payer: &'me AccountInfo<'info>,
    ///Collection Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Mint of Collection Metadata
    pub mint: &'me AccountInfo<'info>,
    ///System program
    pub system_program: &'me AccountInfo<'info>,
    ///Rent info
    pub rent: &'me AccountInfo<'info>,
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
impl From<&ApproveCollectionAuthorityAccounts<'_, '_>> for ApproveCollectionAuthorityKeys {
    fn from(accounts: &ApproveCollectionAuthorityAccounts) -> Self {
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
impl From<[Pubkey; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]>
    for ApproveCollectionAuthorityKeys
{
    fn from(pubkeys: [Pubkey; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collection_authority_record: pubkeys[0],
            new_collection_authority: pubkeys[1],
            update_authority: pubkeys[2],
            payer: pubkeys[3],
            metadata: pubkeys[4],
            mint: pubkeys[5],
            system_program: pubkeys[6],
            rent: pubkeys[7],
        }
    }
}
impl<'info> From<&ApproveCollectionAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ApproveCollectionAuthorityAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]>
    for ApproveCollectionAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collection_authority_record: &arr[0],
            new_collection_authority: &arr[1],
            update_authority: &arr[2],
            payer: &arr[3],
            metadata: &arr[4],
            mint: &arr[5],
            system_program: &arr[6],
            rent: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ApproveCollectionAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ApproveCollectionAuthorityIxData(pub ApproveCollectionAuthorityIxArgs);
pub const APPROVE_COLLECTION_AUTHORITY_IX_DISCM: u8 = 23u8;
impl From<ApproveCollectionAuthorityIxArgs> for ApproveCollectionAuthorityIxData {
    fn from(args: ApproveCollectionAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl ApproveCollectionAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != APPROVE_COLLECTION_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    APPROVE_COLLECTION_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ApproveCollectionAuthorityIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[APPROVE_COLLECTION_AUTHORITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: ApproveCollectionAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn approve_collection_authority_invoke<'info, A: Into<ApproveCollectionAuthorityIxArgs>>(
    accounts: &ApproveCollectionAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = approve_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn approve_collection_authority_invoke_signed<
    'info,
    A: Into<ApproveCollectionAuthorityIxArgs>,
>(
    accounts: &ApproveCollectionAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = approve_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; APPROVE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn approve_collection_authority_verify_account_keys(
    accounts: &ApproveCollectionAuthorityAccounts<'_, '_>,
    keys: &ApproveCollectionAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.collection_authority_record.key,
            &keys.collection_authority_record,
        ),
        (
            accounts.new_collection_authority.key,
            &keys.new_collection_authority,
        ),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.payer.key, &keys.payer),
        (accounts.metadata.key, &keys.metadata),
        (accounts.mint.key, &keys.mint),
        (accounts.system_program.key, &keys.system_program),
        (accounts.rent.key, &keys.rent),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn approve_collection_authority_verify_account_privileges<'me, 'info>(
    accounts: &ApproveCollectionAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.collection_authority_record] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.update_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct RevokeCollectionAuthorityAccounts<'me, 'info> {
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'info>,
    ///Update Authority of Collection NFT
    pub update_authority: &'me AccountInfo<'info>,
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Mint of Metadata
    pub mint: &'me AccountInfo<'info>,
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
impl From<&RevokeCollectionAuthorityAccounts<'_, '_>> for RevokeCollectionAuthorityKeys {
    fn from(accounts: &RevokeCollectionAuthorityAccounts) -> Self {
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
impl From<[Pubkey; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]> for RevokeCollectionAuthorityKeys {
    fn from(pubkeys: [Pubkey; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collection_authority_record: pubkeys[0],
            update_authority: pubkeys[1],
            metadata: pubkeys[2],
            mint: pubkeys[3],
        }
    }
}
impl<'info> From<&RevokeCollectionAuthorityAccounts<'_, 'info>>
    for [AccountInfo<'info>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RevokeCollectionAuthorityAccounts<'_, 'info>) -> Self {
        [
            accounts.collection_authority_record.clone(),
            accounts.update_authority.clone(),
            accounts.metadata.clone(),
            accounts.mint.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]>
    for RevokeCollectionAuthorityAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            collection_authority_record: &arr[0],
            update_authority: &arr[1],
            metadata: &arr[2],
            mint: &arr[3],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RevokeCollectionAuthorityIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RevokeCollectionAuthorityIxData(pub RevokeCollectionAuthorityIxArgs);
pub const REVOKE_COLLECTION_AUTHORITY_IX_DISCM: u8 = 24u8;
impl From<RevokeCollectionAuthorityIxArgs> for RevokeCollectionAuthorityIxData {
    fn from(args: RevokeCollectionAuthorityIxArgs) -> Self {
        Self(args)
    }
}
impl RevokeCollectionAuthorityIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REVOKE_COLLECTION_AUTHORITY_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REVOKE_COLLECTION_AUTHORITY_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RevokeCollectionAuthorityIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REVOKE_COLLECTION_AUTHORITY_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: RevokeCollectionAuthorityIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn revoke_collection_authority_invoke<'info, A: Into<RevokeCollectionAuthorityIxArgs>>(
    accounts: &RevokeCollectionAuthorityAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = revoke_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn revoke_collection_authority_invoke_signed<
    'info,
    A: Into<RevokeCollectionAuthorityIxArgs>,
>(
    accounts: &RevokeCollectionAuthorityAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = revoke_collection_authority_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REVOKE_COLLECTION_AUTHORITY_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn revoke_collection_authority_verify_account_keys(
    accounts: &RevokeCollectionAuthorityAccounts<'_, '_>,
    keys: &RevokeCollectionAuthorityKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (
            accounts.collection_authority_record.key,
            &keys.collection_authority_record,
        ),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.metadata.key, &keys.metadata),
        (accounts.mint.key, &keys.mint),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn revoke_collection_authority_verify_account_privileges<'me, 'info>(
    accounts: &RevokeCollectionAuthorityAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.collection_authority_record] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.update_authority] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN: usize = 8;
#[derive(Copy, Clone, Debug)]
pub struct SetAndVerifyCollectionAccounts<'me, 'info> {
    ///Metadata account
    pub metadata: &'me AccountInfo<'info>,
    ///Collection Update authority
    pub collection_authority: &'me AccountInfo<'info>,
    ///Payer
    pub payer: &'me AccountInfo<'info>,
    ///Update Authority of Collection NFT and NFT
    pub update_authority: &'me AccountInfo<'info>,
    ///Mint of the Collection
    pub collection_mint: &'me AccountInfo<'info>,
    ///Metadata Account of the Collection
    pub collection: &'me AccountInfo<'info>,
    ///MasterEdition2 Account of the Collection Token
    pub collection_master_edition_account: &'me AccountInfo<'info>,
    ///Collection Authority Record PDA
    pub collection_authority_record: &'me AccountInfo<'info>,
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
impl From<&SetAndVerifyCollectionAccounts<'_, '_>> for SetAndVerifyCollectionKeys {
    fn from(accounts: &SetAndVerifyCollectionAccounts) -> Self {
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
impl From<[Pubkey; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]> for SetAndVerifyCollectionKeys {
    fn from(pubkeys: [Pubkey; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            collection_authority: pubkeys[1],
            payer: pubkeys[2],
            update_authority: pubkeys[3],
            collection_mint: pubkeys[4],
            collection: pubkeys[5],
            collection_master_edition_account: pubkeys[6],
            collection_authority_record: pubkeys[7],
        }
    }
}
impl<'info> From<&SetAndVerifyCollectionAccounts<'_, 'info>>
    for [AccountInfo<'info>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &SetAndVerifyCollectionAccounts<'_, 'info>) -> Self {
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
impl<'me, 'info> From<&'me [AccountInfo<'info>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]>
    for SetAndVerifyCollectionAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            collection_authority: &arr[1],
            payer: &arr[2],
            update_authority: &arr[3],
            collection_mint: &arr[4],
            collection: &arr[5],
            collection_master_edition_account: &arr[6],
            collection_authority_record: &arr[7],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SetAndVerifyCollectionIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct SetAndVerifyCollectionIxData(pub SetAndVerifyCollectionIxArgs);
pub const SET_AND_VERIFY_COLLECTION_IX_DISCM: u8 = 25u8;
impl From<SetAndVerifyCollectionIxArgs> for SetAndVerifyCollectionIxData {
    fn from(args: SetAndVerifyCollectionIxArgs) -> Self {
        Self(args)
    }
}
impl SetAndVerifyCollectionIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != SET_AND_VERIFY_COLLECTION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    SET_AND_VERIFY_COLLECTION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(SetAndVerifyCollectionIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[SET_AND_VERIFY_COLLECTION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: SetAndVerifyCollectionIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn set_and_verify_collection_invoke<'info, A: Into<SetAndVerifyCollectionIxArgs>>(
    accounts: &SetAndVerifyCollectionAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = set_and_verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn set_and_verify_collection_invoke_signed<'info, A: Into<SetAndVerifyCollectionIxArgs>>(
    accounts: &SetAndVerifyCollectionAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = set_and_verify_collection_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; SET_AND_VERIFY_COLLECTION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn set_and_verify_collection_verify_account_keys(
    accounts: &SetAndVerifyCollectionAccounts<'_, '_>,
    keys: &SetAndVerifyCollectionKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (
            accounts.collection_authority.key,
            &keys.collection_authority,
        ),
        (accounts.payer.key, &keys.payer),
        (accounts.update_authority.key, &keys.update_authority),
        (accounts.collection_mint.key, &keys.collection_mint),
        (accounts.collection.key, &keys.collection),
        (
            accounts.collection_master_edition_account.key,
            &keys.collection_master_edition_account,
        ),
        (
            accounts.collection_authority_record.key,
            &keys.collection_authority_record,
        ),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn set_and_verify_collection_verify_account_privileges<'me, 'info>(
    accounts: &SetAndVerifyCollectionAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.collection_authority, accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct FreezeDelegatedAccountAccounts<'me, 'info> {
    ///Delegate
    pub delegate: &'me AccountInfo<'info>,
    ///Token account to freeze
    pub token_account: &'me AccountInfo<'info>,
    ///Edition
    pub edition: &'me AccountInfo<'info>,
    ///Token mint
    pub mint: &'me AccountInfo<'info>,
    ///Token Program
    pub token_program: &'me AccountInfo<'info>,
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
impl From<&FreezeDelegatedAccountAccounts<'_, '_>> for FreezeDelegatedAccountKeys {
    fn from(accounts: &FreezeDelegatedAccountAccounts) -> Self {
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
impl From<[Pubkey; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]> for FreezeDelegatedAccountKeys {
    fn from(pubkeys: [Pubkey; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate: pubkeys[0],
            token_account: pubkeys[1],
            edition: pubkeys[2],
            mint: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<&FreezeDelegatedAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &FreezeDelegatedAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.delegate.clone(),
            accounts.token_account.clone(),
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]>
    for FreezeDelegatedAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate: &arr[0],
            token_account: &arr[1],
            edition: &arr[2],
            mint: &arr[3],
            token_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FreezeDelegatedAccountIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct FreezeDelegatedAccountIxData(pub FreezeDelegatedAccountIxArgs);
pub const FREEZE_DELEGATED_ACCOUNT_IX_DISCM: u8 = 26u8;
impl From<FreezeDelegatedAccountIxArgs> for FreezeDelegatedAccountIxData {
    fn from(args: FreezeDelegatedAccountIxArgs) -> Self {
        Self(args)
    }
}
impl FreezeDelegatedAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != FREEZE_DELEGATED_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    FREEZE_DELEGATED_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(FreezeDelegatedAccountIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[FREEZE_DELEGATED_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: FreezeDelegatedAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn freeze_delegated_account_invoke<'info, A: Into<FreezeDelegatedAccountIxArgs>>(
    accounts: &FreezeDelegatedAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = freeze_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn freeze_delegated_account_invoke_signed<'info, A: Into<FreezeDelegatedAccountIxArgs>>(
    accounts: &FreezeDelegatedAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = freeze_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; FREEZE_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn freeze_delegated_account_verify_account_keys(
    accounts: &FreezeDelegatedAccountAccounts<'_, '_>,
    keys: &FreezeDelegatedAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.delegate.key, &keys.delegate),
        (accounts.token_account.key, &keys.token_account),
        (accounts.edition.key, &keys.edition),
        (accounts.mint.key, &keys.mint),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn freeze_delegated_account_verify_account_privileges<'me, 'info>(
    accounts: &FreezeDelegatedAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.delegate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct ThawDelegatedAccountAccounts<'me, 'info> {
    ///Delegate
    pub delegate: &'me AccountInfo<'info>,
    ///Token account to thaw
    pub token_account: &'me AccountInfo<'info>,
    ///Edition
    pub edition: &'me AccountInfo<'info>,
    ///Token mint
    pub mint: &'me AccountInfo<'info>,
    ///Token Program
    pub token_program: &'me AccountInfo<'info>,
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
impl From<&ThawDelegatedAccountAccounts<'_, '_>> for ThawDelegatedAccountKeys {
    fn from(accounts: &ThawDelegatedAccountAccounts) -> Self {
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
impl From<[Pubkey; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]> for ThawDelegatedAccountKeys {
    fn from(pubkeys: [Pubkey; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate: pubkeys[0],
            token_account: pubkeys[1],
            edition: pubkeys[2],
            mint: pubkeys[3],
            token_program: pubkeys[4],
        }
    }
}
impl<'info> From<&ThawDelegatedAccountAccounts<'_, 'info>>
    for [AccountInfo<'info>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &ThawDelegatedAccountAccounts<'_, 'info>) -> Self {
        [
            accounts.delegate.clone(),
            accounts.token_account.clone(),
            accounts.edition.clone(),
            accounts.mint.clone(),
            accounts.token_program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]>
    for ThawDelegatedAccountAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            delegate: &arr[0],
            token_account: &arr[1],
            edition: &arr[2],
            mint: &arr[3],
            token_program: &arr[4],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ThawDelegatedAccountIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct ThawDelegatedAccountIxData(pub ThawDelegatedAccountIxArgs);
pub const THAW_DELEGATED_ACCOUNT_IX_DISCM: u8 = 27u8;
impl From<ThawDelegatedAccountIxArgs> for ThawDelegatedAccountIxData {
    fn from(args: ThawDelegatedAccountIxArgs) -> Self {
        Self(args)
    }
}
impl ThawDelegatedAccountIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != THAW_DELEGATED_ACCOUNT_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    THAW_DELEGATED_ACCOUNT_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(ThawDelegatedAccountIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[THAW_DELEGATED_ACCOUNT_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: ThawDelegatedAccountIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn thaw_delegated_account_invoke<'info, A: Into<ThawDelegatedAccountIxArgs>>(
    accounts: &ThawDelegatedAccountAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = thaw_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn thaw_delegated_account_invoke_signed<'info, A: Into<ThawDelegatedAccountIxArgs>>(
    accounts: &ThawDelegatedAccountAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = thaw_delegated_account_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; THAW_DELEGATED_ACCOUNT_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn thaw_delegated_account_verify_account_keys(
    accounts: &ThawDelegatedAccountAccounts<'_, '_>,
    keys: &ThawDelegatedAccountKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.delegate.key, &keys.delegate),
        (accounts.token_account.key, &keys.token_account),
        (accounts.edition.key, &keys.edition),
        (accounts.mint.key, &keys.mint),
        (accounts.token_program.key, &keys.token_program),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn thaw_delegated_account_verify_account_privileges<'me, 'info>(
    accounts: &ThawDelegatedAccountAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.token_account] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.delegate] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub const REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationAccounts<'me, 'info> {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: &'me AccountInfo<'info>,
    ///Creator
    pub creator: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug)]
pub struct RemoveCreatorVerificationKeys {
    ///Metadata (pda of ['metadata', program id, mint id])
    pub metadata: Pubkey,
    ///Creator
    pub creator: Pubkey,
}
impl From<&RemoveCreatorVerificationAccounts<'_, '_>> for RemoveCreatorVerificationKeys {
    fn from(accounts: &RemoveCreatorVerificationAccounts) -> Self {
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
impl From<[Pubkey; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]> for RemoveCreatorVerificationKeys {
    fn from(pubkeys: [Pubkey; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: pubkeys[0],
            creator: pubkeys[1],
        }
    }
}
impl<'info> From<&RemoveCreatorVerificationAccounts<'_, 'info>>
    for [AccountInfo<'info>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]
{
    fn from(accounts: &RemoveCreatorVerificationAccounts<'_, 'info>) -> Self {
        [accounts.metadata.clone(), accounts.creator.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]>
    for RemoveCreatorVerificationAccounts<'me, 'info>
{
    fn from(arr: &'me [AccountInfo<'info>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            metadata: &arr[0],
            creator: &arr[1],
        }
    }
}
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RemoveCreatorVerificationIxArgs {}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveCreatorVerificationIxData(pub RemoveCreatorVerificationIxArgs);
pub const REMOVE_CREATOR_VERIFICATION_IX_DISCM: u8 = 28u8;
impl From<RemoveCreatorVerificationIxArgs> for RemoveCreatorVerificationIxData {
    fn from(args: RemoveCreatorVerificationIxArgs) -> Self {
        Self(args)
    }
}
impl RemoveCreatorVerificationIxData {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = u8::deserialize(buf)?;
        if maybe_discm != REMOVE_CREATOR_VERIFICATION_IX_DISCM {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!(
                    "discm does not match. Expected: {:?}. Received: {:?}",
                    REMOVE_CREATOR_VERIFICATION_IX_DISCM, maybe_discm
                ),
            ));
        }
        Ok(Self(RemoveCreatorVerificationIxArgs::deserialize(buf)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&[REMOVE_CREATOR_VERIFICATION_IX_DISCM])?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
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
    let data: RemoveCreatorVerificationIxData = args_full.into();
    Ok(Instruction {
        program_id: crate::ID,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn remove_creator_verification_invoke<'info, A: Into<RemoveCreatorVerificationIxArgs>>(
    accounts: &RemoveCreatorVerificationAccounts<'_, 'info>,
    args: A,
) -> ProgramResult {
    let ix = remove_creator_verification_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke(&ix, &account_info)
}
pub fn remove_creator_verification_invoke_signed<
    'info,
    A: Into<RemoveCreatorVerificationIxArgs>,
>(
    accounts: &RemoveCreatorVerificationAccounts<'_, 'info>,
    args: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let ix = remove_creator_verification_ix(accounts, args)?;
    let account_info: [AccountInfo<'info>; REMOVE_CREATOR_VERIFICATION_IX_ACCOUNTS_LEN] =
        accounts.into();
    invoke_signed(&ix, &account_info, seeds)
}
pub fn remove_creator_verification_verify_account_keys(
    accounts: &RemoveCreatorVerificationAccounts<'_, '_>,
    keys: &RemoveCreatorVerificationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (accounts.metadata.key, &keys.metadata),
        (accounts.creator.key, &keys.creator),
    ] {
        if actual != expected {
            return Err((*actual, *expected));
        }
    }
    Ok(())
}
pub fn remove_creator_verification_verify_account_privileges<'me, 'info>(
    accounts: &RemoveCreatorVerificationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.metadata] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    for should_be_signer in [accounts.creator] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
