use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{parse::Parser, parse_macro_input, punctuated::Punctuated, ItemStruct, LitInt, Token};
use quote::quote;
use zs_core::Num;
use crate::backend::*;

pub(crate) fn gen_matrix(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let args_parsed = Punctuated::<LitInt, Token![,]>::parse_terminated
        .parse2(attr.into())
        .unwrap();
    
    if args_parsed.len() != 2
    {
        panic!("Attribute must have a rows and columns argument.")
    }
    
    let row_li = &args_parsed[0];
    let col_li = &args_parsed[1];
    let row = row_li.base10_parse::<usize>().unwrap();
    let col = col_li.base10_parse::<usize>().unwrap();
    
    // let row_num: Vec<_> = Numbers { max: row, i: 0 }.collect();
    // let col_num: Vec<_> = Numbers { max: col, i: 0 }.collect();
    let rows: Vec<_> = Dimension::new(row, "row").collect();
    let cols: Vec<_> = Dimension::new(col, "col").collect();
    
    let vec_row = Ident::new(format!("Vector{row}").as_str(), Span::call_site());
    let vec_col = Ident::new(format!("Vector{col}").as_str(), Span::call_site());
    
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;
    
    return quote! {
        #(#attrs)*
        #[derive(Clone, Copy, Debug)]
        #vis struct #name<S>
        {
            data: [[S; #row_li]; #col_li]
        }
        
        impl<S: Copy> #name<S>
        {
            pub fn new(#(#rows: #vec_col<S>),*) -> Self
            {
                return Self {
                    data: [
                        #(#rows.into()),*
                    ]
                };
            }
        }
        impl<S: Copy> std::convert::From<&[[S; #row_li]; #col_li]> for #name<S>
        {
            #[inline]
            fn from(value: &[[S; #row_li]; #col_li]) -> Self
            {
                return Self {
                    data: *value
                };
            }
        }
        impl<S: Copy> std::convert::From<[[S; #row_li]; #col_li]> for #name<S>
        {
            #[inline]
            fn from(value: [[S; #row_li]; #col_li]) -> Self
            {
                return Self {
                    data: value
                };
            }
        }
        
        impl<S: PartialEq> PartialEq for #name<S>
        {
            #[inline]
            fn eq(&self, other: &Self) -> bool
            {
                return self.data == other.data;
            }
            #[inline]
            fn ne(&self, other: &Self) -> bool
            {
                return self.data != other.data;
            }
        }
    }.into();
}