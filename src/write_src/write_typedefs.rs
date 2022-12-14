use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_typedefs(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let body = match idl.typedefs_file() {
        None => return Ok(()),
        Some(t) => t,
    };
    let mut contents = idl.typedefs_header();
    contents.extend(body);
    write_src_file(args, "src/typedefs.rs", contents)
}
