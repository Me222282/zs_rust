use proc_macro2::{Span, TokenStream};
use syn::{parse::Parser, punctuated::Punctuated, Ident, Token};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix_multi(args: TokenStream) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse2(args)
        .unwrap();
    
    let name: syn::TypePath;
    let rhs: syn::TypePath;
    let out: syn::TypePath;
    
    let row: usize;
    let col: usize;
    let assign: bool;
    
    match args_parsed.len()
    {
        5 => {
            let row_li = args_parsed[0].expect_lit_int();
            name = args_parsed[1].expect_type();
            rhs = args_parsed[2].expect_type();
            let col_li = args_parsed[3].expect_lit_int();
            out = args_parsed[4].expect_type();
            assign = false;
            
            col = col_li.base10_parse::<usize>().unwrap();
            row = row_li.base10_parse::<usize>().unwrap();
        },
        4 => {
            let row_li = args_parsed[0].expect_lit_int();
            name = args_parsed[1].expect_type();
            rhs = args_parsed[2].expect_type();
            let col_li = args_parsed[3].expect_lit_int();
            out = name.clone();
            assign = true;
            
            col = col_li.base10_parse::<usize>().unwrap();
            row = row_li.base10_parse::<usize>().unwrap();
        },
        2 => {
            let size_li = args_parsed[0].expect_lit_int();
            rhs = args_parsed[1].expect_type();
            name = rhs.clone(); 
            out = name.clone();
            assign = true;
            
            col = size_li.base10_parse::<usize>().unwrap();
            row = col;
        },
        _ => panic!("Attribute must have either 0, 2 or 3 arguments.")
    }
    
    let code: Vec<_> = MatMulti::new(row, col).collect();
    
    let assign = if assign
    {
        quote! {
            impl<S: num_traits::Num + Copy>
            core::ops::MulAssign<#rhs<S>> for #name<S>
            {
                #[inline]
                fn mul_assign(&mut self, rhs: #rhs<S>)
                {
                    self.data = [#([#(#code),*]),*];
                }
            }
        }
    }
    else
    {
        TokenStream::new()
    };
    
    return quote! {
        impl<S: num_traits::Num + Copy>
            core::ops::Mul<#rhs<S>> for #name<S>
        {
            type Output = #out<S>;
            
            #[inline]
            fn mul(self, rhs: #rhs<S>) -> Self::Output
            {
                return [
                    #([#(#code),*]),*
                ].into();
            }
        }
        #assign
    };
}

struct MatMulti
{
    rows: usize,
    cols: usize,
    i: usize
}
impl MatMulti
{
    pub fn new(rows: usize, cols: usize) -> Self
    {
        return Self {
            rows,
            cols,
            i: 0
        };
    }
}
impl Iterator for MatMulti
{
    type Item = Vec<TokenStream>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.rows
        {
            let mut v = Vec::<TokenStream>::with_capacity(self.cols);
            for x in 0..self.cols
            {
                let row = Ident::new(format!("row{ci}").as_str(), Span::call_site());
                let col = Ident::new(format!("col{x}").as_str(), Span::call_site());
                
                v.push(quote! {
                    self.#row().dot(rhs.#col())
                });
            }
            
            return Some(v);
        }
        
        return None;
    }
}