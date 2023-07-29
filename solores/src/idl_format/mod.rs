#![doc = include_str!("./README.md")]

use proc_macro2::TokenStream;

pub mod anchor;
pub mod shank;

pub trait IdlCodegenModule {
    /// The module file's name e.g. "errors"
    fn name(&self) -> &str;

    /// Generate the headers to prefix the module file with.
    /// Typically import statements
    fn gen_head(&self) -> TokenStream;

    /// Generate the main body content of the module file
    fn gen_body(&self) -> TokenStream;
}

pub trait IdlFormat {
    fn program_name(&self) -> &str;

    fn program_version(&self) -> &str;

    fn program_address(&self) -> Option<&str>;

    fn is_correct_idl_format(&self) -> bool;

    fn has_errors(&self) -> bool;

    fn modules<'me>(&'me self) -> Vec<Box<dyn IdlCodegenModule + 'me>>;
}

/// The gen_body() impl for simple newtypes i.e. `struct NewType(&[ToTokens])`
#[macro_export]
macro_rules! gen_body_newtype_slice {
    () => {
        fn gen_body(&self) -> TokenStream {
            let elems = self.0.iter().map(|e| e.into_token_stream());
            let mut res = quote! {};
            res.extend(elems);
            res
        }
    };
}
