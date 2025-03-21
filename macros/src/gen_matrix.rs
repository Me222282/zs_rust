use std::panic;

use proc_macro2::{Ident, Span, TokenStream};
use syn::{parse::Parser, punctuated::Punctuated, ItemStruct, LitInt, Token, TypePath};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix(attr: proc_macro::TokenStream, input: &mut ItemStruct) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse(attr)
        .unwrap();
    
    let row_li: &LitInt;
    let col_li: &LitInt;
    let vec_col: TypePath;
    let vec_row: TypePath;
    
    let row: usize;
    let col: usize;
    
    if args_parsed.len() == 4
    {
        row_li = args_parsed[0].expect_lit_int();
        col_li = args_parsed[1].expect_lit_int();
        vec_col = args_parsed[2].expect_type();
        vec_row = args_parsed[3].expect_type();
        
        row = row_li.base10_parse::<usize>().unwrap();
        col = col_li.base10_parse::<usize>().unwrap();
    }
    else if args_parsed.len() == 2
    {
        row_li = args_parsed[0].expect_lit_int();
        col_li = row_li;
        vec_col = args_parsed[1].expect_type();
        vec_row = vec_col.clone();
        
        row = row_li.base10_parse::<usize>().unwrap();
        col = row;
    }
    else
    {
        panic!("Attribute must have a rows, columns and vectors argument for either 1 or 2 dimensions.");
    }
    
    let rows: Vec<_> = Dimension::new(row, "row").collect();
    let cols: Vec<_> = Dimension::new(col, "col").collect();
    
    // let x_nums: Vec<_> = Numbers::new(col).collect();
    let y_nums: Vec<_> = Numbers::new(row).collect();
    
    let range_start: Vec<_> = MatIndex::new(row, col, 0).collect();
    let range_end: Vec<_> = MatIndex::new(row, col, col).collect();
    
    let unit_one = Ident::new("one", Span::call_site());
    let unit_zero = Ident::new("zero", Span::call_site());
    let identity: Vec<_> = MatIdent::<Ident>::new(row, col, &unit_zero, &unit_one).collect();
    let col_grid: Vec<_> = GridInv::new(col, row).collect();
    
    let row_grid: Vec<_> = Grid::new(row, col).collect();
    
    // let vec_row = Ident::new(format!("Vector{row}").as_str(), Span::call_site());
    // let vec_col = Ident::new(format!("Vector{col}").as_str(), Span::call_site());
    
    let size = LitInt::new((row * col).to_string().as_str(), Span::call_site());
    
    // constructors
    let const_args = find_remove(&mut input.attrs, |a| is_attri(a, "matrix_constructors"));
    let const_impls = const_args.iter().map(|a|
    {
        rethrow_panic("Matrix constructor: ", ||
        {
            gen_matrix_con(attri_args(a).unwrap(), &input.ident, row, col)
        })
    });
    // square functions
    let square_args = find_remove(&mut input.attrs, |a| is_attri(a, "matrix_square"));
    if square_args.len() > 1
    {
        panic!("Cannot call square generation more than once.");
    }
    else if square_args.len() == 1 && row != col
    {
        panic!("Cannot call square generation on non-square matrices.");
    }
    let square_impls = square_args.iter().map(|a|
    {
        rethrow_panic("Matrix square: ", ||
        {
            gen_matrix_square(attri_args(a).unwrap(), &input.ident, row, &cols)
        })
    });
    
    // let input = parse_macro_input!(item as ItemStruct);
    let name = &input.ident;
    let attrs = &input.attrs;
    let vis = &input.vis;
    
    let assign_vec = if row == col
    {
        let args = vector_args(col);
        
        quote! {
            impl<S: num_traits::Num + Copy> core::ops::MulAssign<#name<S>> for #vec_col<S>
            {
                #[inline]
                fn mul_assign(&mut self, rhs: #name<S>)
                {
                    let r = #vec_col::<S>::new(
                        #(self.dot(rhs.#cols())),*
                    );
                    #(self.#args = r.#args);*
                }
            }
        }
    }
    else
    {
        TokenStream::new()
    };
    
    return quote! {
        #(#attrs)*
        #[derive(Clone, Copy, Debug, PartialEq)]
        #vis struct #name<S>
        {
            data: [[S; #col_li]; #row_li]
        }
        
        #(#const_impls)*
        #(#square_impls)*
        
        impl<S: Copy> #name<S>
        {
            #[inline]
            pub fn new(#(#rows: #vec_row<S>),*) -> Self
            {
                return Self {
                    data: [
                        #(#rows.into()),*
                    ]
                };
            }
            #(
            #[inline]
            pub fn #rows(&self) -> #vec_row<S>
            {
                return #vec_row::<S>::from(self.data[#y_nums]);
            })*
            
            #(
            #[inline]
            pub fn #cols(&self) -> #vec_col<S>
            {
                return #vec_col::<S>::new(
                    #(self[#col_grid]),*
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
        impl<S: Copy> std::convert::From<&[[S; #col_li]; #row_li]> for #name<S>
        {
            #[inline]
            fn from(value: &[[S; #col_li]; #row_li]) -> Self
            {
                return Self {
                    data: *value
                };
            }
        }
        impl<S: Copy> std::convert::From<&[&[S; #col_li]; #row_li]> for #name<S>
        {
            #[inline]
            fn from(value: &[&[S; #col_li]; #row_li]) -> Self
            {
                return Self {
                    data: [
                        #(*value[#y_nums]),*
                    ]
                };
            }
        }
        impl<S: Copy> std::convert::From<[[S; #col_li]; #row_li]> for #name<S>
        {
            #[inline]
            fn from(value: [[S; #col_li]; #row_li]) -> Self
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
        
        impl<S: core::ops::Add<Output = S> + Copy> core::ops::Add for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn add(self, rhs: Self) -> Self
            {
                return Self
                {
                    data: [#([#(self[#row_grid] + rhs[#row_grid]),*]),*]
                };
            }
        }
        impl<S: core::ops::Sub<Output = S> + Copy> core::ops::Sub for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn sub(self, rhs: Self) -> Self
            {
                return Self
                {
                    data: [#([#(self[#row_grid] - rhs[#row_grid]),*]),*]
                };
            }
        }
        impl<S: core::ops::Add<Output = S> + Copy> core::ops::Add<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn add(self, rhs: S) -> Self
            {
                return Self
                {
                    data: [#([#(self[#row_grid] + rhs),*]),*]
                };
            }
        }
        impl<S: core::ops::Sub<Output = S> + Copy> core::ops::Sub<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn sub(self, rhs: S) -> Self
            {
                return Self
                {
                    data: [#([#(self[#row_grid] - rhs),*]),*]
                };
            }
        }
        impl<S: core::ops::Mul<Output = S> + Copy> core::ops::Mul<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn mul(self, rhs: S) -> Self
            {
                return Self
                {
                    data: [#([#(self[#row_grid] * rhs),*]),*]
                };
            }
        }
        impl<S: core::ops::Div<Output = S> + num_traits::One + Copy> core::ops::Div<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn div(self, rhs: S) -> Self
            {
                let div = S::one() / rhs;
                return Self
                {
                    data: [#([#(self[#row_grid] * rhs),*]),*]
                };
            }
        }
        impl<S: core::ops::Rem<Output = S> + Copy> core::ops::Rem<S> for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn rem(self, rhs: S) -> Self
            {
                return Self
                {
                    data: [#([#(self[#row_grid] % rhs),*]),*]
                };
            }
        }
        impl<S: core::ops::AddAssign + Copy> core::ops::AddAssign for #name<S>
        {
            #[inline]
            fn add_assign(&mut self, rhs: Self)
            {
                #(#(self[#row_grid] += rhs[#row_grid]);*);*
            }
        }
        impl<S: core::ops::SubAssign + Copy> core::ops::SubAssign for #name<S>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: Self)
            {
                #(#(self[#row_grid] -= rhs[#row_grid]);*);*
            }
        }
        impl<S: core::ops::AddAssign + Copy> core::ops::AddAssign<S> for #name<S>
        {
            #[inline]
            fn add_assign(&mut self, rhs: S)
            {
                #(#(self[#row_grid] += rhs);*);*
            }
        }
        impl<S: core::ops::SubAssign + Copy> core::ops::SubAssign<S> for #name<S>
        {
            #[inline]
            fn sub_assign(&mut self, rhs: S)
            {
                #(#(self[#row_grid] -= rhs);*);*
            }
        }
        impl<S: core::ops::MulAssign + Copy> core::ops::MulAssign<S> for #name<S>
        {
            #[inline]
            fn mul_assign(&mut self, rhs: S)
            {
                #(#(self[#row_grid] *= rhs);*);*
            }
        }
        impl<S: core::ops::Div<Output = S> +
            core::ops::MulAssign + num_traits::One +
            Copy> core::ops::DivAssign<S> for #name<S>
        {
            #[inline]
            fn div_assign(&mut self, rhs: S)
            {
                let div = S::one() / rhs;
                #(#(self[#row_grid] *= rhs);*);*
            }
        }
        impl<S: core::ops::RemAssign + Copy> core::ops::RemAssign<S> for #name<S>
        {
            #[inline]
            fn rem_assign(&mut self, rhs: S)
            {
                #(#(self[#row_grid] %= rhs);*);*
            }
        }
        impl<S: core::ops::Neg<Output = S> + Copy> core::ops::Neg for #name<S>
        {
            type Output = Self;
            
            #[inline]
            fn neg(self) -> Self
            {
                return Self
                {
                    data: [#([#(-self[#row_grid]),*]),*]
                };
            }
        }
        
        impl<S: num_traits::Zero + PartialEq + Copy> num_traits::Zero for #name<S>
        {
            #[inline]
            fn zero() -> Self
            {
                return Self {
                    data: [[S::zero(); #col_li]; #row_li]
                };
            }
            #[inline]
            fn is_zero(&self) -> bool
            {
                return self == &Self::zero();
            }
        }
        
        impl<S: num_traits::Num + Copy> core::ops::Mul<#vec_row<S>> for #name<S>
        {
            type Output = #vec_col<S>;
            
            #[inline]
            fn mul(self, rhs: #vec_row<S>) -> Self::Output
            {
                return Self::Output::new(
                    #(rhs.dot(self.#rows())),*
                );
            }
        }
        
        impl<S: num_traits::Num + Copy> core::ops::Mul<#name<S>> for #vec_col<S>
        {
            type Output = #vec_row<S>;
            
            #[inline]
            fn mul(self, rhs: #name<S>) -> Self::Output
            {
                return Self::Output::new(
                    #(self.dot(rhs.#cols())),*
                );
            }
        }
        
        #assign_vec
        
    };
}