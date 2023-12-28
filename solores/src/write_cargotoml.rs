use std::io::Write;

use serde::Serialize;
use toml::{map::Map, Value};

use crate::{idl_format::IdlFormat, utils::open_file_create_overwrite, Args};

pub const BORSH_CRATE: &str = "borsh";
pub const BYTEMUCK_CRATE: &str = "bytemuck";
pub const SERDE_CRATE: &str = "serde";
pub const SOLANA_PROGRAM_CRATE: &str = "solana-program";
pub const THISERROR_CRATE: &str = "thiserror";
pub const NUM_DERIVE_CRATE: &str = "num-derive";
pub const NUM_TRAITS_CRATE: &str = "num-traits";

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
    pub package: Package<'a>,
    pub dependencies: Map<String, Value>,
}

impl<'a> CargoToml<'a> {
    pub fn from_args_and_idl(args: &'a Args, idl: &'a dyn IdlFormat) -> Self {
        Self {
            package: Package {
                name: &args.output_crate_name,
                version: idl.program_version(),
                edition: "2021",
            },
            dependencies: idl.dependencies(args),
        }
    }
}

#[derive(Serialize)]
pub struct Package<'a> {
    pub name: &'a str,
    pub version: &'a str,
    pub edition: &'a str,
}

/// Contained str value is the version string arg.
/// e.g. "^1.16", "workspace = true"
pub struct DependencyValue<'a>(pub &'a str);

impl From<DependencyValue<'_>> for Map<String, Value> {
    fn from(value: DependencyValue) -> Self {
        match toml::from_str::<Map<_, _>>(value.0) {
            Ok(m) => m, // "workspace = true"
            Err(_) => {
                let mut map = Map::new();
                map.insert("version".into(), value.0.into());
                map
            }
        }
    }
}

impl From<DependencyValue<'_>> for Value {
    fn from(value: DependencyValue) -> Self {
        Value::Table(value.into())
    }
}

pub struct OptionalDependencyValue<T>(pub T);

impl<T: Into<Map<String, Value>>> From<OptionalDependencyValue<T>> for Map<String, Value> {
    fn from(value: OptionalDependencyValue<T>) -> Self {
        let mut map = value.0.into();
        map.insert("optional".into(), true.into());
        map
    }
}

impl<T: Into<Map<String, Value>>> From<OptionalDependencyValue<T>> for Value {
    fn from(value: OptionalDependencyValue<T>) -> Self {
        Value::Table(value.into())
    }
}

/// Contained str value is the version string arg.
/// e.g. "^1.16", "workspace = true"
pub struct FeaturesDependencyValue<T> {
    pub dependency: T,
    pub features: Vec<String>,
}

impl<T: Into<Map<String, Value>>> From<FeaturesDependencyValue<T>> for Map<String, Value> {
    fn from(
        FeaturesDependencyValue {
            dependency,
            features,
        }: FeaturesDependencyValue<T>,
    ) -> Self {
        let mut map = dependency.into();
        map.insert("features".into(), features.into());
        map
    }
}

impl<T: Into<Map<String, Value>>> From<FeaturesDependencyValue<T>> for Value {
    fn from(value: FeaturesDependencyValue<T>) -> Self {
        Value::Table(value.into())
    }
}
