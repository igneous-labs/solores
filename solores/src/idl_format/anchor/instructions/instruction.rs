// TODO: determine borsh version for more efficient implementations of deserialize_reader
// that makes use of ix_args' deserialize_reader method if available

use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use syn::{LitBool, LitInt};

use crate::{idl_format::anchor::typedefs::TypedefField, utils::unique_by_report_dups};

#[derive(Deserialize)]
pub struct NamedInstruction {
    pub name: String,
    pub accounts: Vec<IxAccountEntry>,
    pub args: Option<Vec<TypedefField>>,
}

impl NamedInstruction {
    pub fn ix_args_ident(&self) -> Ident {
        format_ident!("{}IxArgs", self.name.to_pascal_case())
    }

    pub fn ix_data_ident(&self) -> Ident {
        format_ident!("{}IxData", self.name.to_pascal_case())
    }

    pub fn ix_fn_ident(&self) -> Ident {
        format_ident!("{}_ix", self.name.to_snake_case())
    }

    pub fn discm_ident(&self) -> Ident {
        format_ident!("{}_IX_DISCM", &self.name.to_shouty_snake_case())
    }

    pub fn accounts_ident(&self) -> Ident {
        format_ident!("{}Accounts", self.name.to_pascal_case())
    }

    pub fn keys_ident(&self) -> Ident {
        format_ident!("{}Keys", self.name.to_pascal_case())
    }

    pub fn accounts_len_ident(&self) -> Ident {
        format_ident!("{}_IX_ACCOUNTS_LEN", self.name.to_shouty_snake_case())
    }

    pub fn has_ix_args(&self) -> bool {
        let args = match &self.args {
            Some(a) => a,
            None => return false,
        };
        !args.is_empty()
    }

    pub fn args_has_defined_type(&self) -> bool {
        let args = if !self.has_ix_args() {
            return false;
        } else {
            self.args.as_ref().unwrap()
        };
        args.iter().map(|a| a.r#type.is_or_has_defined()).any(|b| b)
    }

    pub fn has_privileged_accounts(&self) -> bool {
        self.accounts
            .iter()
            .map(|a| a.has_privileged_accounts())
            .any(|b| b)
    }

    /// export accounts_len as const
    pub fn write_accounts_len(&self, tokens: &mut TokenStream, accounts_len: usize) {
        let accounts_len_ident = self.accounts_len_ident();
        let n_accounts_lit = LitInt::new(&accounts_len.to_string(), Span::call_site());
        tokens.extend(quote! {
            pub const #accounts_len_ident: usize = #n_accounts_lit;
        });
    }

    pub fn write_accounts_struct(&self, tokens: &mut TokenStream, unique_accounts: &[&IxAccount]) {
        let accounts_ident = self.accounts_ident();
        let accounts_fields = unique_accounts.iter().map(|acc| {
            let account_name = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
                pub #account_name: &'me AccountInfo<'info>
            }
        });
        tokens.extend(quote! {
            #[derive(Copy, Clone, Debug)]
            pub struct #accounts_ident<'me, 'info> {
                #(#accounts_fields),*
            }
        });
    }

    pub fn write_keys_struct(&self, tokens: &mut TokenStream, unique_accounts: &[&IxAccount]) {
        let keys_ident = self.keys_ident();
        let keys_fields = unique_accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
                pub #account_ident: Pubkey
            }
        });
        tokens.extend(quote! {
            #[derive(Copy, Clone, Debug, PartialEq)]
            pub struct #keys_ident {
                #(#keys_fields),*
            }
        });
    }

    /// From<&XAccounts> for XKeys
    pub fn write_from_accounts_for_keys(
        &self,
        tokens: &mut TokenStream,
        unique_accounts: &[&IxAccount],
    ) {
        let accounts_ident = self.accounts_ident();
        let keys_ident = self.keys_ident();
        let from_keys_fields = unique_accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
                #account_ident: *accounts.#account_ident.key
            }
        });
        tokens.extend(quote! {
            impl From<&#accounts_ident<'_, '_>> for #keys_ident {
                fn from(accounts: &#accounts_ident) -> Self {
                    Self {
                        #(#from_keys_fields),*
                    }
                }
            }
        });
    }

    /// From <&XKeys> for [AccountMeta]
    pub fn write_from_keys_for_meta_arr(&self, tokens: &mut TokenStream, accounts: &[IxAccount]) {
        let keys_ident = self.keys_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let from_keys_meta = accounts.iter().map(|acc| acc.to_keys_account_meta_tokens());
        tokens.extend(quote! {
            impl From<&#keys_ident> for [AccountMeta; #accounts_len_ident] {
                fn from(keys: &#keys_ident) -> Self {
                    [
                        #(#from_keys_meta),*
                    ]
                }
            }
        });
    }

    /// From <[Pubkey]> for XKeys
    pub fn write_from_pubkey_arr_for_keys(
        &self,
        tokens: &mut TokenStream,
        unique_accounts: &[&IxAccount],
    ) {
        let accounts_len_ident = self.accounts_len_ident();
        let keys_ident = self.keys_ident();
        let from_pubkey_arr_fields = unique_accounts.iter().enumerate().map(|(i, acc)| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            let index_lit = LitInt::new(&i.to_string(), Span::call_site());
            quote! {
                #account_ident: pubkeys[#index_lit]
            }
        });
        tokens.extend(quote! {
            impl From<[Pubkey; #accounts_len_ident]> for #keys_ident {
                fn from(pubkeys: [Pubkey; #accounts_len_ident]) -> Self {
                    Self {
                        #(#from_pubkey_arr_fields),*
                    }
                }
            }
        });
    }

    /// From <XAccounts> for [AccountInfo]
    pub fn write_from_accounts_for_account_info_arr(
        &self,
        tokens: &mut TokenStream,
        accounts: &[IxAccount],
    ) {
        let accounts_ident = self.accounts_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let account_info_clone = accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
               accounts.#account_ident.clone()
            }
        });
        tokens.extend(quote! {
            impl<'info> From<&#accounts_ident<'_, 'info>> for [AccountInfo<'info>; #accounts_len_ident] {
                fn from(accounts: &#accounts_ident<'_, 'info>) -> Self {
                    [
                        #(#account_info_clone),*
                    ]
                }
            }
        });
    }

    /// From <[AccountInfo]> for XAccounts
    pub fn write_from_account_info_arr_for_accounts(
        &self,
        tokens: &mut TokenStream,
        accounts: &[IxAccount],
    ) {
        let accounts_ident = self.accounts_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let from_account_info_fields = accounts.iter().enumerate().map(|(i, acc)| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            let index_lit = LitInt::new(&i.to_string(), Span::call_site());
            quote! {
               #account_ident: &arr[#index_lit]
            }
        });
        tokens.extend(quote! {
            impl<'me, 'info> From<&'me [AccountInfo<'info>; #accounts_len_ident]> for #accounts_ident<'me, 'info> {
                fn from(arr: &'me [AccountInfo<'info>; #accounts_len_ident]) -> Self {
                    Self {
                        #(#from_account_info_fields),*
                    }
                }
            }
        });
    }

    pub fn write_ix_args_struct(&self, tokens: &mut TokenStream) {
        let args = if !self.has_ix_args() {
            return;
        } else {
            self.args.as_ref().unwrap()
        };
        let ix_args_ident = self.ix_args_ident();
        let args_fields = args.iter().map(|a| quote! { pub #a });
        tokens.extend(quote! {
            #[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
            #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
            pub struct #ix_args_ident {
                #(#args_fields),*
            }
        });
    }

    pub fn write_discm(&self, tokens: &mut TokenStream) {
        let discm_ident = self.discm_ident();
        // pre-image: "global:{instruction_fn_name}"
        // TODO: handle non-global instructions (state methods - idk if thats deprecated)
        let discm = <[u8; 8]>::try_from(
            &Sha256::digest(format!("global:{}", self.name.to_snake_case()).as_bytes()).as_slice()
                [..8],
        )
        .unwrap();
        let discm_value_tokens: TokenStream = format!("{:?}", discm).parse().unwrap();
        tokens.extend(quote! {
            pub const #discm_ident: [u8; 8] = #discm_value_tokens;

        })
    }

    pub fn write_ix_data_struct(&self, tokens: &mut TokenStream) {
        let ix_data_ident = self.ix_data_ident();
        let struct_decl = if self.has_ix_args() {
            let ix_args_ident = self.ix_args_ident();
            quote! { pub struct #ix_data_ident(pub #ix_args_ident); }
        } else {
            quote! { pub struct #ix_data_ident; }
        };

        tokens.extend(quote! {
            #[derive(Clone, Debug, PartialEq)]
            #struct_decl
        });
    }

    pub fn write_from_ix_args_for_ix_data(&self, tokens: &mut TokenStream) {
        if !self.has_ix_args() {
            return;
        }
        let ix_data_ident = self.ix_data_ident();
        let ix_args_ident = self.ix_args_ident();
        tokens.extend(quote! {
            impl From<#ix_args_ident> for #ix_data_ident {
                fn from(args: #ix_args_ident) -> Self {
                    Self(args)
                }
            }
        });
    }

    pub fn write_ix_data_impl(&self, tokens: &mut TokenStream) {
        let discm_ident = self.discm_ident();
        let ix_data_ident = self.ix_data_ident();
        let mut deserialize_body = quote! {
            let mut reader = buf;
            let mut maybe_discm = [0u8; 8];
            reader.read_exact(&mut maybe_discm)?;
            if maybe_discm != #discm_ident {
                return Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other, format!("discm does not match. Expected: {:?}. Received: {:?}", #discm_ident, maybe_discm)
                    )
                );
            }
        };
        if self.has_ix_args() {
            let ix_args_ident = self.ix_args_ident();
            deserialize_body.extend(quote! {
                Ok(Self(#ix_args_ident::deserialize(&mut reader)?))
            })
        } else {
            deserialize_body.extend(quote! {
                Ok(Self)
            })
        }
        let serialize_body = if self.has_ix_args() {
            quote! {
                writer.write_all(&#discm_ident)?;
                self.0.serialize(&mut writer)
            }
        } else {
            quote! {
                writer.write_all(&#discm_ident)
            }
        };
        tokens.extend(quote! {
            impl #ix_data_ident {
                pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
                    #deserialize_body
                }

                pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
                    #serialize_body
                }

                pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
                    let mut data = Vec::new();
                    self.serialize(&mut data)?;
                    Ok(data)
                }
            }
        });
    }

    /// _ix()
    pub fn write_ix_fn(&self, tokens: &mut TokenStream) {
        let ix_fn_ident = self.ix_fn_ident();
        let keys_ident = self.keys_ident();
        let ix_args_ident = self.ix_args_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let ix_data_ident = self.ix_data_ident();

        let mut fn_generics = quote! { K: Into<#keys_ident>,};
        if self.has_ix_args() {
            fn_generics.extend(quote! { A: Into<#ix_args_ident> })
        }

        let mut fn_params = quote! { accounts: K, };
        if self.has_ix_args() {
            fn_params.extend(quote! { args: A,  });
        }

        let mut fn_body = quote! {
            let keys: #keys_ident = accounts.into();
            let metas: [AccountMeta; #accounts_len_ident] = (&keys).into();
        };
        if self.has_ix_args() {
            fn_body.extend(quote! {
                let args_full: #ix_args_ident = args.into();
                let data: #ix_data_ident = args_full.into();
            })
        }
        let data_expr = if self.has_ix_args() {
            quote! { data.try_to_vec()? }
        } else {
            quote! { #ix_data_ident.try_to_vec()? }
        };

        tokens.extend(quote! {
            pub fn #ix_fn_ident<#fn_generics>(#fn_params) -> std::io::Result<Instruction> {
                #fn_body
                Ok(Instruction {
                    program_id: crate::ID,
                    accounts: Vec::from(metas),
                    data: #data_expr,
                })
            }
        });
    }

    fn invoke_fn_generics(&self) -> TokenStream {
        let mut res = quote! {'info,};
        if self.has_ix_args() {
            let ix_args_ident = self.ix_args_ident();
            res.extend(quote! { A: Into<#ix_args_ident> });
        }
        res
    }

    fn invoke_fn_params_prefix(&self) -> TokenStream {
        let accounts_ident = self.accounts_ident();
        let mut fn_params = quote! {accounts: &#accounts_ident<'_, 'info>,};
        if self.has_ix_args() {
            fn_params.extend(quote! { args: A, })
        }
        fn_params
    }

    fn ix_fn_call_assign(&self) -> TokenStream {
        let ix_fn_ident = self.ix_fn_ident();
        if self.has_ix_args() {
            quote! { let ix = #ix_fn_ident(accounts, args)?; }
        } else {
            quote! { let ix = #ix_fn_ident(accounts)?; }
        }
    }

    /// _invoke()
    pub fn write_invoke_fn(&self, tokens: &mut TokenStream) {
        let invoke_fn_ident = format_ident!("{}_invoke", self.name.to_snake_case());
        let accounts_len_ident = self.accounts_len_ident();
        let fn_generics = self.invoke_fn_generics();
        let fn_params = self.invoke_fn_params_prefix();
        let call_assign = self.ix_fn_call_assign();
        tokens.extend(quote! {
            pub fn #invoke_fn_ident<#fn_generics>(#fn_params) -> ProgramResult {
                #call_assign
                let account_info: [AccountInfo<'info>; #accounts_len_ident] = accounts.into();
                invoke(&ix, &account_info)
            }
        });
    }

    /// _invoke_signed()
    pub fn write_invoke_signed_fn(&self, tokens: &mut TokenStream) {
        let invoke_signed_fn_ident = format_ident!("{}_invoke_signed", self.name.to_snake_case());
        let accounts_len_ident = self.accounts_len_ident();
        let fn_generics = self.invoke_fn_generics();
        let mut fn_params = self.invoke_fn_params_prefix();
        fn_params.extend(quote! { seeds: &[&[&[u8]]], });
        let call_assign = self.ix_fn_call_assign();
        tokens.extend(quote! {
            pub fn #invoke_signed_fn_ident<#fn_generics>(#fn_params) -> ProgramResult {
                #call_assign
                let account_info: [AccountInfo<'info>; #accounts_len_ident] = accounts.into();
                invoke_signed(&ix, &account_info, seeds)
            }
        });
    }

    /// _verify_account_keys()
    pub fn write_verify_account_keys_fn(
        &self,
        tokens: &mut TokenStream,
        unique_accounts: &[&IxAccount],
    ) {
        let verify_account_keys_fn_ident =
            format_ident!("{}_verify_account_keys", self.name.to_snake_case());
        let accounts_ident = self.accounts_ident();
        let keys_ident = self.keys_ident();
        let key_tups = unique_accounts
            .iter()
            .map(|a| IxAccount::to_verify_account_keys_tuple(a));
        // edge-case of accounts and keys being empty
        let pubkeys_loop_check = if unique_accounts.is_empty() {
            quote! {}
        } else {
            quote! {
                for (actual, expected) in [
                    #(#key_tups),*
                ] {
                    if actual != expected {
                        return Err((*actual, *expected));
                    }
                }
            }
        };
        tokens.extend(quote! {
            pub fn #verify_account_keys_fn_ident(
                accounts: &#accounts_ident<'_, '_>,
                keys: &#keys_ident
            ) -> Result<(), (Pubkey, Pubkey)> {
                #pubkeys_loop_check
                Ok(())
            }
        });
    }

    // _verify_account_privileges
    pub fn write_verify_account_privileges_fn(
        &self,
        tokens: &mut TokenStream,
        unique_accounts: &[&IxAccount],
    ) {
        if !self.has_privileged_accounts() {
            return;
        }
        let verify_account_privileges_fn_ident =
            format_ident!("{}_verify_account_privileges", self.name.to_snake_case());
        let mut writables = unique_accounts
            .iter()
            .filter_map(|a| {
                if a.is_mut {
                    let name = a.field_ident();
                    Some(quote! {
                        accounts.#name
                    })
                } else {
                    None
                }
            })
            .peekable();
        let writables_loop_check = if writables.peek().is_none() {
            quote! {}
        } else {
            quote! {
                for should_be_writable in [
                    #(#writables),*
                ] {
                    if !should_be_writable.is_writable {
                        return Err((should_be_writable, ProgramError::InvalidAccountData));
                    }
                }
            }
        };
        let mut signers = unique_accounts
            .iter()
            .filter_map(|a| {
                if a.is_signer {
                    let name = a.field_ident();
                    Some(quote! {
                        accounts.#name
                    })
                } else {
                    None
                }
            })
            .peekable();
        let signers_loop_check = if signers.peek().is_none() {
            quote! {}
        } else {
            quote! {
                for should_be_signer in [
                    #(#signers),*
                ] {
                    if !should_be_signer.is_signer {
                        return Err((should_be_signer, ProgramError::MissingRequiredSignature));
                    }
                }
            }
        };
        let accounts_ident = self.accounts_ident();
        tokens.extend(quote! {
            pub fn #verify_account_privileges_fn_ident<'me, 'info>(
                accounts: &#accounts_ident<'me, 'info>,
            ) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
                #writables_loop_check
                #signers_loop_check
                Ok(())
            }
        });
    }
}

impl ToTokens for NamedInstruction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let accounts = to_ix_accounts(&self.accounts);
        let n_accounts = accounts.len();

        let accounts_dedup = unique_by_report_dups(accounts.iter(), |acc| acc.name.clone());

        if !accounts_dedup.duplicates.is_empty() {
            log::warn!(
                "Found duplicate accounts for instruction {}: {}. Assuming different indexes in generated AccountInfo/Meta arrays refer to the same account",
                &self.name, accounts_dedup.duplicates.iter().map(|acc| &acc.name).format(", ")
            );
        }
        let unique_accounts = &accounts_dedup.unique;

        self.write_accounts_len(tokens, n_accounts);
        self.write_accounts_struct(tokens, unique_accounts);
        self.write_keys_struct(tokens, unique_accounts);
        self.write_from_accounts_for_keys(tokens, unique_accounts);
        self.write_from_keys_for_meta_arr(tokens, &accounts);
        self.write_from_pubkey_arr_for_keys(tokens, unique_accounts);
        self.write_from_accounts_for_account_info_arr(tokens, &accounts);
        self.write_from_account_info_arr_for_accounts(tokens, &accounts);

        self.write_discm(tokens);
        self.write_ix_args_struct(tokens);
        self.write_ix_data_struct(tokens);
        self.write_from_ix_args_for_ix_data(tokens);
        self.write_ix_data_impl(tokens);

        self.write_ix_fn(tokens);
        self.write_invoke_fn(tokens);
        self.write_invoke_signed_fn(tokens);

        self.write_verify_account_keys_fn(tokens, unique_accounts);
        self.write_verify_account_privileges_fn(tokens, unique_accounts);
    }
}

#[derive(Deserialize)]
pub struct InnerAccountStruct {
    pub name: String,
    pub accounts: Vec<IxAccountEntry>,
}

impl InnerAccountStruct {
    pub fn has_privileged_accounts(&self) -> bool {
        self.accounts
            .iter()
            .map(|a| a.has_privileged_accounts())
            .any(|b| b)
    }
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum IxAccountEntry {
    Account(IxAccount),
    Struct(Box<InnerAccountStruct>),
}

impl IxAccountEntry {
    pub fn has_privileged_accounts(&self) -> bool {
        match self {
            Self::Account(a) => a.is_privileged(),
            Self::Struct(s) => s.has_privileged_accounts(),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IxAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
}

impl IxAccount {
    pub fn field_ident(&self) -> Ident {
        format_ident!("{}", self.name.to_snake_case())
    }

    pub fn is_privileged(&self) -> bool {
        self.is_mut || self.is_signer
    }

    pub fn to_keys_account_meta_tokens(&self) -> TokenStream {
        let is_writable_arg = LitBool::new(self.is_mut, Span::call_site());
        let is_signer_arg = LitBool::new(self.is_signer, Span::call_site());
        let name = self.field_ident();
        quote! {
            AccountMeta {
                pubkey: keys.#name,
                is_signer: #is_signer_arg,
                is_writable: #is_writable_arg,
            }
        }
    }

    pub fn to_verify_account_keys_tuple(&self) -> TokenStream {
        let name = self.field_ident();
        quote! {
            (accounts.#name.key, &keys.#name)
        }
    }
}

pub fn to_ix_accounts(accounts: &[IxAccountEntry]) -> Vec<IxAccount> {
    accounts.iter().fold(Vec::new(), |mut vec, entry| {
        match entry {
            IxAccountEntry::Account(a) => vec.push(a.clone()),
            IxAccountEntry::Struct(s) => {
                vec.extend(to_ix_accounts(&s.accounts).into_iter().map(|mut acc| {
                    acc.name = format!("{}_{}", s.name, acc.name.to_snake_case());
                    acc
                }))
            }
        };
        vec
    })
}
