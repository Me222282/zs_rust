use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{parse::Parser, punctuated::Punctuated, ItemStruct, LitInt};
use quote::quote;
use crate::{backend::*, gen_matrix};

pub(crate) fn gen_matrix_square(attr: TokenStream, input: &ItemStruct) -> proc_macro2::TokenStream
{
    let li = syn::parse::<LitInt>(attr).expect("Expected a numerial size.");
    let size = li.base10_parse::<usize>().unwrap();
    
    let name = &input.ident;
    let mat = gen_matrix(quote! { #size, #size }.into(), input);
    
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