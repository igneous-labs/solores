use quote::{quote, ToTokens};

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_typedefs<'a, T: ToTokens, A: ToTokens, I: ToTokens, Idl: IdlFormat<T, A, I>>(
    args: &'a Args,
    idl: &'a Idl,
) -> std::io::Result<()> {
    let typedefs = idl.typedefs();
    let mut contents = quote! {
        // TODO: maybe these imports should be part of the idl trait like typedefs_headers() or smth

        use borsh::{BorshDeserialize, BorshSerialize};
        // TODO: not all typedefs use pubkey, remove if unnecessary
        use solana_program::pubkey::Pubkey;
    };
    for t in typedefs.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/typedefs.rs", contents)
}
