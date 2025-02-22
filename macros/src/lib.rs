use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct, LitInt};
use quote::quote;
use proc_macro2::{Ident, Span};
use zs_core::*;

// macro_rules! ident_vec {
//     ($($x:expr),*) => {
//         vec![$(Ident::new($x, Span::call_site())),*]
//     };
// }
macro_rules! ident_vec {
    ($($x:expr),*) => {
        vec![$(Ident::new($x, Span::call_site())),*]
    };
}

#[proc_macro_attribute]
pub fn generate_vector(attr: TokenStream, item: TokenStream) -> TokenStream {
    let li = syn::parse::<LitInt>(attr).expect("Expected a numerial size.");
    let size = li.base10_parse::<usize>().unwrap();
    
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;
    
    let args = match size {
        s if s > 4 => Dimension { max: s, i: 0 }.collect(),
        4 => ident_vec!["x", "y", "z", "w"],
        3 => ident_vec!["x", "y", "z"],
        2 => ident_vec!["x", "y"],
        _ => panic!("Size must be 2 or greater.")
    };
    
    // let selfRef = 
    
    return quote! {
        #(#attrs)*
        #[derive(Clone, Copy)]
        #vis struct #name<S>
        {
            #(pub #args: S),*
        }
        
        impl<S: Copy> #name<S>
        {
            #[inline]
            pub fn new(#(#args: S),*) -> Self
            {
                return Self { #(#args),* };
            }
            #[inline]
            pub fn single(value: S) -> Self
            {
                return Self { #(#args: value),* };
            }
        }
        
        impl<S: num_traits::Num> #name<S>
            where Self: Copy
        {
            #[inline]
            pub fn dot(self, other: Self) -> S
            {
                return #(self.#args * other.#args)+* ;
            }
            #[inline]
            pub fn squared_length(self) -> S
            {
                return self.dot(self);
            }
            // #[inline]
            // pub fn squared_distance(self, other: Self) -> S
            // {
            //     return (other - self).squared_length();
            // }
        }
        impl<S: num_traits::Float> #name<S>
            where Self: Copy
        {
            #[inline]
            pub fn length(self) -> S
            {
                return self.squared_length().sqrt();
            }
            // #[inline]
            // pub fn distance(self, other: Self) -> S
            // {
            //     return self.distance(other).sqrt();
            // }
            #[inline]
            pub fn lerp(self, other: Self, blend: S) -> Self
            {
                return Self
                {
                    #(#args: (blend * (self.#args - other.#args)) + other.#args),*
                };
            }
        }
    }.into();
}

struct Dimension
{
    max: usize,
    i: usize
}
impl Iterator for Dimension {
    type Item = Ident;

    fn next(&mut self) -> Option<Self::Item>
    {   
        let ci = self.i;
        self.i += 1;
        if ci < self.max
        {
            let name = format!("i{ci}");
            return Some(Ident::new(name.as_str(), Span::call_site()));
        }
        
        return None;
    }
}