extern crate proc_macro;
extern crate syn;

mod into;
mod utils;

use into::into_impl;
use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(IntoPyObject)]
pub fn derive_into_py_object(input: TokenStream) -> TokenStream {
    let ast = syn::parse_macro_input!(input as DeriveInput);
    TokenStream::from(into_impl(ast))
}
