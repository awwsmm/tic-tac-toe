use proc_macro::{Group, TokenStream, TokenTree};

use proc_macro2::Span;
use quote::quote;
use syn::*;

const GRID_SPACING: f32 = 250.0;
const HALFSIZE: f32 = GRID_SPACING / 2.0;

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
}

#[proc_macro_derive(Dimension)]
pub fn dimension_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_dimension_macro(&ast)
}

fn impl_dimension_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        Data::Enum(data_enum) => {
            match data_enum.variants.len() {
                3 => {
                    let mut variants = data_enum.variants.iter();
                    let (first, second, third) = (variants.next().unwrap(), variants.next().unwrap(), variants.next().unwrap());

                    let gen = quote! {
                        impl #name {
                            fn values() -> [#name; 3] {
                                [ #name::#first, #name::#second, #name::#third ]
                            }

                            fn position(&self) -> i8 {
                                match self {
                                    #name::#first => -1,
                                    #name::#second => 0,
                                    #name::#third => 1
                                }
                            }

                            fn range(&self) -> Vec2 {
                                match self {
                                    #name::#first => Vec2::new(-3.0*#HALFSIZE, -#HALFSIZE),
                                    #name::#second => Vec2::new(-#HALFSIZE, #HALFSIZE),
                                    #name::#third => Vec2::new(#HALFSIZE, 3.0*#HALFSIZE),
                                }
                            }

                            fn in_range(&self, value: f32) -> bool {
                                let Vec2 { x: min, y: max } = self.range();
                                min <= value && value < max
                            }

                            fn containing(value: f32) -> Option<#name> {
                                if #name::#first.in_range(value) {
                                    Some(#name::#first)
                                } else if #name::#second.in_range(value) {
                                    Some(#name::#second)
                                } else if #name::#third.in_range(value) {
                                    Some(#name::#third)
                                } else {
                                    None
                                }
                            }
                        }
                    };

                    gen.into()
                }
                _ => return derive_error!("Dimension requires an enum with exactly three variants")
            }
        }
        _ => return derive_error!("Dimension is only implemented for enums")
    }
}

#[proc_macro_derive(Enumerated)]
pub fn enumerated_macro_derive(input: TokenStream) -> TokenStream {

    // filters "#[default]" out of enums, otherwise data.enum.variants.iter() has a hard time
    //
    // enum Difficulty {      enum Difficulty {
    //     Easy,                  Easy,
    //     Medium,        =>      Medium,
    //     #[default]             Hard,
    //     Hard,              }
    // }

    // uncomment to see TokenStream before this change
    // let file = syn::parse_file(&input.to_string()).unwrap();
    // println!("{}", prettyplease::unparse(&file));

    let token_stream = input.into_iter().map(|token| {
        match token {
            TokenTree::Group(group) => {
                let delimiter = group.delimiter();

                let tokens = group.stream().into_iter().flat_map(|token| {
                    match token {
                        TokenTree::Group(_) => None, // throw away group: [default]
                        TokenTree::Punct(p) if p.as_char() != ',' => None, // throw away punct: #
                        other => Some(other)
                    }
                }).collect::<TokenStream>();

                TokenTree::Group(Group::new(delimiter.into(), tokens.into()))
            }
            other => other
        }
    }).collect::<TokenStream>();

    // uncomment to see TokenStream after this change
    // let file = syn::parse_file(&token_stream.to_string()).unwrap();
    // println!("{}", prettyplease::unparse(&file));

    let ast = syn::parse(token_stream).unwrap();
    impl_enumerated_macro(&ast)
}

fn impl_enumerated_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    match &ast.data {
        Data::Enum(data_enum) => {
            let variants = data_enum.variants.iter().map(|each| quote!(#name::#each));
            let cardinality = variants.len();

            let gen = quote! {
                impl Enumerated for #name {
                    type Item = #name;

                    const CARDINALITY: usize = #cardinality;

                    fn variants() -> Vec<Self::Item> {
                        vec![#(#variants), *]
                    }
                }
            };

            gen.into()
        }
        _ => return derive_error!("Enumerated is only implemented for enums")
    }
}