mod gen_vector;
mod gen_matrix;
mod gen_matrix_square;
mod gen_matrix_multi;
mod backend;
use gen_vector::gen_vector;
use gen_matrix::gen_matrix;
use gen_matrix_square::gen_matrix_square;
use gen_matrix_multi::gen_matrix_multi;

use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput, ItemStruct};
// use zs_core::*;

#[proc_macro_attribute]
pub fn generate_vector(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(item as ItemStruct);
    return gen_vector(attr, &input).into();
}

#[proc_macro_attribute]
pub fn generate_matrix(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(item as ItemStruct);
    return gen_matrix(attr, &input).into();
}
#[proc_macro_attribute]
pub fn generate_matrix_square(attr: TokenStream, item: TokenStream) -> TokenStream
{
    let input = parse_macro_input!(item as ItemStruct);
    return gen_matrix_square(attr, &input).into();
}

#[proc_macro_derive(Mul, attributes(mult_args))]
pub fn generate_matrix_multiply(input: TokenStream) -> TokenStream
{
    let mut input = parse_macro_input!(input as DeriveInput);
    return gen_matrix_multi(&mut input).into();
}
