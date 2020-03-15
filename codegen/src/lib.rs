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
        .map(|n| (format_ident!("t{}", n), format_ident!("t{}", n)))
        .unzip();

    // let value_name = format_ident!("{}_Value", self.ident);
    let value_name = format_ident!("DomainValue");

    let result = quote! {
        #[derive(std::fmt::Debug)]
        pub struct #domain_name {
            #(#field: im::HashMap<crate::value::LVar<#domain_type>, crate::value::Val<#domain_type>>),*
        }

        impl<'a> crate::domain::Domain<'a> for #domain_name {
            type Value = DomainValue;
            fn new() -> Self {
                #domain_name {
                    #(#field: im::HashMap::new(),)*
                }
            }
            fn unify_domain_values(
                state: crate::state::State<'a, Self>,
                a: Self::Value,
                b: Self::Value,
            ) -> Option<crate::state::State<Self>> {
                match (a, b) {
                    #((#value_name::#variant(a), #value_name::#variant(b)) => {
                        state.unify::<#domain_type, crate::value::Val<#domain_type>, crate::value::Val<#domain_type>>(a, b)
                    },)*
                    _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
                }
            }
        }

        #(
            impl<'a> crate::domain::IntoDomainVal<'a, #domain_type> for #domain_name {
                fn into_domain_val(val: crate::value::Val<#domain_type>) -> #value_name {
                    #value_name::#variant(val)
                }
            }
        )*

        #(
            impl<'a> crate::domain::DomainType<'a, #domain_type> for #domain_name {
                fn values_as_ref(
                    &self,
                ) -> &im::HashMap<crate::value::LVar<#domain_type>, crate::value::Val<#domain_type>> {
                    &self.#field
                }
                fn values_as_mut(
                    &mut self,
                ) -> &mut im::HashMap<crate::value::LVar<#domain_type>, crate::value::Val<#domain_type>> {
                    &mut self.#field
                }
            }
        )*

        #(
            impl<'a> crate::domain::UnifyIn<'a, #domain_name> for #domain_type {
                fn unify_with(&self, other: &Self) -> crate::domain::Unified<'a, #domain_name> {
                    if self == other {
                        crate::domain::Unified::Success
                    } else {
                        crate::domain::Unified::Failed
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
            #(#variant(crate::value::Val<#domain_type>)),*
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
