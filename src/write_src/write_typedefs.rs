use quote::ToTokens;

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_typedefs<'a, T: ToTokens, A: ToTokens, I: ToTokens, Idl: IdlFormat<T, A, I>>(
    args: &'a Args,
    idl: &'a Idl,
) -> std::io::Result<()> {
    let typedefs = match idl.typedefs() {
        None => return Ok(()),
        Some(t) => t,
    };
    let mut contents = idl.typedefs_header();
    for t in typedefs.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/typedefs.rs", contents)
}
