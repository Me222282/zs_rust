use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, Ident, Token, TypePath};
use quote::quote;
use crate::*;

pub(crate) fn gen_vector_multi(args: TokenStream, name: &Ident, size: usize) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse2(args)
        .unwrap();
    
    let rhs: &TypePath;
    let out: TypePath;
    
    let col: usize;
    
    if args_parsed.len() == 3
    {
        let col_li = args_parsed[0].expect_lit_int();
        rhs = args_parsed[1].expect_type();
        out = args_parsed[2].expect_type().clone();
        
        col = col_li.base10_parse::<usize>().unwrap();
    }
    else if args_parsed.len() == 1
    {
        rhs = args_parsed[0].expect_type();
        out = ident_type_path(name.clone());
        
        col = size;
    }
    else
    {
        panic!("Attribute must have either a type argument or a size and 2 types.")
    }
    
    let cols: Vec<_> = Dimension::new(col, "col").collect();
    
    let assign = if out.path.segments.last().unwrap().ident.eq(name)
    {
        let args = vector_args(col);
        
        quote! {
            impl<S: num_traits::Num + Copy> core::ops::MulAssign<#rhs<S>> for #name<S>
            {
                #[inline]
                fn mul_assign(&mut self, rhs: #rhs<S>)
                {
                    let r = #out::<S>::new(
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
        impl<S: num_traits::Num + Copy> core::ops::Mul<#rhs<S>> for #name<S>
        {
            type Output = #out<S>;
            
            #[inline]
            fn mul(self, rhs: #rhs<S>) -> Self::Output
            {
                return Self::Output::new(
                    #(self.dot(rhs.#cols())),*
                );
            }
        }
        #assign
    };
}
