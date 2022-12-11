use quote::{quote, ToTokens};

use crate::{idl_format::IdlFormat, Args};

use super::write_src_file;

pub fn write_lib<'a, T: ToTokens, A: ToTokens, I: IdlFormat<T, A>>(
    args: &'a Args,
    idl: &'a I,
) -> std::io::Result<()> {
    let id = idl.program_address();
    let contents = quote! {
        pub mod accounts;
        pub mod instructions;
        pub mod typedefs;

        pub use accounts::*;
        pub use instructions::*;
        pub use typedefs::*;

        solana_program::declare_id!(#id);
    };
    write_src_file(args, "src/lib.rs", contents)
}
