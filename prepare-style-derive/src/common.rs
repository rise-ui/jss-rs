use syn::{GenericArgument, Field, Path, PathArguments, PathSegment, Type};
use proc_macro2::Ident;
use syn::punctuated::Punctuated;

/// A field description.
#[derive(Debug, Clone)]
pub struct StructField {
  pub name: Ident,
  /// Underlying type of the original field (if the field was `Option<T>`, this would be `T).
  pub ftype: Ident,
}

/// Checks whether a given field is an `Option<T>` and returns it as a field description on success.
pub fn optioned_type<T>(field_name: Ident, segments: Punctuated<PathSegment, T>) -> Option<StructField> {
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
pub fn field_to_name_and_ty(field: Field) -> Option<(Ident, Path)> {
  if let Type::Path(ty) = field.ty {
    field.ident.map(|id| (id, ty.path))
  } else {
    None
  }
}
