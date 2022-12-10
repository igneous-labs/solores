use std::{
    fs::{File, OpenOptions},
    path::Path,
};

pub fn open_file_create_overwrite<P: AsRef<Path>>(path: P) -> std::io::Result<File> {
    OpenOptions::new().create(true).write(true).open(path)
}
