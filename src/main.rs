use std::path::PathBuf;

use clap::{command, Parser};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
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
        default_value = "<name-of-program>_interface"
    )]
    pub output_crate_name: String,
}

fn main() {
    env_logger::init();
    let args = Args::parse();

    // TODO: multithread, 1 thread per generated file
    println!("{:?}, {}", args.idl_path, args.keep_partial_artifacts);
}
