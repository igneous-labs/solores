use std::{path::PathBuf, process::Command};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

const BIN_NAME: &'static str = "solores";

fn example_dir(example_name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.extend(&["examples", example_name]);
    p
}

fn run_example(
    example_name: &str,
    gen_package_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut solores_cmd = Command::cargo_bin(BIN_NAME)?;

    let dir = example_dir(example_name);

    let mut idl_path = dir.clone();
    idl_path.push("idl.json");

    solores_cmd.arg(idl_path).arg("-o").arg(&dir);
    solores_cmd.assert().success();

    // cargo check to ensure valid rust code
    let mut cargo_check_cmd = Command::new("cargo");
    let mut generated_cargo_toml_path = dir.clone();
    generated_cargo_toml_path.push(gen_package_name);
    generated_cargo_toml_path.push("Cargo.toml");
    cargo_check_cmd
        .arg("check")
        .arg("--manifest-path")
        .arg(generated_cargo_toml_path);
    cargo_check_cmd.assert().success();

    Ok(())
}

#[test]
fn test_token_metadata() -> Result<(), Box<dyn std::error::Error>> {
    run_example("shank/token-metadata", "mpl_token_metadata_interface")
}

#[test]
fn test_unstake_it() -> Result<(), Box<dyn std::error::Error>> {
    run_example("anchor/unstake-it", "unstake_interface")
}

#[test]
fn test_marinade() -> Result<(), Box<dyn std::error::Error>> {
    run_example("anchor/marinade", "marinade_finance_interface")
}

#[test]
fn test_drift() -> Result<(), Box<dyn std::error::Error>> {
    run_example("anchor/drift", "drift_interface")
}
