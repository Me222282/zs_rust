use proc_macro2::TokenStream;
use syn::{ItemStruct, LitInt};
use quote::quote;
use crate::{backend::*, gen_matrix};

pub(crate) fn gen_matrix_square(attr: proc_macro::TokenStream, input: &ItemStruct) -> proc_macro2::TokenStream
{
    let li = syn::parse::<LitInt>(attr).expect("Expected a numerial size.");
    let size = li.base10_parse::<usize>().unwrap();
    
    let name = &input.ident;
    let mat = gen_matrix(quote! { #size, #size }.into(), input);
    
    let cols: Vec<_> = Dimension::new(size, "col").collect();
    
    let unit_one = quote! { scale };
    let unit_zero = quote! { S::zero() };
    let scale: Vec<_> = MatIdent::<TokenStream>::new(size, size, &unit_zero, &unit_one).collect();
    
    return quote! {
        #mat
        
        impl<S: Copy> #name<S>
        {
            pub fn transpose(&self) -> Self
            {
                return Self::new(#(self.#cols()),*);
            }
        }
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