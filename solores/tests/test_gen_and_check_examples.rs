#![cfg(test)]

use test_utils::{check_example, gen_example};

#[test]
fn test_token_metadata() -> Result<(), Box<dyn std::error::Error>> {
    gen_example(
        "shank/token-metadata",
        &[
            "--solana-program-vers",
            "^1.9,<1.16",
            "--borsh-vers",
            "^0.9",
        ],
    )?;
    check_example("shank/token-metadata", "mpl_token_metadata_interface")
}

#[test]
fn test_unstake_it() -> Result<(), Box<dyn std::error::Error>> {
    gen_example(
        "anchor/unstake-it",
        &[
            "--solana-program-vers",
            "^1.9,<1.16",
            "--borsh-vers",
            "^0.9",
        ],
    )?;
    check_example("anchor/unstake-it", "unstake_interface")
}

#[test]
fn test_marinade() -> Result<(), Box<dyn std::error::Error>> {
    gen_example(
        "anchor/marinade",
        &[
            "--solana-program-vers",
            "^1.9,<1.16",
            "--borsh-vers",
            "^0.9",
        ],
    )?;
    check_example("anchor/marinade", "marinade_finance_interface")
}

#[test]
fn test_drift() -> Result<(), Box<dyn std::error::Error>> {
    gen_example("anchor/drift", &[])?;
    check_example("anchor/drift", "drift_interface")
}
