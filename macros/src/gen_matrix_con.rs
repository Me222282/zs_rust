use std::cmp;

use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, ItemStruct, LitInt, Token};
use quote::{quote, ToTokens};
use crate::*;

pub(crate) fn gen_matrix_con(attr: proc_macro::TokenStream, input: &ItemStruct) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap();
    
    let include_trans = args_parsed.len() == 2;
    if args_parsed.len() != 1 && !include_trans
    {
        panic!("Attribute must have either 1 or 2 type arguments.")
    }
    
    let vec = args_parsed[0].expect_type();
    
    let name = &input.ident;
    
    // get matrix size
    let ty = &input.fields;
    let ty = match ty
    {
        syn::Fields::Named(n) => n,
        _ => panic!("fields must be named")
    };
    let ty = &ty.named.first().unwrap().ty;
    let row_li: &LitInt;
    let ty = match ty
    {
        syn::Type::Array(a) =>
        {
            row_li = expect_lit_int(&a.len);
            
            a.elem.as_ref()
        }
        _ => panic!("invalid matrix fields")
    };
    let col_li = match ty
    {
        syn::Type::Array(a) => expect_lit_int(&a.len),
        _ => panic!("invalid matrix fields")
    };
    
    let row = row_li.base10_parse::<usize>().unwrap();
    let col = col_li.base10_parse::<usize>().unwrap();
    
    let unit_one = quote! { scale };
    let unit_zero = quote! { S::zero() };
    let scale: Vec<_> = MatIdent::<_>::new(row, col, &unit_zero, &unit_one).collect();
    
    let min = cmp::min(row, col);
    
    let scale_names = vector_args_str(min, "scale_");
    let vec_args = vector_args(min);
    
    let sn_tokens: Vec<_> = scale_names.iter().map(ToTokens::to_token_stream).collect();
    let scale_v: Vec<_> = MatScale::<_, _>::new(row, col, &unit_zero, sn_tokens.iter()).collect();
    
    let scale_name_fn = ident_str![format!("create_scale_{min}").as_str()];
    
    let trans_code = if include_trans
    {
        let vec_s = args_parsed[1].expect_type();
        
        let unit_one = quote! { S::one() };
        let trans: Vec<_> = MatIdent::<_>::new(row - 1, col, &unit_zero, &unit_one).collect();
        let vec_s_args = vector_args(col - 1);
        
        quote! {
            impl<S: num_traits::Zero + num_traits::One + Copy> #name<S>
            {
                #[inline]
                pub fn create_translation(offset: #vec_s<S>) -> Self
                {
                    return Self {
                        data: [
                            #([#(#trans),*]),*,
                            [#(offset.#vec_s_args),*, S::one()]
                        ]
                    };
                }
            }
        }
    }
    else
    {
        TokenStream::new()
    };
    
    return quote! {
        #input
        
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
        
        #trans_code
    };
}