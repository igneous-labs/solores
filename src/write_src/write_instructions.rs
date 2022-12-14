use quote::ToTokens;

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_instructions<'a, T: ToTokens, A: ToTokens, I: ToTokens, Idl: IdlFormat<T, A, I>>(
    args: &'a Args,
    idl: &'a Idl,
) -> std::io::Result<()> {
    let ixs = match idl.instructions() {
        None => return Ok(()),
        Some(i) => i,
    };
    let mut contents = idl.instructions_header();
    for t in ixs.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/instructions.rs", contents)
}
