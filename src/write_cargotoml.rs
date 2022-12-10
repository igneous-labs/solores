use std::io::Write;

use serde::Serialize;

use crate::{idl_format::IdlFormat, utils::open_file_create_overwrite, Args};

pub fn write_cargotoml<'a, I: IdlFormat>(args: &'a Args, idl: &'a I) -> std::io::Result<()> {
    let cargo_toml = CargoToml::from_args_and_idl(args, idl);
    let cargo_toml_str = toml::to_string(&cargo_toml).unwrap();

    let path = args.output_dir.join("Cargo.toml");
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(cargo_toml_str.as_bytes())
}

#[derive(Serialize)]
struct CargoToml<'a> {
    package: Package<'a>,
    dependencies: GeneratedCrateDependencies<'a>,
}

impl<'a> CargoToml<'a> {
    pub fn from_args_and_idl<I: IdlFormat>(args: &'a Args, idl: &'a I) -> Self {
        Self {
            package: Package {
                name: &args.output_crate_name,
                version: idl.program_version(),
                edition: "2021",
            },
            dependencies: GeneratedCrateDependencies {
                borsh: &args.borsh_vers,
                solana_program: &args.solana_program_vers,
            },
        }
    }
}

#[derive(Serialize)]
struct Package<'a> {
    name: &'a str,
    version: &'a str,
    edition: &'a str,
}

#[derive(Serialize)]
struct GeneratedCrateDependencies<'a> {
    borsh: &'a str,

    #[serde(rename = "solana-program")]
    solana_program: &'a str,
}
