use std::{
    fs::{self, File},
    path::PathBuf,
};

use clap::{command, Parser};
use idl_format::IdlFormat;

use crate::idl_format::shank::ShankIdl;

mod idl_format;
mod utils;
mod write_cargotoml;
mod write_gitignore;
mod write_src;

use write_cargotoml::write_cargotoml;
use write_gitignore::write_gitignore;
use write_src::*;

const DEFAULT_OUTPUT_CRATE_NAME: &str = "<name-of-program>_interface";

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
        short,
        help = "Keep partially output built artifacts instead of deleting everything. Useful for debugging",
        default_value = "false"
    )]
    pub keep_partial_artifacts: bool,

    #[arg(
        long,
        short,
        help = "solana-program dependency version for generated crate",
        default_value = "^1.9"
    )]
    pub solana_program_vers: String,

    #[arg(
        long,
        short,
        help = "borsh dependency version for generated crate",
        default_value = "^0.9"
    )]
    pub borsh_vers: String,

    #[arg(
        long,
        help = "output crate name",
        default_value = DEFAULT_OUTPUT_CRATE_NAME,
    )]
    pub output_crate_name: String,
}

fn check_valid_semver_req(arg: &str, arg_name: &str) {
    semver::VersionReq::parse(arg)
        .unwrap_or_else(|_| panic!("Invalid version req '{}' for {}", arg, arg_name));
}

fn main() {
    env_logger::init();

    let mut args = Args::parse();

    check_valid_semver_req(&args.solana_program_vers, "solana-program");
    check_valid_semver_req(&args.borsh_vers, "borsh");

    let file = File::open(&args.idl_path).unwrap();

    // TODO: anchor
    let idl: ShankIdl = serde_json::from_reader(&file).unwrap();

    args.output_crate_name = if args.output_crate_name == DEFAULT_OUTPUT_CRATE_NAME {
        format!("{}_interface", idl.program_name())
    } else {
        args.output_crate_name
    };

    args.output_dir.push(&args.output_crate_name);
    fs::create_dir_all(&args.output_dir.join("src/")).unwrap();

    // TODO: multithread, 1 thread per generated file
    write_gitignore(&args).unwrap();
    write_cargotoml(&args, &idl).unwrap();
    write_lib(&args, &idl).unwrap();
    write_accounts(&args, &idl).unwrap();
    write_typedefs(&args, &idl).unwrap();
}
