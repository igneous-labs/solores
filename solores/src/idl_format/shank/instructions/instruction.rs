use heck::{ToPascalCase, ToShoutySnakeCase, ToSnakeCase};
use itertools::Itertools;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use syn::{LitBool, LitInt};

use crate::{idl_format::shank::typedefs::TypedefField, utils::unique_by_report_dups};

#[derive(Deserialize)]
pub struct NamedInstruction {
    pub name: String,
    pub accounts: Option<Vec<IxAccount>>,
    pub args: Option<Vec<TypedefField>>,
    pub discriminant: Discriminant,
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

    pub fn has_accounts(&self) -> bool {
        let accounts = match &self.accounts {
            Some(a) => a,
            None => return false,
        };
        !accounts.is_empty()
    }

    pub fn args_has_defined_type(&self) -> bool {
        let args = if !self.has_ix_args() {
            return false;
        } else {
            self.args.as_ref().unwrap()
        };
        args.iter().map(|a| a.r#type.is_or_has_defined()).any(|b| b)
    }

    pub fn args_has_pubkeys(&self) -> bool {
        let args = if !self.has_ix_args() {
            return false;
        } else {
            self.args.as_ref().unwrap()
        };
        args.iter().map(|a| a.r#type.is_or_has_pubkey()).any(|b| b)
    }

    pub fn has_privileged_accounts(&self) -> bool {
        let accounts = if !self.has_accounts() {
            return false;
        } else {
            self.accounts.as_ref().unwrap()
        };
        accounts.iter().map(|a| a.is_privileged()).any(|b| b)
    }

    /// export accounts_len as const
    pub fn write_accounts_len(&self, tokens: &mut TokenStream, accounts_len: usize) {
        if !self.has_accounts() {
            return;
        }
        let accounts_len_ident = self.accounts_len_ident();
        let n_accounts_lit = LitInt::new(&accounts_len.to_string(), Span::call_site());
        tokens.extend(quote! {
            pub const #accounts_len_ident: usize = #n_accounts_lit;
        });
    }

    pub fn write_accounts_struct(&self, tokens: &mut TokenStream, unique_accounts: &[&IxAccount]) {
        if !self.has_accounts() {
            return;
        }
        let accounts_ident = self.accounts_ident();
        let accounts_fields = unique_accounts.iter().map(|acc| {
            let account_name = format_ident!("{}", &acc.name.to_snake_case());
            let maybe_doc_comment = acc.desc.as_ref().map_or(quote! {}, |desc| {
                quote! {
                    #[doc = #desc]
                }
            });
            quote! {
                #maybe_doc_comment
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
        if !self.has_accounts() {
            return;
        }
        let keys_ident = self.keys_ident();
        let keys_fields = unique_accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            let maybe_doc_comment = acc.desc.as_ref().map_or(quote! {}, |desc| {
                quote! {
                    #[doc = #desc]
                }
            });
            quote! {
                #maybe_doc_comment
                pub #account_ident: Pubkey
            }
        });
        tokens.extend(quote! {
            #[derive(Copy, Clone, Debug)]
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
        if !self.has_accounts() {
            return;
        }
        let accounts_ident = self.accounts_ident();
        let keys_ident = self.keys_ident();
        let from_keys_fields = unique_accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
                #account_ident: *accounts.#account_ident.key
            }
        });
        tokens.extend(quote! {
            impl From<#accounts_ident<'_, '_>> for #keys_ident {
                fn from(accounts: #accounts_ident) -> Self {
                    Self {
                        #(#from_keys_fields),*
                    }
                }
            }
        });
    }

    /// From <&XKeys> for [AccountMeta]
    pub fn write_from_keys_for_meta_arr(&self, tokens: &mut TokenStream, accounts: &[IxAccount]) {
        if !self.has_accounts() {
            return;
        }
        let keys_ident = self.keys_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let from_keys_meta = accounts.iter().map(|acc| acc.to_keys_account_meta_tokens());
        tokens.extend(quote! {
            impl From<#keys_ident> for [AccountMeta; #accounts_len_ident] {
                fn from(keys: #keys_ident) -> Self {
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
        if !self.has_accounts() {
            return;
        }
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
        if !self.has_accounts() {
            return;
        }
        let accounts_ident = self.accounts_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let account_info_clone = accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
               accounts.#account_ident.clone()
            }
        });
        tokens.extend(quote! {
            impl<'info> From<#accounts_ident<'_, 'info>> for [AccountInfo<'info>; #accounts_len_ident] {
                fn from(accounts: #accounts_ident<'_, 'info>) -> Self {
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
        if !self.has_accounts() {
            return;
        }
        let accounts_len_ident = self.accounts_len_ident();
        let accounts_ident = self.accounts_ident();
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
        let discm_value = self.discriminant.value;
        tokens.extend(quote! {
            pub const #discm_ident: u8 = #discm_value;
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
            let mut maybe_discm_buf = [0u8; 1];
            reader.read_exact(&mut maybe_discm_buf)?;
            let maybe_discm = maybe_discm_buf[0];
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
                writer.write_all(&[#discm_ident])?;
                self.0.serialize(&mut writer)
            }
        } else {
            quote! {
                writer.write_all(&[#discm_ident])
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
    /// _ix_with_program_id()
    pub fn write_ix_fn(&self, tokens: &mut TokenStream) {
        let ix_fn_ident = self.ix_fn_ident();
        let ix_with_program_id_fn_ident =
            format_ident!("{}_ix_with_program_id", self.name.to_snake_case());
        let keys_ident = self.keys_ident();
        let ix_args_ident = self.ix_args_ident();
        let accounts_len_ident = self.accounts_len_ident();
        let ix_data_ident = self.ix_data_ident();

        let mut fn_params = quote! {};
        let mut fn_args = quote! {};
        if self.has_accounts() {
            fn_params.extend(quote! { keys: #keys_ident, });
            fn_args.extend(quote! { keys, });
        }
        if self.has_ix_args() {
            fn_params.extend(quote! { args: #ix_args_ident, });
            fn_args.extend(quote! { args, });
        }

        let (mut fn_body, accounts_expr) = if self.has_accounts() {
            (
                quote! {
                    let metas: [AccountMeta; #accounts_len_ident] = keys.into();
                },
                quote! {
                    Vec::from(metas)
                },
            )
        } else {
            (
                quote! {},
                quote! {
                    Vec::new()
                },
            )
        };
        if self.has_ix_args() {
            fn_body.extend(quote! {
                let data: #ix_data_ident = args.into();
            })
        }
        let data_expr = if self.has_ix_args() {
            quote! { data.try_to_vec()? }
        } else {
            quote! { #ix_data_ident.try_to_vec()? }
        };

        tokens.extend(quote! {
            pub fn #ix_with_program_id_fn_ident(program_id: Pubkey, #fn_params) -> std::io::Result<Instruction> {
                #fn_body
                Ok(Instruction {
                    program_id,
                    accounts: #accounts_expr,
                    data: #data_expr,
                })
            }

            pub fn #ix_fn_ident(#fn_params) -> std::io::Result<Instruction> {
                #ix_with_program_id_fn_ident(crate::ID, #fn_args)
            }
        });
    }

    fn invoke_fn_params_prefix(&self) -> TokenStream {
        let accounts_ident = self.accounts_ident();
        let ix_args_ident = self.ix_args_ident();
        let mut fn_params = quote! {};
        if self.has_accounts() {
            fn_params.extend(quote! { accounts: #accounts_ident<'_, '_>, });
        }
        if self.has_ix_args() {
            fn_params.extend(quote! { args: #ix_args_ident, })
        }
        fn_params
    }

    fn ix_fn_call_assign(&self) -> TokenStream {
        let ix_fn_ident = self.ix_fn_ident();
        let keys_ident = self.keys_ident();
        let mut res = quote! {};
        let mut args = quote! {};
        if self.has_accounts() {
            res.extend(quote! {
                let keys: #keys_ident = accounts.into();
            });
            args.extend(quote! { keys, });
        }
        if self.has_ix_args() {
            args.extend(quote! { args });
        }
        res.extend(quote! {
            let ix = #ix_fn_ident(#args)?;
        });
        res
    }

    /// _invoke()
    pub fn write_invoke_fn(&self, tokens: &mut TokenStream) {
        let invoke_fn_ident = format_ident!("{}_invoke", self.name.to_snake_case());
        let fn_params = self.invoke_fn_params_prefix();
        let call_assign = self.ix_fn_call_assign();
        let invoke = if self.has_accounts() {
            quote! {
                invoke_instruction(&ix, accounts)
            }
        } else {
            quote! {
                invoke(&ix, &[])
            }
        };
        tokens.extend(quote! {
            pub fn #invoke_fn_ident(#fn_params) -> ProgramResult {
                #call_assign
                #invoke
            }
        });
    }

    /// _invoke_signed()
    pub fn write_invoke_signed_fn(&self, tokens: &mut TokenStream) {
        let invoke_signed_fn_ident = format_ident!("{}_invoke_signed", self.name.to_snake_case());
        let mut fn_params = self.invoke_fn_params_prefix();
        fn_params.extend(quote! { seeds: &[&[&[u8]]], });
        let call_assign = self.ix_fn_call_assign();
        let invoke = if self.has_accounts() {
            quote! {
                invoke_instruction_signed(&ix, accounts, seeds)
            }
        } else {
            quote! {
                invoke_signed(&ix, &[], seeds)
            }
        };
        tokens.extend(quote! {
            pub fn #invoke_signed_fn_ident(#fn_params) -> ProgramResult {
                #call_assign
                #invoke
            }
        });
    }

    /// _verify_account_keys()
    pub fn write_verify_account_keys_fn(
        &self,
        tokens: &mut TokenStream,
        unique_accounts: &[&IxAccount],
    ) {
        if !self.has_accounts() {
            return;
        }
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
                accounts: #accounts_ident<'_, '_>,
                keys: #keys_ident
            ) -> Result<(), (Pubkey, Pubkey)> {
                #pubkeys_loop_check
                Ok(())
            }
        });
    }

    // _verify_account_privileges()
    // _verify_writable_privileges()
    // _verify_signer_privileges()
    pub fn write_verify_account_privileges_fns(
        &self,
        tokens: &mut TokenStream,
        unique_accounts: &[&IxAccount],
    ) {
        if !self.has_privileged_accounts() {
            return;
        }
        let verify_account_privileges_fn_ident =
            format_ident!("{}_verify_account_privileges", self.name.to_snake_case());
        let verify_writable_privileges_fn_ident =
            format_ident!("{}_verify_writable_privileges", self.name.to_snake_case());
        let verify_signer_privileges_fn_ident =
            format_ident!("{}_verify_signer_privileges", self.name.to_snake_case());
        let accounts_ident = self.accounts_ident();

        let mut verify_fn_body = quote! {};

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
        let has_writables = writables.peek().is_some();
        if has_writables {
            tokens.extend(quote! {
                pub fn #verify_writable_privileges_fn_ident<'me, 'info>(
                    accounts: #accounts_ident<'me, 'info>,
                ) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
                    for should_be_writable in [
                        #(#writables),*
                    ] {
                        if !should_be_writable.is_writable {
                            return Err((should_be_writable, ProgramError::InvalidAccountData));
                        }
                    }
                    Ok(())
                }
            });
            verify_fn_body.extend(quote! {
                #verify_writable_privileges_fn_ident(accounts)?;
            });
        }

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
        let has_signers = signers.peek().is_some();
        if has_signers {
            tokens.extend(quote! {
                pub fn #verify_signer_privileges_fn_ident<'me, 'info>(
                    accounts: #accounts_ident<'me, 'info>,
                ) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
                    for should_be_signer in [
                        #(#signers),*
                    ] {
                        if !should_be_signer.is_signer {
                            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
                        }
                    }
                    Ok(())
                }
            });
            verify_fn_body.extend(quote! {
                #verify_signer_privileges_fn_ident(accounts)?;
            });
        }

        tokens.extend(quote! {
            pub fn #verify_account_privileges_fn_ident<'me, 'info>(
                accounts: #accounts_ident<'me, 'info>,
            ) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
                #verify_fn_body
                Ok(())
            }
        });
    }
}

impl ToTokens for NamedInstruction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let accounts: &[IxAccount] = self.accounts.as_ref().map_or(&[], |v| v.as_slice());
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
        self.write_from_keys_for_meta_arr(tokens, accounts);
        self.write_from_pubkey_arr_for_keys(tokens, unique_accounts);
        self.write_from_accounts_for_account_info_arr(tokens, accounts);
        self.write_from_account_info_arr_for_accounts(tokens, accounts);

        self.write_discm(tokens);
        self.write_ix_args_struct(tokens);
        self.write_ix_data_struct(tokens);
        self.write_from_ix_args_for_ix_data(tokens);
        self.write_ix_data_impl(tokens);

        self.write_ix_fn(tokens);
        self.write_invoke_fn(tokens);
        self.write_invoke_signed_fn(tokens);

        self.write_verify_account_keys_fn(tokens, unique_accounts);
        self.write_verify_account_privileges_fns(tokens, unique_accounts);
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IxAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: Option<String>,
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

#[derive(Deserialize)]
pub struct Discriminant {
    pub r#type: String,
    pub value: u8,
}
