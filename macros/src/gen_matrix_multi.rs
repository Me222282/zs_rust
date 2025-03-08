use proc_macro2::{Span, TokenStream};
use syn::{parse::Parser, punctuated::Punctuated, DeriveInput, Ident, Token};
use quote::quote;
use crate::*;

pub(crate) fn gen_matrix_multi(input: &mut DeriveInput) -> TokenStream
{
    let name = &input.ident;
    
    let args = find_remove(&mut input.attrs, |a| is_attri(a, "mult_mat_args"));
    let impls = args.iter().map(|a| multi_impl(attri_args(a).unwrap(), name));
    
    return quote! {
        #(#impls)*
    };
}

fn multi_impl(args: TokenStream, name: &Ident) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse2(args)
        .unwrap();
    
    if args_parsed.len() != 4
    {
        panic!("Attribute must have 4 arguments.")
    }
    
    let row_li = args_parsed[0].expect_lit_int();
    let col_li = args_parsed[1].expect_lit_int();
    let rhs = args_parsed[2].expect_type();
    let out = args_parsed[3].expect_type();
    
    let row = row_li.base10_parse::<usize>().unwrap();
    // let com = com_li.base10_parse::<usize>().unwrap();
    let col = col_li.base10_parse::<usize>().unwrap();
    
    let code: Vec<_> = MatMulti::new(row, col).collect();
    
    let assign = if out.path.segments.last().unwrap().ident.eq(name)
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