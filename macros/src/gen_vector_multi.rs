use proc_macro2::TokenStream;
use syn::{parse::Parser, punctuated::Punctuated, DeriveInput, Ident, Token, TypePath};
use quote::quote;
use crate::*;

pub(crate) fn gen_vector_multi(input: &mut DeriveInput) -> TokenStream
{
    let name = &input.ident;
    
    let size = &input.data;
    let size = match size
    {
        syn::Data::Struct(s) => s.fields.len(),
        _ => panic!("invalid item")
    };
    
    let args = find_remove(&mut input.attrs, |a| is_attri(a, "mult_vec_args"));
    let impls = args.iter().map(|a| multi_impl(attri_args(a).unwrap(), name, size));
    
    return quote! {
        #(#impls)*
    };
}

fn multi_impl(args: TokenStream, name: &Ident, size: usize) -> TokenStream
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
        let mut punc = Punctuated::<syn::PathSegment, Token![::]>::new();
        punc.push(syn::PathSegment {
            ident: name.clone(),
            arguments: syn::PathArguments::None
        });
        
        rhs = args_parsed[0].expect_type();
        out = TypePath {
            qself: None,
            path: syn::Path {
                leading_colon: None,
                segments: punc
            }
        };
        
        col = size;
    }
    else
    {
        panic!("Attribute must have 3 arguments.")
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
