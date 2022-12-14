use quote::ToTokens;

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_accounts<'a, T: ToTokens, A: ToTokens, I: ToTokens, Idl: IdlFormat<T, A, I>>(
    args: &'a Args,
    idl: &'a Idl,
) -> std::io::Result<()> {
    let accounts = match idl.accounts() {
        None => return Ok(()),
        Some(a) => a,
    };
    let mut contents = idl.accounts_header();
    for t in accounts.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/accounts.rs", contents)
}
