use std::str::FromStr;

use proc_macro2::{Ident, Span};
use syn::LitInt;

macro_rules! ident_vec {
    ($($x:expr),*) => {
        vec![$(Ident::new($x, Span::call_site())),*]
    };
}

pub(crate) use ident_vec;

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