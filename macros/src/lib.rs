mod gen_vector;
mod gen_matrix;
mod backend;
use gen_vector::gen_vector;
use gen_matrix::gen_matrix;

use proc_macro::TokenStream;
// use zs_core::*;

#[proc_macro_attribute]
pub fn generate_vector(attr: TokenStream, item: TokenStream) -> TokenStream {
    return gen_vector(attr, item);
}

#[proc_macro_attribute]
pub fn generate_matrix(attr: TokenStream, item: TokenStream) -> TokenStream {
    return gen_matrix(attr, item);
}
