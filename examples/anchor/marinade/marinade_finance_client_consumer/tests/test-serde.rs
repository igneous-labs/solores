use borsh::BorshSerialize;
use marinade_finance_interface::{
    ChangeAuthorityData, ChangeAuthorityIxArgs, ChangeAuthorityIxData, DepositIxArgs,
    MarinadeFinanceProgramIx,
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

#[test]
fn test_program_ix_borsh_roundtrip() {
    let program_ix = MarinadeFinanceProgramIx::Deposit(DepositIxArgs { lamports: 1000 });
    let serialized = program_ix.try_to_vec().unwrap();
    let deserialized = MarinadeFinanceProgramIx::deserialize(&mut serialized.as_ref()).unwrap();
    assert_eq!(program_ix, deserialized);
}
