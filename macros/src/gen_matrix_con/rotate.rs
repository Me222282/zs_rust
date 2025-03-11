use std::cmp;

use proc_macro2::{Span, TokenStream};
use syn::{punctuated::Punctuated, Ident, Token};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix_con_rotate(args: &Punctuated::<Arg, Token![,]>, name: &Ident, row: usize, col: usize) -> TokenStream
{
    let min: usize;
    
    if args.len() == 2
    {
        let li = args[1].expect_lit_int();
        min = li.base10_parse::<usize>().unwrap();
        
        if min > cmp::min(row, col)
        {
            panic!("Provided size is too large.")
        }
    }
    else if args.len() == 1
    {
        min = cmp::min(row, col);
    }
    else
    {
        panic!("Attribute must have either 0 or 1 size arguments.")
    }
    
    let comb = gen_combinations(min);
    let vec_args = vector_args(min);
    
    let fn_names: Vec<_> = RotNames::new(comb.iter(), &vec_args).collect();
    
    let unit_one = quote! { S::one() };
    let unit_zero = quote! { S::zero() };
    let sin = quote! { angle.sin() };
    let n_sin = quote! { -angle.sin() };
    let cos = quote! { angle.cos() };
    let rotate: Vec<_> = MatRot::new(comb.iter(), row, col,
        &unit_zero, &unit_one, &sin, &n_sin, &cos).collect();
    
    return quote! {
        impl<S: num_traits::Float + Copy> #name<S>
        {
            #(
            #[inline]
            pub fn #fn_names(angle: S) -> Self
            {
                return Self {
                    data: [#([#(#rotate),*]),*]
                };
            }
            )*
        }
    };
}

fn gen_combinations(size: usize) -> Vec<(usize, usize)>
{
    let num = (size * (size - 1)) / 2;
    let mut vec = Vec::<(usize, usize)>::with_capacity(num);
    
    for a in 0..size
    {
        for b in (a + 1)..size
        {
            vec.push((a, b));
        }
    }
    
    return vec;
}


pub(crate) struct MatRot<'a, T, I: Iterator<Item = &'a (usize, usize)>>
{
    pub comb: I,
    pub rows: usize,
    pub cols: usize,
    zero: &'a T,
    one: &'a T,
    sin: &'a T,
    n_sin: &'a T,
    cos: &'a T
}
impl<'a, T, I: Iterator<Item = &'a (usize, usize)>> MatRot<'a, T, I>
{
    pub fn new(comb: I, rows: usize, cols: usize,
        zero: &'a T, one: &'a T, sin: &'a T, n_sin: &'a T, cos: &'a T) -> Self
    {
        return Self {
            comb,
            rows,
            cols,
            zero,
            one,
            sin,
            n_sin,
            cos
        };
    }
}
impl<'a, T, I: Iterator<Item = &'a (usize, usize)>> Iterator for MatRot<'a, T, I>
{
    type Item = Vec<Vec<&'a T>>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let comb = self.comb.next();
        match comb
        {
            Some(c) =>
            {
                let mut v = Vec::<Vec<&'a T>>::with_capacity(self.rows);
                for y in 0..self.rows
                {
                    v.push(Vec::<&'a T>::with_capacity(self.cols));
                    
                    for x in 0..self.cols
                    {
                        let item = if (x == c.0 && y == c.0) ||
                            (x == c.1 && y == c.1)
                        {
                            self.cos
                        }
                        else if x == c.1 && y == c.0
                        {
                            self.n_sin
                        }
                        else if x == c.0 && y == c.1
                        {
                            self.sin
                        }
                        else if x == y { self.one }
                        else { self.zero };
                        
                        v[y].push(item);
                    }
                }
                return Some(v);
            },
            None => None
        }
    }
}

pub(crate) struct RotNames<'a, I: Iterator<Item = &'a(usize, usize)>>
{
    comb: I,
    names: &'a Vec<Ident>
}
impl<'a, I: Iterator<Item = &'a (usize, usize)>> RotNames<'a, I>
{
    pub fn new(comb: I, names: &'a Vec<Ident>) -> Self
    {
        return Self {
            comb,
            names
        };
    }
}
impl<'a, I: Iterator<Item = &'a (usize, usize)>> Iterator for RotNames<'a, I>
{
    type Item = Ident;

    fn next(&mut self) -> Option<Self::Item>
    {
        let comb = self.comb.next();
        match comb
        {
            Some(c) =>
            {
                let a = &self.names[c.0];
                let b = &self.names[c.1];
                let mut str = "create_rotation_".to_owned();
                str.push_str(&a.to_string());
                str.push_str(&b.to_string());
                return Some(Ident::new(str.as_str(), Span::call_site()));
            },
            None => None
        }
    }
}