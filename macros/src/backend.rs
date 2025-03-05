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

pub(crate) struct MatIndex
{
    pub count: usize,
    pub i: usize,
    pub size: usize,
    pub offset: usize
}
impl MatIndex
{
    pub fn new(s1: usize, s2: usize, offset: usize) -> Self
    {
        return Self {
            count: s1,
            i: 0,
            size: s2,
            offset
        };
    }
}
impl Iterator for MatIndex
{
    type Item = LitInt;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.count
        {
            let value = (ci * self.size + self.offset).to_string();
            return Some(LitInt::new(value.as_str(), Span::call_site()));
        }
        
        return None;
    }
}

pub(crate) struct MatIdent
{
    pub rows: usize,
    pub cols: usize,
    i: usize,
    
}
impl MatIdent
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
impl Iterator for MatIdent
{
    type Item = Vec<Ident>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.rows
        {
            let mut v = Vec::<Ident>::with_capacity(self.cols);
            for x in 0..self.cols
            {
                let value = if x == ci { "one" } else { "zero" };
                let li = Ident::new(value, Span::call_site());
                v.push(li);
            }
            return Some(v);
        }
        
        return None;
    }
}
pub(crate) struct Grid
{
    pub rows: usize,
    pub cols: usize,
    i: usize,
    
}
impl Grid
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
impl Iterator for Grid
{
    type Item = Vec<LitInt>;

    fn next(&mut self) -> Option<Self::Item>
    {
        let ci = self.i;
        self.i += 1;
        if ci < self.rows
        {
            let mut v = Vec::<LitInt>::with_capacity(self.cols);
            for x in 0..self.cols
            {
                let value = (ci + x * self.rows).to_string();
                let li = LitInt::new(value.as_str(), Span::call_site());
                v.push(li);
            }
            return Some(v);
        }
        
        return None;
    }
}