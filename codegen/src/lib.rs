extern crate proc_macro;
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::parse::{Parse, ParseStream};
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Result, Token};

struct DomainDef {
    domain_visibility: syn::Visibility,
    domain_name: syn::Ident,
    domain_types: Vec<syn::Type>,
}

mod kw {
    syn::custom_keyword!(domain);
}

struct Defs {
    defs: Vec<DomainDef>,
}
impl Parse for Defs {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut defs = Vec::new();

        while !input.is_empty() {
            if input.peek(kw::domain) || input.peek2(kw::domain) {
                defs.push(input.parse()?);
            }
        }

        Ok(Defs { defs })
    }
}

impl Parse for DomainDef {
    fn parse(input: ParseStream) -> Result<Self> {
        let domain_visibility = input.parse()?;
        input.parse::<kw::domain>()?;
        // input.parse::<proc_macro2::Delimiter>()?;
        let domain_name: syn::Ident = input.parse()?;
        let content;
        syn::braced!(content in input);
        let raw_types: Punctuated<syn::Type, Token![,]> =
            content.parse_terminated(syn::Type::parse)?;
        let domain_types: Vec<_> = raw_types.into_iter().collect();

        Ok(DomainDef {
            domain_visibility,
            domain_name,
            domain_types,
        })
    }
}

impl quote::ToTokens for DomainDef {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let DomainDef {
            domain_visibility,
            domain_name,
            domain_types,
        } = self;

        let (fields, variants): (Vec<_>, Vec<_>) = (0..domain_types.len())
            .into_iter()
            .map(|n| (format_ident!("t{}", n), format_ident!("T{}", n)))
            .unzip();

        let value_name = format_ident!("{}Value", domain_name);

        let result = quote! {
            #[derive(std::fmt::Debug)]
            #domain_visibility struct #domain_name {
                #(#fields: im::HashMap<canrun::value::LVar<#domain_types>, canrun::value::Val<#domain_types>>),*
            }

            impl<'a> canrun::domain::Domain<'a> for #domain_name {
                type Value = #value_name;
                fn new() -> Self {
                    #domain_name {
                        #(#fields: im::HashMap::new(),)*
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
                            (#value_name::#variants(a), #value_name::#variants(b)) => {
                                state.unify::<#domain_types, Val<#domain_types>, Val<#domain_types>>(a, b)
                            }
                        ,)*
                        _ => None, // This should only happen if a DomainVal constructor allows two values with different types.
                    }
                }
            }

            #(
                impl<'a> canrun::domain::IntoDomainVal<'a, #domain_types> for #domain_name {
                    fn into_domain_val(val: canrun::value::Val<#domain_types>) -> #value_name {
                        #value_name::#variants(val)
                    }
                }
            )*

            #(
                impl<'a> canrun::domain::DomainType<'a, #domain_types> for #domain_name {
                    fn values_as_ref(
                        &self,
                    ) -> &im::HashMap<canrun::value::LVar<#domain_types>, canrun::value::Val<#domain_types>> {
                        &self.#fields
                    }
                    fn values_as_mut(
                        &mut self,
                    ) -> &mut im::HashMap<canrun::value::LVar<#domain_types>, canrun::value::Val<#domain_types>> {
                        &mut self.#fields
                    }
                }
            )*

            #(
                impl<'a> canrun::domain::UnifyIn<'a, #domain_name> for #domain_types {
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
                        #(#fields: self.#fields.clone()),*
                    }
                }
            }

            #[derive(std::fmt::Debug)]
            #domain_visibility enum #value_name {
                #(#variants(canrun::value::Val<#domain_types>)),*
            }

            impl Clone for #value_name {
                fn clone(&self) -> Self {
                    match self {
                        #(#value_name::#variants(val) => #value_name::#variants(val.clone())),*
                    }
                }
            }
        };
        result.to_tokens(tokens);
    }
}

#[proc_macro]
pub fn domains(item: TokenStream) -> TokenStream {
    let Defs { defs } = parse_macro_input!(item as Defs);
    quote!(#(#defs)*).into()
}
