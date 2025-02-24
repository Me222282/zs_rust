mod gen_vector;
mod backend;
use gen_vector::gen_vector;

use proc_macro::TokenStream;
// use zs_core::*;

#[proc_macro_attribute]
pub fn generate_vector(attr: TokenStream, item: TokenStream) -> TokenStream {
    return gen_vector(attr, item);
}
