use borsh::BorshSerialize;
use marinade_finance_interface::{
    ChangeAuthorityData, ChangeAuthorityIxArgs, ChangeAuthorityIxData,
};
use solana_sdk::pubkey::Pubkey;

#[test]
fn test_ix_data_borsh_roundtrip() {
    let sample = ChangeAuthorityIxData(ChangeAuthorityIxArgs {
        data: ChangeAuthorityData {
            admin: None,
            validator_manager: None,
            operational_sol_account: Some(Pubkey::new_unique()),
            treasury_msol_account: None,
        },
    });
    let serialized = sample.try_to_vec().unwrap();
    let deserialized = ChangeAuthorityIxData::deserialize(&mut serialized.as_ref()).unwrap();
    assert_eq!(sample, deserialized);
}
