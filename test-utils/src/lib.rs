use std::{path::PathBuf, process::Command};

use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

pub const BIN_NAME: &str = "solores";

pub fn example_dir(example_name: &str) -> PathBuf {
    let mut p = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    p.extend(&["../", "examples", example_name]);
    p
}

pub fn gen_example(example_name: &str, args: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    let mut solores_cmd = Command::cargo_bin(BIN_NAME)?;

    let dir = example_dir(example_name);

    let mut idl_path = dir.clone();
    idl_path.push("idl.json");

    solores_cmd.arg(idl_path).arg("-o").arg(&dir);
    for arg in args {
        solores_cmd.arg(arg);
    }
    solores_cmd.assert().success();

    Ok(())
}

/// `cargo check` a generated interface crate
/// to ensure valid rust code
pub fn check_example(
    example_name: &str,
    gen_package_name: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut generated_cargo_toml_path = example_dir(example_name);
    generated_cargo_toml_path.push(gen_package_name);
    generated_cargo_toml_path.push("Cargo.toml");
    let mut cargo_check_cmd = Command::new("cargo");
    cargo_check_cmd
        .current_dir(example_dir(example_name))
        .arg("check")
        .arg("--manifest-path")
        .arg(generated_cargo_toml_path);
    cargo_check_cmd.assert().success();
    Ok(())
}
