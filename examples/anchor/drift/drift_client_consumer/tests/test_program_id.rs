mod expected_drift_program_id {
    solana_program::declare_id!("dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH");
}

/// Check that custom program ID was successfully passed to
/// `--program-id`
pub fn test_check_program_id() {
    assert_eq!(expected_drift_program_id::ID, drift_interface::ID);
}
