use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_errors(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let body = match idl.errors_file() {
        None => return Ok(()),
        Some(a) => a,
    };
    let mut contents = idl.errors_header();
    contents.extend(body);
    write_src_file(args, "src/errors.rs", contents)
}
