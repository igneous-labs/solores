use quote::{quote, ToTokens};

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_accounts<'a, T: ToTokens, A: ToTokens, I: IdlFormat<T, A>>(
    args: &'a Args,
    idl: &'a I,
) -> std::io::Result<()> {
    let typedefs = idl.accounts();
    let mut contents = quote! {
        // TODO: not all accounts use defined types, remove if necessary
        use crate::*;
        // TODO: not all accounts use pubkey, remove if unnecessary
        use solana_program::pubkey::Pubkey;
        use borsh::{BorshDeserialize, BorshSerialize};
    };
    for t in typedefs.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/accounts.rs", contents)
}
