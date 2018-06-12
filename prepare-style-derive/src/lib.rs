#![recursion_limit = "128"]

extern crate inflector;
extern crate ordered_float;
extern crate proc_macro;
extern crate proc_macro2;
extern crate regex;
extern crate syn;

#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate enum_extract;
#[macro_use]
extern crate quote;

mod common;
mod merge;
mod prepare;

use syn::DeriveInput;

#[proc_macro_derive(PrepareStyle)]
pub fn prepare_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  if let syn::Data::Struct(data_struct) = ast.data {
    prepare::get_impl_trait_tokens(ast.ident, data_struct).into()
  } else {
    panic!("Only `struct`s could be derived using the macro")
  }
}

#[proc_macro_derive(MergeStyle)]
pub fn merge_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  if let syn::Data::Struct(data_struct) = ast.data {
    merge::get_impl_trait_tokens(ast.ident, data_struct).into()
  } else {
    panic!("Only `struct`s could be derived using the macro")
  }
}
