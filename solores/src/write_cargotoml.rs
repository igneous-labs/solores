use std::io::Write;

use serde::Serialize;
use toml::map::Map;

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
pub struct CargoToml<'a> {
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
                borsh: DependencyValue(&args.borsh_vers),
                solana_program: DependencyValue(&args.solana_program_vers),
                serde: OptionalDependencyValue(&args.serde_vers),
                thiserror: thiserror.map(DependencyValue),
                num_derive: num_derive.map(DependencyValue),
                num_traits: num_traits.map(DependencyValue),
            },
        }
    }
}

#[derive(Serialize)]
pub struct Package<'a> {
    name: &'a str,
    version: &'a str,
    edition: &'a str,
}

#[derive(Serialize)]
pub struct GeneratedCrateDependencies<'a> {
    borsh: DependencyValue<'a>,

    #[serde(rename = "solana-program")]
    solana_program: DependencyValue<'a>,

    serde: OptionalDependencyValue<'a>,

    thiserror: Option<DependencyValue<'a>>,

    #[serde(rename = "num-derive")]
    num_derive: Option<DependencyValue<'a>>,

    #[serde(rename = "num-traits")]
    num_traits: Option<DependencyValue<'a>>,
}

pub struct DependencyValue<'a>(&'a str);

impl Serialize for DependencyValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match toml::from_str::<Map<_, _>>(self.0) {
            Ok(v) => v.serialize(serializer), // "workspace = true"
            Err(_) => self.0.serialize(serializer),
        }
    }
}

pub struct OptionalDependencyValue<'a>(&'a str);

impl Serialize for OptionalDependencyValue<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut map: Map<_, _> = match toml::from_str(self.0) {
            Ok(v) => v,
            Err(_) => {
                let mut m = Map::new();
                m.insert("version".to_owned(), self.0.into());
                m
            }
        };
        map.insert("optional".to_owned(), true.into());
        map.serialize(serializer)
    }
}
