use proc_macro::{TokenStream, TokenTree};
use proc_macro_error::abort;
use quote::{format_ident, quote};
use syn::{Data, Type};

use crate::tag::Tag;

fn ty(s: &str) -> Type {
    syn::parse_str(s).unwrap()
}

fn deduct_type(variants_no: u64) -> Type {
    match variants_no {
        0..=0xFF => ty("u8"),
        0x100..=0xFF_FF => ty("u16"),
        0x10000..=0xFF_FF_FF_FF => ty("u32"),
        0x100000000..=0xFF_FF_FF_FF_FF_FF_FF_FF => ty("u64"),
    }
}

fn parse_type(args: TokenStream, variants_no: u64) -> Option<Type> {
    let integral_tys: Vec<TokenTree> = args.into_iter().collect();

    match integral_tys.as_slice() {
        [integral_ty] => match integral_ty {
            TokenTree::Ident(i) => Some(match i.to_string().as_str() {
                actual_ty @ ("u8" | "u16" | "u32" | "u64" | "u128" | "i8" | "i16" | "i32"
                | "i64" | "i128" | "usize" | "isize") => ty(actual_ty),

                _ => return None,
            }),

            _ => None,
        },

        [_, ..] => None,

        [] => Some(deduct_type(variants_no)),
    }
}

pub fn integral_enum_impl(args: TokenStream, body: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(body as syn::DeriveInput);
    let tyname = input.ident.clone();

    match &input.data {
        Data::Enum(e) => {
            let mut tag = Tag::default();
            let Some(ty) = parse_type(args, e.variants.len() as u64) else {
                abort! {
                    input, "Expected single valid integral type or absence of it";
                    example = "#[integral_enum(u8)]"
                }
            };
            let definition = quote! {
                #[derive(
                    Debug, Clone, Copy, PartialEq, Eq,
                    PartialOrd, Ord
                )]
                #[repr(#ty)]
                #input
            };

            let mut c = 0_usize;
            let variants = e
                .variants
                .iter()
                .map(|v| {
                    if !v.fields.is_empty() {
                        abort! {
                            v, "Enum with fields are not supported"
                        }
                    }

                    let ident = v.ident.clone();
                    let tag = tag.process(v.discriminant.as_ref().map(|(_, ref e)| e));
                    let definition_ident = format_ident!("__{c}");
                    c += 1;

                    let definition = quote! {
                        const #definition_ident: #ty = #tag;
                    };
                    let match_arm = quote! {
                        #definition_ident => Ok(Self::#ident),
                    };

                    (definition, match_arm)
                })
                .collect::<Vec<_>>();
            let arms = variants.iter().map(|(_, arm)| arm);
            let def = variants.iter().map(|(def, _)| def);

            quote! {
                #definition

                impl ::core::convert::TryFrom<#ty> for #tyname {
                    type Error = ();

                    fn try_from(value: #ty) -> ::core::result::Result<
                        Self, <Self as ::core::convert::TryFrom<#ty>>::Error
                    >
                    {
                        #(#def)*
                        match value {
                            #(#arms)*
                            _ => Err(())
                        }
                    }
                }
            }
            .into()
        }

        _ => {
            abort! {
                input, "This proc macro is intended to be used on enums"
            }
        }
    }
}
