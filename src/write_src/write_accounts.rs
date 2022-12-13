use quote::{quote, ToTokens};

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
    let mut contents = quote! {
        // TODO: maybe these imports should be part of the idl trait like accounts_headers() or smth

        // TODO: not all accounts use defined types, remove if necessary
        use crate::*;
        use borsh::{BorshDeserialize, BorshSerialize};
        // TODO: not all accounts use pubkey, remove if unnecessary
        use solana_program::pubkey::Pubkey;
    };
    for t in accounts.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/accounts.rs", contents)
}
