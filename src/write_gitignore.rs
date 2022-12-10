use std::{io::Write, path::PathBuf};

use crate::utils::open_file_create_overwrite;

pub fn write_gitignore<P: Into<PathBuf>>(dir: P) -> std::io::Result<()> {
    let mut path: PathBuf = dir.into();
    path.push(".gitignore");
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(b"/target\nCargo.lock")
}
