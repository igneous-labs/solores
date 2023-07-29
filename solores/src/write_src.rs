use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use solana_program::pubkey::Pubkey;
use std::{io::Write, path::Path, str::FromStr};

use crate::{idl_format::IdlFormat, utils::open_file_create_overwrite, Args};

const DEFAULT_ADDRESS: &str = "11111111111111111111111111111111";

pub fn write_lib(args: &Args, idl: &dyn IdlFormat) -> std::io::Result<()> {
    let user_provided_id_opt = args.program_id.as_ref().and_then(|s| {
        Pubkey::from_str(s)
            .map_err(|e| {
                log::warn!("provided pubkey {s} invalid. Error: {e}");
                e
            })
            .ok()
            .map(|_| s)
    });
    let id = user_provided_id_opt
        .map(|string| string.as_ref())
        .unwrap_or_else(|| {
            idl.program_address().unwrap_or_else(|| {
                log::warn!(
                    "program address not in IDL, setting to default: {}",
                    DEFAULT_ADDRESS
                );
                DEFAULT_ADDRESS
            })
        });

    let mut contents = quote! {
        solana_program::declare_id!(#id);
    };

    for module in idl.modules() {
        let module_name = module.name();
        let module_ident = Ident::new(module.name(), Span::call_site());
        contents.extend(quote! {
            pub mod #module_ident;
            pub use #module_ident::*;
        });
        let mut module_contents = module.gen_head();
        module_contents.extend(module.gen_body());
        write_src_file(args, &format!("src/{module_name}.rs"), module_contents)?;
    }

    write_src_file(args, "src/lib.rs", contents)
}

fn write_src_file<P: AsRef<Path>>(
    args: &Args,
    src_file_path: P,
    contents: TokenStream,
) -> std::io::Result<()> {
    let unpretty = syn::parse2(contents).unwrap();
    let formatted = prettyplease::unparse(&unpretty);

    let path = args.output_dir.join(src_file_path);
    let mut file = open_file_create_overwrite(path)?;
    file.write_all(formatted.as_bytes())?;
    file.flush()
}
