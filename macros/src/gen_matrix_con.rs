mod scale;
mod trans;
mod rotate;

use proc_macro2::TokenStream;
use scale::gen_matrix_con_scale;
use trans::gen_matrix_con_trans;
use rotate::gen_matrix_con_rotate;
use syn::{parse::Parser, punctuated::Punctuated, Ident, Token};
use crate::*;

pub(crate) fn gen_matrix_con(attr: TokenStream, name: &Ident, row: usize, col: usize) -> TokenStream
{
    let args_parsed = Punctuated::<Arg, Token![,]>::parse_terminated
        .parse2(attr)
        .unwrap();
    
    if args_parsed.len() < 1
    {
        panic!("Attribute must have constructor specifier argument.");
    }
    
    let opt = args_parsed[0].expect_option();
    
    if ident_eq!(opt, "scale")
    {
        return rethrow_panic("Scale: ", ||
        {
            gen_matrix_con_scale(&args_parsed, name, row, col)
        });
    }
    if ident_eq!(opt, "trans")
    {
        return rethrow_panic("Trans: ", ||
        {
            gen_matrix_con_trans(&args_parsed, name, row, col)
        });
    }
    if ident_eq!(opt, "rotate")
    {
        return rethrow_panic("Rotate: ", ||
        {
            gen_matrix_con_rotate(&args_parsed, name, row, col)
        });
    }
    
    panic!("Invalid option.");
}