use std::io::Write;

use serde::Serialize;

use crate::{idl_format::IdlFormat, utils::open_file_create_overwrite, Args};

pub fn write_cargotoml(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let cargo_toml = CargoToml::from_args_and_idl(args, idl);
    let cargo_toml_str = toml::to_string(&cargo_toml).unwrap();

    let path = args.output_dir.join("Cargo.toml");
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(cargo_toml_str.as_bytes())?;
    file.flush()
}

#[derive(Serialize)]
struct CargoToml<'a> {
    package: Package<'a>,
    dependencies: GeneratedCrateDependencies<'a>,
}

impl<'a> CargoToml<'a> {
    pub fn from_args_and_idl(args: &'a Args, idl: &'a dyn IdlFormat) -> Self {
        let (thiserror, num_derive, num_traits) = match idl.has_errors() {
            true => (
                Some(args.thiserror_vers.as_str()),
                Some(args.num_derive_vers.as_str()),
                Some(args.num_traits_vers.as_str()),
            ),
            false => (None, None, None),
        };
        Self {
            package: Package {
                name: &args.output_crate_name,
                version: idl.program_version(),
                edition: "2021",
            },
            dependencies: GeneratedCrateDependencies {
                borsh: &args.borsh_vers,
                solana_program: &args.solana_program_vers,
                thiserror,
                num_derive,
                num_traits,
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

    thiserror: Option<&'a str>,

    #[serde(rename = "num-derive")]
    num_derive: Option<&'a str>,

    #[serde(rename = "num-traits")]
    num_traits: Option<&'a str>,
}
