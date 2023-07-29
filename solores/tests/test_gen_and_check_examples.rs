//! NB: Running these tests generate ~20GB of rust target/ output because
//! we can't put the examples into the same workspace since they use
//! different vers of borsh and solana

use test_utils::{check_example, gen_example, test_consumer};

#[test]
fn test_token_metadata() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "shank/token-metadata";
    gen_example(
        EXAMPLE_PATH,
        &[
            "--solana-program-vers",
            "^1.9,<1.16",
            "--borsh-vers",
            "^0.9",
        ],
    )?;
    check_example(EXAMPLE_PATH, "mpl_token_metadata_interface")?;
    test_consumer(EXAMPLE_PATH, "mpl_token_metadata_onchain_consumer")?;
    test_consumer(EXAMPLE_PATH, "mpl_token_metadata_client_consumer")
}

#[test]
fn test_unstake_it() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/unstake-it";
    gen_example(
        EXAMPLE_PATH,
        &[
            "--solana-program-vers",
            "^1.9,<1.16",
            "--borsh-vers",
            "^0.9",
        ],
    )?;
    check_example(EXAMPLE_PATH, "unstake_interface")?;
    test_consumer(EXAMPLE_PATH, "unstake_onchain_consumer")?;
    test_consumer(EXAMPLE_PATH, "unstake_client_consumer")
}

#[test]
fn test_marinade() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/marinade";
    gen_example(
        EXAMPLE_PATH,
        &[
            "--solana-program-vers",
            "^1.9,<1.16",
            "--borsh-vers",
            "^0.9",
        ],
    )?;
    check_example(EXAMPLE_PATH, "marinade_finance_interface")?;
    test_consumer(EXAMPLE_PATH, "marinade_finance_client_consumer")
}

#[test]
fn test_drift() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/drift";
    gen_example(
        EXAMPLE_PATH,
        &[
            "--program-id",
            "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
        ],
    )?;
    check_example(EXAMPLE_PATH, "drift_interface")?;
    test_consumer(EXAMPLE_PATH, "drift_client_consumer")
}
