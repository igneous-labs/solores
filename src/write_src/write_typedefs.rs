use quote::{quote, ToTokens};

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_typedefs<'a, T: ToTokens, A: ToTokens, I: IdlFormat<T, A>>(
    args: &'a Args,
    idl: &'a I,
) -> std::io::Result<()> {
    let typedefs = idl.typedefs();
    let mut contents = quote! {
        // TODO: not all typedefs use pubkey, remove if unnecessary
        use solana_program::pubkey::Pubkey;
        use borsh::{BorshDeserialize, BorshSerialize};
    };
    for t in typedefs.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/typedefs.rs", contents)
}
