use std::str::FromStr; 

use proc_macro2::{Ident, Span, TokenStream};
use syn::{AttrStyle, Attribute, Expr, LitInt};

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

pub(crate) fn vector_args(size: usize) -> Vec<Ident>
{
    return match size
    {
        s if s > 4 => Dimension::new(s, "i").collect(),
        4 => ident_vec!["x", "y", "z", "w"],
        3 => ident_vec!["x", "y", "z"],
        2 => ident_vec!["x", "y"],
        _ => panic!("Size must be 2 or greater.")
    };
}
pub(crate) fn vector_args_str(size: usize, pre: &str) -> Vec<Ident>
{
    return match size
    {
        s if s > 4 => Dimension::new(s, format!("{pre}i").as_str()).collect(),
        
        4 => ident_vec![format!("{pre}x").as_str(), format!("{pre}y").as_str(),
            format!("{pre}z").as_str(), format!("{pre}w").as_str()],
            
        3 => ident_vec![format!("{pre}x").as_str(), format!("{pre}y").as_str(),
            format!("{pre}z").as_str()],
            
        2 => ident_vec![format!("{pre}x").as_str(), format!("{pre}y").as_str()],
        
        _ => panic!("Size must be 2 or greater.")
    };
}

pub(crate) fn is_attri(attri: &Attribute, str: &str) -> bool
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
pub(crate) fn attri_args(attri: &Attribute) -> Option<TokenStream>
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
pub(crate) fn find_remove<T, P>(vec: &mut Vec<T>, predicate: P) -> Vec<T>
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