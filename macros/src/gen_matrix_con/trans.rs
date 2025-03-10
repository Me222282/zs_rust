use proc_macro2::TokenStream;
use syn::{punctuated::Punctuated, Ident, Token, TypePath};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix_con_trans(args: &Punctuated::<Arg, Token![,]>, name: &Ident, row: usize, col: usize) -> TokenStream
{
    let vec_r: TypePath;
    let vec_c: TypePath;
    
    if args.len() == 2
    {
        if row != col
        {
            panic!("Attribute with 1 type argument must be square matrix.")
        }
        
        vec_r = args[1].expect_type();
        vec_c = vec_r.clone();
    }
    else if args.len() == 3
    {
        vec_r = args[1].expect_type();
        vec_c = args[2].expect_type();
    }
    else
    {
        panic!("Attribute must have 1 or 2 type arguments.")
    }
    
    let unit_zero = quote! { S::zero() };    
    let unit_one = quote! { S::one() };    
    
    #[cfg(feature = "reversed_matrix")]
    {
        let vec_args = vector_args(col);
        let trans: Vec<_> = MatIdent::<_>::new(row - 1, col, &unit_zero, &unit_one).collect();
        
        return quote! {
            impl<S: num_traits::Zero + num_traits::One + Copy> #name<S>
            {
                #[inline]
                pub fn create_translation(offset: #vec_c<S>) -> Self
                {
                    return Self {
                        data: [
                            #([#(#trans),*]),*,
                            [#(offset.#vec_args),*, S::one()]
                        ]
                    };
                }
            }
        };
    }
    #[cfg(not(feature = "reversed_matrix"))]
    {
        let vec_args = vector_args(row);
        let vec_args: Vec<_> = vec_args.iter().map(|i| quote! { offset.#i }).collect();
        
        let trans: Vec<_> = MatTrans::<_>::new(row, col, &unit_zero, &unit_one, vec_args.iter()).collect();
        
        return quote! {
            impl<S: num_traits::Zero + num_traits::One + Copy> #name<S>
            {
                #[inline]
                pub fn create_translation(offset: #vec_r<S>) -> Self
                {
                    return Self {
                        data: [#([#(#trans),*]),*]
                    };
                }
            }
        };
    }
}

#[cfg(not(feature = "reversed_matrix"))]
struct MatTrans<'a, I: Iterator<Item = &'a TokenStream>>
{
    pub rows: usize,
    pub cols: usize,
    i: usize,
    zero: &'a TokenStream,
    one: &'a TokenStream,
    values: I,
    iter: bool
}
#[cfg(not(feature = "reversed_matrix"))]
impl<'a, I: Iterator<Item = &'a TokenStream>> MatTrans<'a, I>
{
    pub fn new(rows: usize, cols: usize, zero: &'a TokenStream, one: &'a TokenStream, values: I) -> Self
    {
        return Self {
            rows,
            cols,
            i: 0,
            zero,
            one,
            values,
            iter: true
        };
    }
}
#[cfg(not(feature = "reversed_matrix"))]
impl<'a, I: Iterator<Item = &'a TokenStream>> Iterator for MatTrans<'a, I>
{
    type Item = Vec<&'a TokenStream>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.rows
        {
            let mut v = Vec::<&'a TokenStream>::with_capacity(self.cols);
            let end = self.cols - 1;
            for x in 0..self.cols
            {
                if x == ci
                {
                    if x == end
                    {
                        self.iter = false;
                    }
                    v.push(self.one);
                }
                else if x == end && self.iter
                {
                    let val = self.values.next().unwrap();
                    v.push(val);
                }
                else
                {
                    v.push(self.zero);
                }
            }
            
            return Some(v);
        }
        
        return None;
    }
}