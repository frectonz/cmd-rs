use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, parse_quote, DeriveInput, ItemImpl};

#[proc_macro_derive(Cmd, attributes(prompt))]
pub fn derive_cmd(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let ident = input.ident;

    let doc_comment = input
        .attrs
        .iter()
        .find_map(|attr| match attr.meta.clone() {
            syn::Meta::NameValue(val) if val.path.is_ident("doc") => Some(val.value),
            _ => None,
        })
        .and_then(|expr| match expr {
            syn::Expr::Lit(lit) => Some(lit.lit),
            _ => None,
        })
        .and_then(|lit| match lit {
            syn::Lit::Str(lit_str) => Some(lit_str.value()),
            _ => None,
        })
        .unwrap_or_else(|| "welcome".to_owned());
    let doc_comment = doc_comment.trim();

    let prompt = input
        .attrs
        .iter()
        .find_map(|attr| match attr.meta.clone() {
            syn::Meta::NameValue(val) if val.path.is_ident("prompt") => Some(val.value),
            _ => None,
        })
        .and_then(|expr| match expr {
            syn::Expr::Lit(lit) => Some(lit.lit),
            _ => None,
        })
        .and_then(|lit| match lit {
            syn::Lit::Str(lit_str) => Some(lit_str.value()),
            _ => None,
        })
        .unwrap_or_else(|| "> ".to_owned());

    let expanded = quote! {
        impl Cmd for #ident {
            fn welcome(&self) -> &str {
                #doc_comment
            }

            fn prompt(&self) -> &str {
                #prompt
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_attribute]
pub fn cmd_handler(_: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemImpl);

    let saved_input = input.clone();

    let ident = input.self_ty;

    let postcmd = input
        .items
        .iter()
        .filter_map(|it| match it {
            syn::ImplItem::Fn(f) if f.sig.ident == "postcmd" => Some(f.sig.ident.clone()),
            _ => None,
        })
        .map(|ident| -> syn::Stmt {
            parse_quote! { self.#ident(); }
        });

    let commands = input
        .items
        .clone()
        .into_iter()
        .filter_map(|it| match it {
            syn::ImplItem::Fn(f) => {
                let name = f.sig.ident.to_string();
                let name = name.strip_prefix("do_")?.to_owned();
                let ident = f.sig.ident.to_owned();

                let doc_comment = f
                    .attrs
                    .iter()
                    .find_map(|attr| match attr.meta.clone() {
                        syn::Meta::NameValue(val) if val.path.is_ident("doc") => Some(val.value),
                        _ => None,
                    })
                    .and_then(|expr| match expr {
                        syn::Expr::Lit(lit) => Some(lit.lit),
                        _ => None,
                    })
                    .and_then(|lit| match lit {
                        syn::Lit::Str(lit_str) => Some(lit_str.value()),
                        _ => None,
                    })
                    .unwrap_or_default();
                let doc_comment = doc_comment.trim().to_owned();

                Some((name, ident, doc_comment))
            }
            _ => None,
        })
        .collect::<Vec<_>>();

    let match_arms = commands
        .clone()
        .into_iter()
        .map(|(name, ident, _)| -> syn::Arm {
            let postcmd = postcmd.clone();
            parse_quote! {
                [#name, arg] => {
                    self.#ident(arg);
                    #(#postcmd)*;
                }
            }
        });

    let list_commands = commands.into_iter().map(|(name, _, doc)| -> syn::Stmt {
        parse_quote! { println!("{} - {}", #name, #doc); }
    });

    let expanded = quote! {
        #saved_input

        impl CmdHandler for #ident {
            fn handler(&mut self, line: &str) {
                match line.split(' ').collect::<Vec<&str>>().as_slice() {
                    #(#match_arms)*
                    _ => {
                        println!();
                        println!("Available Commands");
                        println!("============================================");
                        #(#list_commands)*
                        println!();
                    },
                }
            }
        }
    };

    TokenStream::from(expanded)
}
