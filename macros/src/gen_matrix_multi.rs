use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, AttrStyle, Attribute, DeriveInput, Expr, Ident, LitInt, Path, Token};
use quote::quote;
use crate::backend::*;

pub(crate) fn gen_matrix_multi(input: &mut DeriveInput) -> TokenStream
{
    let name = &input.ident;
    
    let args = find_remove(&mut input.attrs, |a| is_attri(a, "mult_args"));
    let impls = args.iter().map(|a| multi_impl(attri_args(a).unwrap(), name));
    
    return quote! {
        #input
        #(#impls)*
    };
}

fn multi_impl(args: TokenStream, name: &Ident) -> TokenStream
{
    let args_parsed = Punctuated::<Expr, Token![,]>::parse_terminated
        .parse2(args)
        .unwrap();
    
    if args_parsed.len() != 5
    {
        panic!("Attribute must have 5 arguments.")
    }
    
    let row_li = expect_lit_int(&args_parsed[0]);
    let com_li = expect_lit_int(&args_parsed[1]);
    let col_li = expect_lit_int(&args_parsed[2]);
    let rhs_li = expect_path(&args_parsed[3]);
    let out_li = expect_path(&args_parsed[4]);
    
    let row = row_li.base10_parse::<usize>().unwrap();
    let com = com_li.base10_parse::<usize>().unwrap();
    let col = col_li.base10_parse::<usize>().unwrap();
    
    let rhs = &rhs_li.segments.last().unwrap().ident;
    let out = &out_li.segments.last().unwrap().ident;
    
    return quote! {
        impl<S: core::ops::Mul<Output = S> + core::ops::Add<Output = S> + Copy>
            core::ops::Mul<#rhs<S>> for #name<S>
        {
            type Output = #out<S>;
            
            #[inline]
            fn mul(self, rhs: #rhs<S>) -> Self::Output
            {
                // implement
            }
        }
    };
}

fn is_attri(attri: &Attribute, str: &str) -> bool
{
    match attri.style
    {
        AttrStyle::Inner(_) => false,
        AttrStyle::Outer =>
        {
            match &attri.meta
            {
                syn::Meta::List(m) =>
                {
                    let i = m.path.get_ident();
                    match i
                    {
                        None => false,
                        Some(i) =>
                        {
                            *i == Ident::new(str, i.span())
                        }
                    }
                },
                _ => false
            }
        }
    }
}
fn attri_args(attri: &Attribute) -> Option<TokenStream>
{
    match attri.style
    {
        AttrStyle::Inner(_) => None,
        AttrStyle::Outer =>
        {
            match &attri.meta
            {
                syn::Meta::List(m) => Some(m.tokens.clone()),
                _ => None
            }
        }
    }
}

fn find_remove<T, P>(vec: &mut Vec<T>, predicate: P) -> Vec<T>
    where P: Fn(&T) -> bool
{
    let mut map = vec![false; vec.len()];
    let mut filter = Vec::<T>::with_capacity(vec.len());
    
    let mut i = 0;
    for t in vec.iter()
    {
        if predicate(t)
        {
            map[i] = true;
        }
        
        i += 1;
    }
    
    // Backwards allows it to work
    for i in (0..vec.len()).rev()
    {
        if map[i]
        {
            let t = vec.swap_remove(i);
            filter.push(t);
        }
    }
    
    return filter;
}

fn expect_lit_int(expr: &Expr) -> &LitInt
{
    match expr
    {
        Expr::Lit(l) =>
        {
            match &l.lit
            {
                syn::Lit::Int(i) => &i,
                _ => panic!("Expected an integer argument")
            }
        },
        _ => panic!("Expected an integer argument")
    }
}

fn expect_path(expr: &Expr) -> &Path
{
    match expr
    {
        Expr::Path(p) => &p.path,
        _ => panic!("Expected a type argument")
    }
}