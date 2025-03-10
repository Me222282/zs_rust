use proc_macro2::TokenStream;
use syn::Ident;
use quote::quote;
// use crate::*;

pub(crate) fn gen_matrix_square(attr: TokenStream, name: &Ident, _size: usize, cols: &Vec<Ident>) -> TokenStream
{
    if !attr.is_empty()
    {
        panic!("Arguments should not be provided.");
    }
    
    // let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
    //     .parse2(attr)
    //     .unwrap();
    
    // if args_parsed.len() != 2
    // {
    //     panic!("Attribute must have a size and vector argument.")
    // }
    
    return quote! {
        impl<S: Copy> #name<S>
        {
            pub fn transpose(&self) -> Self
            {
                return Self::new(#(self.#cols()),*);
            }
        }
    };
}