use mpl_token_metadata_interface::{
    CreateMasterEditionArgs, CreateMasterEditionV3IxArgs, CreateMasterEditionV3IxData,
    MplTokenMetadataProgramIx, CREATE_MASTER_EDITION_V3_IX_DISCM, REVOKE_USE_AUTHORITY_IX_DISCM,
};

#[test]
fn test_ix_data_borsh_roundtrip() {
    let sample = CreateMasterEditionV3IxData(CreateMasterEditionV3IxArgs {
        create_master_edition_args: CreateMasterEditionArgs {
            max_supply: Some(69),
        },
    });
    let serialized = sample.try_to_vec().unwrap();
    assert_eq!(serialized[0], CREATE_MASTER_EDITION_V3_IX_DISCM);
    let deserialized = CreateMasterEditionV3IxData::deserialize(&serialized).unwrap();
    assert_eq!(sample, deserialized);
}

#[test]
fn test_program_ix_borsh_roundtrip() {
    let program_ix = MplTokenMetadataProgramIx::RevokeUseAuthority;
    let serialized = program_ix.try_to_vec().unwrap();
    assert_eq!(serialized[0], REVOKE_USE_AUTHORITY_IX_DISCM);
    let deserialized = MplTokenMetadataProgramIx::deserialize(&serialized).unwrap();
    assert_eq!(program_ix, deserialized);
}
