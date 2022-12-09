use std::{path::PathBuf, process::Command};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

const BIN_NAME: &'static str = "solores";

fn example_dir(example_name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.extend(&["examples", example_name]);
    p
}

fn run_example(example_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;

    let dir = example_dir(example_name);

    let mut idl_path = dir.clone();
    idl_path.push("idl.json");

    cmd.arg(idl_path).arg("-o").arg(dir).arg("-k");
    cmd.assert().success();
    Ok(())
}

#[test]
fn test_token_metadata() -> Result<(), Box<dyn std::error::Error>> {
    run_example("token-metadata")
}
