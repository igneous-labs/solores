use quote::{quote, ToTokens};

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
    let mut contents = quote! {
        // TODO: maybe these imports should be part of the idl trait like instructions_headers() or smth

        use crate::*;
        use borsh::{BorshDeserialize, BorshSerialize};
        use solana_program::{
            account_info::AccountInfo,
            entrypoint::ProgramResult,
            instruction::{AccountMeta, Instruction},
            program::{invoke, invoke_signed},
            pubkey::Pubkey,
        };
    };
    for t in ixs.iter() {
        contents.extend(t.into_token_stream());
    }

    write_src_file(args, "src/instructions.rs", contents)
}
