use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_accounts(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let body = match idl.accounts_file() {
        None => return Ok(()),
        Some(a) => a,
    };
    let mut contents = idl.accounts_header();
    contents.extend(body);
    write_src_file(args, "src/accounts.rs", contents)
}
