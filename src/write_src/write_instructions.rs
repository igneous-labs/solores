use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_instructions(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let body = match idl.instructions_file() {
        None => return Ok(()),
        Some(i) => i,
    };
    let mut contents = idl.instructions_header();
    contents.extend(body);
    write_src_file(args, "src/instructions.rs", contents)
}
