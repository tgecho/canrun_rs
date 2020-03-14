extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Result, Token};

// #[derive(Debug)]
struct MyMacroInput {
    ident: syn::Ident,
    types: Vec<syn::Type>,
}

impl Parse for MyMacroInput {
    fn parse(input: ParseStream) -> Result<Self> {
        dbg!(&input);
        let ident: syn::Ident = input.parse()?;
        let raw_types: Punctuated<syn::Type, Token![,]> = Punctuated::parse_terminated(input)?;
        let types: Vec<_> = raw_types.into_iter().collect();

        Ok(MyMacroInput { ident, types })
    }
}

#[proc_macro]
pub fn hello(item: TokenStream) -> TokenStream {
    let MyMacroInput { ident, types } = parse_macro_input!(item as MyMacroInput);

    let types: Vec<_> = types
        .into_iter()
        .enumerate()
        .map(|(i, t)| {
            let field = format_ident!("t{}", i);
            quote! {
                #field: im::HashMap<crate::value::LVar<#t>, crate::value::Val<#t>>
            }
        })
        .collect();

    let result = quote! {
        #[derive(std::fmt::Debug)]
        pub struct #ident {
            #(#types),*
        }
    };
    result.into()
}
