use std::{cmp, iter};

use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, Ident, Token};
use quote::{quote, ToTokens};
use crate::*;

pub(crate) fn gen_matrix_con_scale(args: &Punctuated::<Arg, Token![,]>, name: &Ident, row: usize, col: usize) -> TokenStream
{   
    let min = cmp::min(row, col);
    
    let len = args.len();
    if len < 2 || len > (min * 2 - 1)
    {
        panic!("Attribute must have between 1 and {} type arguments with sizes.", min - 1)
    }
    
    let unit_one = quote! { scale };
    let unit_zero = quote! { S::zero() };
    let scale: Vec<_> = MatIdent::<_>::new(row, col, &unit_zero, &unit_one).collect();
    
    let mut fns = Vec::<TokenStream>::with_capacity(len / 2);
    
    if len > 2
    {
        let unit_one = quote! { S::one() };
        for x in (1..len).step_by(2)
        {
            let size = args[x].expect_lit_int().base10_parse::<usize>().unwrap();
            let vec = args[x + 1].expect_type();
            fns.push(con_scale_size(&vec, size, row, col, &unit_zero, &unit_one));
        }
    }
    else
    {   
        let vec = args[1].expect_type();
        fns.push(con_scale_size(&vec, min, row, col, &unit_zero, &unit_one));
    }
    
    return quote! {
        impl<S: num_traits::Zero + num_traits::One + Copy> #name<S>
        {
            #[inline]
            pub fn create_scale(scale: S) -> Self
            {
                return Self {
                    data: [#([#(#scale),*]),*]
                };
            }
            
            #(#fns)*
        }
    };
}

fn con_scale_size(vec: &syn::TypePath, size: usize,
    row: usize, col: usize, zero: &TokenStream, one: &TokenStream) -> TokenStream
{
    let scale_names = vector_args_str(size, "scale_");
    let vec_args = vector_args(size);
    
    let sn_tokens: Vec<_> = scale_names.iter().map(ToTokens::to_token_stream).collect();
    // incase size is less than min
    let iter = sn_tokens.iter().chain(iter::repeat(one));
    let scale_v: Vec<_> = MatScale::<_, _>::new(row, col, zero, iter).collect();
    
    let scale_name_fn = ident_str![format!("create_scale_{size}").as_str()];
    let scale_name_fn_v = ident_str![format!("create_scale_v{size}").as_str()];
    
    return quote! {
        #[inline]
        pub fn #scale_name_fn(#(#scale_names: S),*) -> Self
        {
            return Self {
                data: [#([#(#scale_v),*]),*]
            };
        }
        #[inline]
        pub fn #scale_name_fn_v(scale: #vec<S>) -> Self
        {
            return Self::#scale_name_fn(#(scale.#vec_args),*);
        }
    };
}