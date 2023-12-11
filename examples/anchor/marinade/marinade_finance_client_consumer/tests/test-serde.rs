use marinade_finance_interface::{
    ChangeAuthorityData, ChangeAuthorityIxArgs, ChangeAuthorityIxData, DepositIxArgs,
    MarinadeFinanceProgramIx, CHANGE_AUTHORITY_IX_DISCM, DEPOSIT_IX_DISCM,
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
    assert_eq!(serialized[..8], CHANGE_AUTHORITY_IX_DISCM);
    let deserialized = ChangeAuthorityIxData::deserialize(&serialized).unwrap();
    assert_eq!(sample, deserialized);
}

#[test]
fn test_program_ix_borsh_roundtrip() {
    let program_ix = MarinadeFinanceProgramIx::Deposit(DepositIxArgs { lamports: 1000 });
    let serialized = program_ix.try_to_vec().unwrap();
    assert_eq!(serialized[..8], DEPOSIT_IX_DISCM);
    let deserialized = MarinadeFinanceProgramIx::deserialize(&serialized).unwrap();
    assert_eq!(program_ix, deserialized);
}
