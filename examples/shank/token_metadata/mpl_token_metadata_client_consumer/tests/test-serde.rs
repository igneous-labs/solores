use mpl_token_metadata_interface::{
    CreateMasterEditionArgs, CreateMasterEditionV3IxArgs, CreateMasterEditionV3IxData,
    MplTokenMetadataProgramIx, RevokeUseAuthorityIxArgs,
};

#[test]
fn test_ix_data_borsh_roundtrip() {
    let sample = CreateMasterEditionV3IxData(CreateMasterEditionV3IxArgs {
        create_master_edition_args: CreateMasterEditionArgs {
            max_supply: Some(69),
        },
    });
    let serialized = sample.try_to_vec().unwrap();
    let deserialized = CreateMasterEditionV3IxData::deserialize(&mut serialized.as_ref()).unwrap();
    assert_eq!(sample, deserialized);
}

#[test]
fn test_program_ix_borsh_roundtrip() {
    let program_ix = MplTokenMetadataProgramIx::RevokeUseAuthority(RevokeUseAuthorityIxArgs {});
    let serialized = program_ix.try_to_vec().unwrap();
    let deserialized = MplTokenMetadataProgramIx::deserialize(&mut serialized.as_ref()).unwrap();
    assert_eq!(program_ix, deserialized);
}
