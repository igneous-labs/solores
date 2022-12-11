#[allow(clippy::all)]
use heck::ToSnakeCase;
use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote, ToTokens};
use serde::Deserialize;
use syn::{Generics, Index, Lifetime, LifetimeDef, LitBool};

use super::typedefs::TypedefField;

#[derive(Deserialize)]
pub struct NamedInstruction {
    pub name: String,
    pub accounts: Vec<IxAccount>,
    pub args: Vec<TypedefField>,
    pub discriminant: Discriminant,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IxAccount {
    pub name: String,
    pub is_mut: bool,
    pub is_signer: bool,
    pub desc: Option<String>,
}

#[derive(Deserialize)]
pub struct Discriminant {
    pub r#type: String,
    pub value: u8,
}

impl ToTokens for NamedInstruction {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let name = &self.name;
        let accounts_ident = format_ident!("{}Accounts", name);
        let accounts = &self.accounts;
        let n_accounts = accounts.len();
        let n_accounts_index = Index::from(n_accounts);

        // impl Accounts
        let mut accounts_lifetimes = Generics::default();
        let struct_lifetime = LifetimeDef::new(Lifetime::new("'me", Span::call_site()));
        accounts_lifetimes.params.push(struct_lifetime.into());
        for i in 0..n_accounts {
            let mut account_info_lifetime =
                LifetimeDef::new(Lifetime::new(&format!("'a{}", i), Span::call_site()));
            account_info_lifetime
                .bounds
                .push(Lifetime::new("'me", Span::call_site()));
            accounts_lifetimes.params.push(account_info_lifetime.into());
        }
        let accounts_fields = accounts.iter().enumerate().map(|(i, acc)| {
            let account_name = format_ident!("{}", &acc.name.to_snake_case());
            let mut account_info_lifetime = Generics::default();
            account_info_lifetime.params.push(
                LifetimeDef::new(Lifetime::new(&format!("'a{}", i), Span::call_site())).into(),
            );
            quote! {
                pub #account_name: &'me AccountInfo #account_info_lifetime
            }
        });
        tokens.extend(quote! {
            #[derive(Copy, Clone, Debug)]
            pub struct #accounts_ident #accounts_lifetimes {
                #(#accounts_fields),*
            }
        });

        // impl Keys
        let keys_ident = format_ident!("{}Keys", name);
        let keys_fields = accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
                pub #account_ident: &'me Pubkey
            }
        });
        tokens.extend(quote! {
            #[derive(Copy, Clone, Debug)]
            pub struct #keys_ident<'me> {
                #(#keys_fields),*
            }
        });

        // impl From &Accounts for Keys
        let mut from_accounts_generics = Generics::default();
        from_accounts_generics
            .params
            .push(LifetimeDef::new(Lifetime::new("'me", Span::call_site())).into());
        for _i in 0..n_accounts {
            from_accounts_generics
                .params
                .push(LifetimeDef::new(Lifetime::new("'_", Span::call_site())).into());
        }
        let from_keys_fields = accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
                #account_ident: accounts.#account_ident.key
            }
        });
        tokens.extend(quote! {
            impl<'me> From<&#accounts_ident #from_accounts_generics> for #keys_ident<'me> {
                fn from(accounts: &#accounts_ident #from_accounts_generics) -> Self {
                    Self {
                        #(#from_keys_fields),*
                    }
                }
            }
        });

        // impl From &Keys for [AccountMeta]
        let from_keys_meta = accounts.iter().map(|acc| acc.to_keys_account_meta_tokens());
        tokens.extend(quote! {
            impl From<&#keys_ident<'_>> for [AccountMeta; #n_accounts_index] {
                fn from(keys: &#keys_ident<'_>) -> Self {
                    [
                        #(#from_keys_meta),*
                    ]
                }
            }
        });

        // impl From Accounts for [AccountInfo]
        let mut from_accounts_to_account_infos_generics = Generics::default();
        from_accounts_to_account_infos_generics
            .params
            .push(LifetimeDef::new(Lifetime::new("'_", Span::call_site())).into());
        for _i in 0..n_accounts {
            from_accounts_to_account_infos_generics
                .params
                .push(LifetimeDef::new(Lifetime::new("'a", Span::call_site())).into());
        }
        let account_info_clone = accounts.iter().map(|acc| {
            let account_ident = format_ident!("{}", &acc.name.to_snake_case());
            quote! {
               accounts.#account_ident.clone()
            }
        });
        tokens.extend(quote! {
            impl<'a> From<&#accounts_ident #from_accounts_to_account_infos_generics> for [AccountInfo<'a>; #n_accounts_index] {
                fn from(accounts: &#accounts_ident #from_accounts_to_account_infos_generics) -> Self {
                    [
                        #(#account_info_clone),*
                    ]
                }
            }
        });
    }
}

impl IxAccount {
    pub fn to_keys_account_meta_tokens(&self) -> TokenStream {
        let call_ident = format_ident!(
            "{}",
            match self.is_mut {
                true => "new",
                false => "new_readonly",
            }
        );
        let is_signer_arg = LitBool::new(self.is_signer, Span::call_site());
        let name = format_ident!("{}", self.name.to_snake_case());
        quote! {
            AccountMeta::#call_ident(*keys.#name, #is_signer_arg)
        }
    }
}
