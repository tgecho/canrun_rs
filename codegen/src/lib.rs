extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Result, Token};

struct DomainDef {
    domain_name: syn::Ident,
    domain_type: Vec<syn::Type>,
}

impl Parse for DomainDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let domain_name: syn::Ident = input.parse()?;
        let raw_types: Punctuated<syn::Type, Token![,]> = Punctuated::parse_terminated(input)?;
        let domain_type: Vec<_> = raw_types.into_iter().collect();

        Ok(DomainDef {
            domain_name,
            domain_type,
        })
    }
}

#[proc_macro]
pub fn domain(item: TokenStream) -> TokenStream {
    let DomainDef {
        domain_name,
        domain_type,
    } = parse_macro_input!(item as DomainDef);

    let (field, variant): (Vec<_>, Vec<_>) = (0..domain_type.len())
        .into_iter()
        .map(|n| (format_ident!("t{}", n), format_ident!("T{}", n)))
        .unzip();

    let value_name = format_ident!("{}Value", domain_name);

    let result = quote! {
        #[derive(std::fmt::Debug)]
        pub struct #domain_name {
            #(#field: im::HashMap<canrun::value::LVar<#domain_type>, canrun::value::Val<#domain_type>>),*
        }

        impl<'a> canrun::domain::Domain<'a> for #domain_name {
            type Value = #value_name;
            fn new() -> Self {
                #domain_name {
                    #(#field: im::HashMap::new(),)*
                }
            }
            fn unify_domain_values(
                state: canrun::state::State<'a, Self>,
                a: Self::Value,
                b: Self::Value,
            ) -> Option<canrun::state::State<Self>> {
                use canrun::value::Val;
                match (a, b) {
                    #(
                        (#value_name::#variant(a), #value_name::#variant(b)) => {
                            state.unify::<#domain_type, Val<#domain_type>, Val<#domain_type>>(a, b)
                        }
                    ,)*
                    _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
                }
            }
        }

        #(
            impl<'a> canrun::domain::IntoDomainVal<'a, #domain_type> for #domain_name {
                fn into_domain_val(val: canrun::value::Val<#domain_type>) -> #value_name {
                    #value_name::#variant(val)
                }
            }
        )*

        #(
            impl<'a> canrun::domain::DomainType<'a, #domain_type> for #domain_name {
                fn values_as_ref(
                    &self,
                ) -> &im::HashMap<canrun::value::LVar<#domain_type>, canrun::value::Val<#domain_type>> {
                    &self.#field
                }
                fn values_as_mut(
                    &mut self,
                ) -> &mut im::HashMap<canrun::value::LVar<#domain_type>, canrun::value::Val<#domain_type>> {
                    &mut self.#field
                }
            }
        )*

        #(
            impl<'a> canrun::domain::UnifyIn<'a, #domain_name> for #domain_type {
                fn unify_with(&self, other: &Self) -> canrun::domain::Unified<'a, #domain_name> {
                    if self == other {
                        canrun::domain::Unified::Success
                    } else {
                        canrun::domain::Unified::Failed
                    }
                }
            }
        )*

        impl<'a> Clone for #domain_name {
            fn clone(&self) -> Self {
                #domain_name {
                    #(#field: self.#field.clone()),*
                }
            }
        }

        #[derive(std::fmt::Debug)]
        pub enum #value_name {
            #(#variant(canrun::value::Val<#domain_type>)),*
        }

        impl Clone for #value_name {
            fn clone(&self) -> Self {
                match self {
                    #(#value_name::#variant(val) => #value_name::#variant(val.clone())),*
                }
            }
        }
    };
    result.into()
}
