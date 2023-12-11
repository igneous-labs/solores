#![cfg(feature = "test_gen_examples")]

//! NB: Running these tests generate ~20GB of rust target/ output because
//! we can't put the examples into the same workspace since they use
//! different vers of borsh and solana

use test_utils::{check_example, gen_example};

const BASE_WORKSPACE_DEPS_ARGS: [&str; 14] = [
    "--solana-program-vers",
    "workspace=true",
    "--borsh-vers",
    "workspace=true",
    "--thiserror-vers",
    "workspace=true",
    "--num-derive-vers",
    "workspace=true",
    "--num-traits-vers",
    "workspace=true",
    "--serde-vers",
    "workspace=true",
    "--bytemuck-vers",
    "workspace=true",
];

#[test]
fn test_token_metadata() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "shank/token_metadata";
    gen_example(EXAMPLE_PATH, &BASE_WORKSPACE_DEPS_ARGS)?;
    check_example(EXAMPLE_PATH, "mpl_token_metadata_interface")
}

#[test]
fn test_phoenix_v1() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "shank/phoenix_v1";
    gen_example(
        EXAMPLE_PATH,
        &[
            BASE_WORKSPACE_DEPS_ARGS.as_ref(),
            &[
                "-z",
                "Ticks",
                "-z",
                "MarketSizeParams",
                "-z",
                "TokenParams",
                "-z",
                "Seat",
                "-z",
                "MarketHeader",
                "-z",
                "FIFOOrderId",
            ],
        ]
        .concat(),
    )?;
    check_example(EXAMPLE_PATH, "phoenix_v1_interface")
}

#[test]
fn test_unstake_it() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/unstake_it";
    gen_example(EXAMPLE_PATH, &BASE_WORKSPACE_DEPS_ARGS)?;
    check_example(EXAMPLE_PATH, "unstake_interface")
}

#[test]
fn test_marinade() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/marinade";
    gen_example(EXAMPLE_PATH, &BASE_WORKSPACE_DEPS_ARGS)?;
    check_example(EXAMPLE_PATH, "marinade_finance_interface")
}

#[test]
fn test_anchor_ix_no_privilege() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/ix_no_privilege";
    gen_example(EXAMPLE_PATH, &BASE_WORKSPACE_DEPS_ARGS)?;
    check_example(EXAMPLE_PATH, "anchor_ix_no_privilege_interface")
}

#[test]
fn test_drift() -> Result<(), Box<dyn std::error::Error>> {
    const EXAMPLE_PATH: &str = "anchor/drift";
    gen_example(
        EXAMPLE_PATH,
        &[
            BASE_WORKSPACE_DEPS_ARGS.as_ref(),
            &[
                "--program-id",
                "dRiftyHA39MWEi3m9aunc5MzRF1JYuBsbn6VPcn33UH",
            ],
        ]
        .concat(),
    )?;
    check_example(EXAMPLE_PATH, "drift_interface")
}
