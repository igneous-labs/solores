#![doc = include_str!("../README.md")]

use std::{
    env,
    fs::{self, File, OpenOptions},
    io::Seek,
    path::PathBuf,
};

use clap::{command, Parser};
use idl_format::IdlFormat;

use crate::idl_format::{anchor::AnchorIdl, shank::ShankIdl};

mod idl_format;
mod utils;
mod write_cargotoml;
mod write_gitignore;
mod write_src;

use write_cargotoml::write_cargotoml;
use write_gitignore::write_gitignore;
use write_src::*;

const DEFAULT_OUTPUT_CRATE_NAME: &str = "<name-of-program>_interface";
const RUST_LOG_ENV_VAR: &str = "RUST_LOG";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    pub idl_path: PathBuf,

    #[arg(
        long,
        short,
        help = "directory to output generated crate to",
        default_value = "./"
    )]
    pub output_dir: PathBuf,

    #[arg(
        long,
        help = "output crate name",
        default_value = DEFAULT_OUTPUT_CRATE_NAME,
    )]
    pub output_crate_name: String,

    #[arg(
        long,
        short,
        help = "solana-program dependency version for generated crate",
        default_value = "^1.16"
    )]
    pub solana_program_vers: String,

    #[arg(
        long,
        short,
        help = "borsh dependency version for generated crate",
        default_value = "^0.10"
    )]
    pub borsh_vers: String,

    #[arg(
        long,
        help = "thiserror dependency version for generated crate",
        default_value = "^1.0"
    )]
    pub thiserror_vers: String,

    #[arg(
        long,
        help = "num-derive dependency version for generated crate",
        default_value = "^0.3"
    )]
    pub num_derive_vers: String,

    #[arg(
        long,
        help = "num-traits dependency version for generated crate",
        default_value = "^0.2"
    )]
    pub num_traits_vers: String,

    #[arg(
        long,
        help = "serde dependency version for generated crate",
        default_value = "^1.0"
    )]
    pub serde_vers: String,
}

fn main() {
    if env::var(RUST_LOG_ENV_VAR).is_err() {
        env::set_var(RUST_LOG_ENV_VAR, "info")
    }
    env_logger::init();
    log_panics::init();

    let mut args = Args::parse();

    let mut file = OpenOptions::new().read(true).open(&args.idl_path).unwrap();

    let idl = load_idl(&mut file);

    args.output_crate_name = if args.output_crate_name == DEFAULT_OUTPUT_CRATE_NAME {
        format!("{}_interface", idl.program_name())
    } else {
        args.output_crate_name
    };

    args.output_dir.push(&args.output_crate_name);
    fs::create_dir_all(args.output_dir.join("src/")).unwrap();

    // TODO: multithread, 1 thread per generated file
    write_gitignore(&args).unwrap();
    write_cargotoml(&args, idl.as_ref()).unwrap();
    write_lib(&args, idl.as_ref()).unwrap();
    write_accounts(&args, idl.as_ref()).unwrap();
    write_errors(&args, idl.as_ref()).unwrap();
    write_typedefs(&args, idl.as_ref()).unwrap();
    write_instructions(&args, idl.as_ref()).unwrap();

    log::info!(
        "{} crate written to {}",
        args.output_crate_name,
        args.output_dir.to_string_lossy()
    );
}

fn load_idl(file: &mut File) -> Box<dyn IdlFormat> {
    if let Ok(shank_idl) = serde_json::from_reader::<&File, ShankIdl>(file) {
        if shank_idl.is_correct_idl_format() {
            log::info!("Successfully loaded shank IDL");
            return Box::new(shank_idl);
        }
    }
    file.rewind().unwrap();
    // Assume anchor if unidentified
    match serde_json::from_reader::<&File, AnchorIdl>(file) {
        Ok(anchor_idl) => {
            log::info!("Successfully loaded anchor IDL");
            Box::new(anchor_idl)
        }
        Err(e) => {
            panic!("Could not determine IDL format: {:?}", e);
        }
    }
}
