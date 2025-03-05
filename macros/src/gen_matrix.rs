use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use syn::{parse::Parser, parse_macro_input, punctuated::Punctuated, ItemStruct, LitInt, Token};
use quote::quote;
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
    
    let rows: Vec<_> = Dimension::new(row, "row").collect();
    let cols: Vec<_> = Dimension::new(col, "col").collect();
    
    let y_nums: Vec<_> = Numbers::new(row).collect();
    
    let range_start: Vec<_> = MatIndex::new(col, row, 0).collect();
    let range_end: Vec<_> = MatIndex::new(row, col, col).collect();
    
    let identity: Vec<_> = MatIdent::new(row, col).collect();
    let grid: Vec<_> = Grid::new(row, col).collect();
    
    let vec_row = Ident::new(format!("Vector{row}").as_str(), Span::call_site());
    let vec_col = Ident::new(format!("Vector{col}").as_str(), Span::call_site());
    
    let size = LitInt::new((row * col).to_string().as_str(), Span::call_site());
    
    let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;
    
    return quote! {
        #(#attrs)*
        #[derive(Clone, Copy, Debug, PartialEq)]
        #vis struct #name<S>
        {
            data: [[S; #row_li]; #col_li]
        }
        
        impl<S: Copy> #name<S>
        {
            #[inline]
            pub fn new(#(#rows: #vec_col<S>),*) -> Self
            {
                return Self {
                    data: [
                        #(#rows.into()),*
                    ]
                };
            }
            #(
            #[inline]
            pub fn #rows(&self) -> #vec_col<S>
            {
                return #vec_col::<S>::from(self.data[#y_nums]);
            })*
            
            #(
            #[inline]
            pub fn #cols(&self) -> #vec_row<S>
            {
                return #vec_row::<S>::new(
                    #(self[#grid]),*
                );
            })*
        }
        impl<S: num_traits::One + num_traits::Zero> #name<S>
        {
            #[inline]
            pub fn identity() -> Self
            {
                return Self {
                    data: [#([#(S::#identity()),*]),*]
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
        impl<S: Copy> std::convert::From<&[&[S; #row_li]; #col_li]> for #name<S>
        {
            #[inline]
            fn from(value: &[&[S; #row_li]; #col_li]) -> Self
            {
                return Self {
                    data: [
                        #(*value[#y_nums]),*
                    ]
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
        impl<S: Copy> std::convert::From<[S; #size]> for #name<S>
        {
            #[inline]
            fn from(value: [S; #size]) -> Self
            {
                return Self {
                    data: [
                        #(value[#range_start..#range_end].try_into().unwrap()),*
                    ]
                };
            }
        }
        impl<S: Copy> std::convert::From<&[S; #size]> for #name<S>
        {
            #[inline]
            fn from(value: &[S; #size]) -> Self
            {
                return Self {
                    data: [
                        #(value[#range_start..#range_end].try_into().unwrap()),*
                    ]
                };
            }
        }
        impl<S: Copy> std::ops::Index<usize> for #name<S>
        {
            type Output = S;
            
            #[inline]
            fn index(&self, index: usize) -> &S
            {
                if index >= #size
                {
                    panic!("index out of bounds");
                }
                let dat = &self.data[0][0];
                let r: &S;
                unsafe
                {
                    r = &*(dat as *const S).add(index);
                }
                return r;
            }
        }
        impl<S: Copy> std::ops::IndexMut<usize> for #name<S>
        {
            #[inline]
            fn index_mut(&mut self, index: usize) -> &mut S
            {
                if index >= #size
                {
                    panic!("index out of bounds");
                }
                let dat = &mut self.data[0][0];
                let r: &mut S;
                unsafe
                {
                    r = &mut *(dat as *mut S).add(index);
                }
                return r;
            }
        }
        impl<S: Copy> std::ops::Index<[usize; 2]> for #name<S>
        {
            type Output = S;
            
            #[inline]
            fn index(&self, index: [usize; 2]) -> &S
            {
                return &self.data[index[0]][index[1]];
            }
        }
        impl<S: Copy> std::ops::IndexMut<[usize; 2]> for #name<S>
        {
            #[inline]
            fn index_mut(&mut self, index: [usize; 2]) -> &mut S
            {
                return &mut self.data[index[0]][index[1]];
            }
        }
        
    }.into();
}