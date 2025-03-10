use std::cmp;

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, Ident, Token};
use quote::{quote, ToTokens};
use crate::*;

pub(crate) fn gen_matrix_con_scale(args: &Punctuated::<Arg, Token![,]>, name: &Ident, row: usize, col: usize) -> TokenStream
{
    if args.len() != 2
    {
        panic!("Attribute must have 1 type arguments.")
    }
    
    let vec = args[1].expect_type();
    
    let unit_one = quote! { scale };
    let unit_zero = quote! { S::zero() };
    let scale: Vec<_> = MatIdent::<_>::new(row, col, &unit_zero, &unit_one).collect();
    
    let min = cmp::min(row, col);
    
    let scale_names = vector_args_str(min, "scale_");
    let vec_args = vector_args(min);
    
    let sn_tokens: Vec<_> = scale_names.iter().map(ToTokens::to_token_stream).collect();
    let scale_v: Vec<_> = MatScale::<_, _>::new(row, col, &unit_zero, sn_tokens.iter()).collect();
    
    let scale_name_fn = ident_str![format!("create_scale_{min}").as_str()];
    
    return quote! {
        impl<S: num_traits::Zero + Copy> #name<S>
        {
            #[inline]
            pub fn create_scale(scale: S) -> Self
            {
                return Self {
                    data: [#([#(#scale),*]),*]
                };
            }
            #[inline]
            pub fn #scale_name_fn(#(#scale_names: S),*) -> Self
            {
                return Self {
                    data: [#([#(#scale_v),*]),*]
                };
            }
            #[inline]
            pub fn create_scale_v(scale: #vec<S>) -> Self
            {
                return Self::#scale_name_fn(#(scale.#vec_args),*);
            }
        }
    };
}