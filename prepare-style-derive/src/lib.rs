extern crate ordered_float;
extern crate proc_macro2;
extern crate proc_macro;
extern crate inflector;

#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

use syn::{Data, DeriveInput, GenericArgument, PathArguments, Type};
use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use inflector::Inflector;

#[derive(Debug, Clone)]
struct StructField {
  name: String,
  ftype: String,
}

fn get_struct_fields(ast: DeriveInput) -> (String, Vec<StructField>) {
  let mut fields: Vec<StructField> = vec![];

  if let Data::Struct(ref ast_struct) = ast.data {
    for field in ast_struct.fields.iter() {
      if let Type::Path(ref ast_field) = &field.ty {
        if let PathArguments::AngleBracketed(ref ast_args) = ast_field.path.segments[0].arguments {
          if let GenericArgument::Type(ref ast_arg) = ast_args.args[0] {
            if let Type::Path(ref final_type) = ast_arg {
              let type_name = &final_type.path.segments[0].ident.to_string();

              if let Some(name) = &field.ident {
                fields.push(StructField {
                  name: name.to_string(),
                  ftype: type_name.to_string(),
                });
              }
            }
          }
        }
      }
    }
  }

  (ast.ident.to_string(), fields)
}

fn generate_expression(field: StructField) -> proc_macro2::TokenStream {
  let ftype = field.ftype;
  let name = field.name;

  let value_block = Ident::new(&*format!("value_{}", &name), Span::call_site());
  let enum_type = Ident::new(&*name.to_pascal_case(), Span::call_site());
  let self_name = Ident::new(&*name, Span::call_site());

  let expression = quote! {
    if let Some(#value_block) = &self.#self_name {
      properties.push(FlexStyle::#enum_type(#value_block.clone()));
    }
  };

  expression
}

fn get_impl_trait_tokens(ast: DeriveInput) -> proc_macro2::TokenStream {
  let parsed = get_struct_fields(ast);
  
  let fields = parsed.1;
  let name = Ident::new(&*parsed.0, Span::call_site());

  let iter_expressions: Vec<proc_macro2::TokenStream> = fields
    .iter()
    .map(|f| generate_expression(f.clone()))
    .collect();
  
  quote! {
    impl PrepareStyleExt for #name {
      fn get_prepared_layout(&self) -> Vec<FlexStyle> {
        let mut properties = vec![];
        #(#iter_expressions)*
        properties
      }
    }
  }
}

#[proc_macro_derive(PrepareStyle)]
pub fn prepare_style(input: TokenStream) -> TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  let expanded = get_impl_trait_tokens(ast);
  expanded.into()
}