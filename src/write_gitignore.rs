use std::io::Write;

use crate::{utils::open_file_create_overwrite, Args};

pub fn write_gitignore(args: &Args) -> std::io::Result<()> {
    let path = args.output_dir.join(".gitignore");
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(b"/target\nCargo.lock")
}
