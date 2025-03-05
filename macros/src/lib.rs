mod gen_vector;
mod gen_matrix;
mod gen_matrix_square;
mod backend;
use gen_vector::gen_vector;
use gen_matrix::gen_matrix;
use gen_matrix_square::gen_matrix_square;

use proc_macro::TokenStream;
use syn::{parse_macro_input, ItemStruct};
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
