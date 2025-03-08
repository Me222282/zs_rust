use proc_macro2::TokenStream;
use syn::{ItemStruct, LitInt, TypePath};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix_con(attr: proc_macro::TokenStream, input: &ItemStruct) -> TokenStream
{
    let vec = syn::parse::<TypePath>(attr).expect("Expected a type.");
    
    let name = &input.ident;
    
    // get matrix size
    let ty = &input.fields;
    let ty = match ty
    {
        syn::Fields::Named(n) => n,
        _ => panic!("fields sure be named")
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
    let scale: Vec<_> = MatIdent::<TokenStream>::new(row, col, &unit_zero, &unit_one).collect();
    
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
            // #[inline]
            // pub fn create_scale_2(scale_x: S, scale_y: S) -> Self
            // {
            //     return Self {
            //         data: [[scale_x, S::zero()], [S::zero(), scale_y]]
            //     };
            // }
            // #[inline]
            // pub fn create_scale_v(scale: Vector2<S>) -> Self
            // {
            //     return Self::create_scale_2(scale.x, scale.y);
            // }
        }
    };
}