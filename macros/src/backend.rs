use std::str::FromStr; 

use proc_macro2::{Ident, Span};
use syn::{Expr, LitInt};

macro_rules! ident_vec {
    ($($x:expr),*) => {
        vec![$(proc_macro2::Ident::new($x, proc_macro2::Span::call_site())),*]
    };
}

pub(crate) use ident_vec;

macro_rules! ident_str {
    ($x:expr) => {
        proc_macro2::Ident::new($x, proc_macro2::Span::call_site())
    };
}

pub(crate) use ident_str;

pub(crate) struct Dimension
{
    max: usize,
    i: usize,
    str: String
}
impl Dimension
{
    pub fn new(max: usize, str: &str) -> Self
    {
        return Self {
            max,
            i: 0,
            str: String::from_str(str).unwrap()
        };
    }
}
impl Iterator for Dimension
{
    type Item = Ident;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.max
        {
            let name = self.str.clone() + &ci.to_string();
            return Some(Ident::new(name.as_str(), Span::call_site()));
        }
        
        return None;
    }
}

pub(crate) struct Numbers
{
    pub max: usize,
    pub i: usize
}
impl Numbers
{
    pub fn new(max: usize) -> Self
    {
        return Self {
            max,
            i: 0
        };
    }
}
impl Iterator for Numbers
{
    type Item = LitInt;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.max
        {
            let index = ci.to_string();
            return Some(LitInt::new(index.as_str(), Span::call_site()));
        }
        
        return None;
    }
}

pub(crate) enum Arg
{
    Lit(LitInt),
    Type(syn::TypePath)
}
impl Arg
{
    pub fn expect_lit_int(&self) -> &LitInt
    {
        match self
        {
            Arg::Lit(l) => &l,
            _ => panic!("Expected an integer argument")
        }
    }

    pub fn expect_type(&self) -> &syn::TypePath
    {
        match self
        {
            Arg::Type(t) => &t,
            _ => panic!("Expected a type argument")
        }
    }
}
impl syn::parse::Parse for Arg
{
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self>
    {
        let r = LitInt::parse(input);
        match r
        {
            Ok(d) => Ok(Arg::Lit(d)),
            Err(_) =>
            {
                let r = syn::TypePath::parse(input);
                match r
                {
                    Ok(d) => Ok(Arg::Type(d)),
                    Err(e) => Err(e)
                }
            }
        }
    }
}

pub(crate) fn expect_lit_int(expr: &Expr) -> &LitInt
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