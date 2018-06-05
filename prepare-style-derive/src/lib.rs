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
use proc_macro2::{Ident, TokenStream, Span};
use inflector::Inflector;
use syn::punctuated::Punctuated;

/// A field description.
#[derive(Debug, Clone)]
struct StructField {
  name: Ident,
  /// Underlying type of the original field (if the field was `Option<T>`, this would be `T).
  ftype: Ident,
}

/// Checks whether a given field is an `Option<T>` and returns it as a field description on success.
fn optioned_type<T>(field_name: Ident, segments: Punctuated<PathSegment, T>) -> Option<StructField> {
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
    .map(|ftype| StructField {
      name: field_name,
      ftype,
    })
}

/// Checks if a named field's type is TypePath and returns its name and the path on success.
fn field_to_name_and_ty(field: syn::Field) -> Option<(Ident, syn::Path)> {
  if let Type::Path(ty) = field.ty {
    field.ident.map(|id| (id, ty.path))
  } else {
    None
  }
}

fn get_struct_fields(ast_struct: syn::DataStruct) -> impl Iterator<Item = TokenStream> {
  let fields = if let syn::Fields::Named(x) = ast_struct.fields {
    x.named
  } else {
    panic!("Only structs with named fields are supported.");
  };

  fields
    .into_iter()
    .filter_map(field_to_name_and_ty)
    .filter_map(|(name, type_path)| optioned_type(name, type_path.segments))
    .map(generate_expression)
}

fn generate_expression(field: StructField) -> TokenStream {
  let _ftype = field.ftype;
  let name = field.name;

  let value_block = Ident::new(&format!("value_{}", name), Span::call_site());
  let enum_type = Ident::new(&name.to_string().to_pascal_case(), Span::call_site());

  quote! {
    if let Some(#value_block) = &self.#name {
      properties.push(FlexStyle::#enum_type(#value_block.clone()));
    }
  }
}

fn get_impl_trait_tokens(struct_id: Ident, data_struct: syn::DataStruct) -> TokenStream {
  let expressions = get_struct_fields(data_struct);

  quote! {
    impl PrepareStyleExt for #struct_id {
      fn get_prepared_layout(&self) -> Vec<FlexStyle> {
        let mut properties = vec![];
        #(#expressions)*
        properties
      }
    }
  }
}

#[proc_macro_derive(PrepareStyle)]
pub fn prepare_style(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let ast: DeriveInput = syn::parse(input).unwrap();
  if let syn::Data::Struct(data_struct) = ast.data {
    get_impl_trait_tokens(ast.ident, data_struct).into()
  } else {
    panic!("Only `struct`s could be derived using the macro")
  }
}
