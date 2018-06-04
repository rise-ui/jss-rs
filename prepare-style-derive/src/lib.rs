extern crate inflector;
extern crate ordered_float;
extern crate proc_macro2;
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate enum_extract;
#[macro_use]
extern crate quote;

use syn::{DeriveInput, GenericArgument, PathArguments, PathSegment, Type};
use proc_macro2::{Ident, Span};
use proc_macro::TokenStream;
use inflector::Inflector;
use syn::punctuated::Punctuated;

#[derive(Debug, Clone)]
struct StructField {
  name: String,
  ftype: String,
}

/// Checks whether a given field is an `Option<T>`, and returns its name with the underlying type
/// `T`.
fn optioned_type<T>(field_name: syn::Ident, segments: Punctuated<PathSegment, T>) -> Option<StructField> {
  if segments.len() != 1 {
    return None;
  }
  let ty = segments.into_iter().next().unwrap();
  if ty.ident != "Option" {
    return None;
  }
  extract!(PathArguments::AngleBracketed(_), ty.arguments)
    .and_then(|generics| generics.args.into_iter().next())
    .and_then(|arg| extract!(GenericArgument::Type(_), arg))
    .and_then(|ty| extract!(Type::Path(_), ty))
    .and_then(|ty_path| ty_path.path.segments.into_iter().next())
    .map(|segment| segment.ident)
    .map(|id| StructField {
      name: field_name.to_string(),
      ftype: id.to_string(),
    })
}

/// Checks if a named field's type is TypePath and returns its name and the path on success.
fn field_to_name_and_ty(field: syn::Field) -> Option<(syn::Ident, syn::Path)> {
  if let Type::Path(ty) = field.ty {
    Some((field.ident.unwrap(), ty.path))
  } else {
    None
  }
}

fn get_struct_fields(ast_struct: syn::DataStruct) -> Vec<StructField> {
  let fields = if let syn::Fields::Named(x) = ast_struct.fields {
    x.named
  } else {
    panic!("Only structs with named fields are supported.");
  };

  fields.into_iter().filter_map(field_to_name_and_ty).filter_map(|(name, type_path)| optioned_type(name, type_path.segments)).collect()
}

fn generate_expression(field: StructField) -> proc_macro2::TokenStream {
  let _ftype = field.ftype;
  let name = field.name;

  let value_block = Ident::new(&format!("value_{}", name), Span::call_site());
  let enum_type = Ident::new(&name.to_pascal_case(), Span::call_site());
  let self_name = Ident::new(&name, Span::call_site());

  let expression = quote! {
    if let Some(#value_block) = &self.#self_name {
      properties.push(FlexStyle::#enum_type(#value_block.clone()));
    }
  };

  expression
}

fn get_impl_trait_tokens(struct_name: &str, data_struct: syn::DataStruct) -> proc_macro2::TokenStream {
  let fields = get_struct_fields(data_struct);
  let name = Ident::new(struct_name, Span::call_site());

  let iter_expressions = fields.into_iter().map(generate_expression);

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
  match ast.data {
    syn::Data::Struct(data_struct) => get_impl_trait_tokens(&ast.ident.to_string(), data_struct).into(),
    _ => panic!("Only `struct`s could be derived using the macro"),
  }
}
