use proc_macro::TokenStream;

use proc_macro2::Span;
use quote::quote;
use syn::*;

const GRID_SPACING: f32 = 250.0;
const HALFSIZE: f32 = GRID_SPACING / 2.0;

#[proc_macro_derive(Dimension)]
pub fn dimension_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_dimension_macro(&ast)
}

macro_rules! derive_error {
    ($string: tt) => {
        Error::new(Span::call_site(), $string)
            .to_compile_error()
            .into()
    };
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
