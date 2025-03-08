use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, ItemStruct, Token};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix_square(attr: proc_macro::TokenStream, input: &ItemStruct) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap();
    
    if args_parsed.len() != 2
    {
        panic!("Attribute must have a size and vector argument.")
    }
    
    let size_li = args_parsed[0].expect_lit_int();
    let vec = args_parsed[1].expect_type();
    
    let size = size_li.base10_parse::<usize>().unwrap();
    
    let name = &input.ident;
    let mat = gen_matrix(quote! { #size_li, #size_li, #vec, #vec }.into(), input);
    
    let cols: Vec<_> = Dimension::new(size, "col").collect();
    
    return quote! {
        #mat
        
        impl<S: Copy> #name<S>
        {
            pub fn transpose(&self) -> Self
            {
                return Self::new(#(self.#cols()),*);
            }
        }
    };
}