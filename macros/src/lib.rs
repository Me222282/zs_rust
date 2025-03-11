mod gen_vector;
mod gen_matrix;
mod gen_matrix_square;
mod gen_matrix_multi;
mod gen_matrix_con;
mod backend;
mod backend_matrix;
use gen_vector::gen_vector;
use gen_matrix::gen_matrix;
use gen_matrix_square::gen_matrix_square;
use gen_matrix_multi::gen_matrix_multi;
use gen_matrix_con::gen_matrix_con;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};
// use zs_core::*;

pub(crate) use backend::*;
pub(crate) use backend_matrix::*;

#[proc_macro_attribute]
pub fn generate_vector(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let mut input = parse_macro_input!(item as ItemStruct);
    return gen_vector(attr, &mut input).into();
}

#[proc_macro_attribute]
pub fn generate_matrix(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let mut input = parse_macro_input!(item as ItemStruct);
    return gen_matrix(attr, &mut input).into();
}

#[proc_macro]
pub fn generate_matrix_multiply(args: TokenStream) -> TokenStream
{
    return gen_matrix_multi(args.into()).into();
}