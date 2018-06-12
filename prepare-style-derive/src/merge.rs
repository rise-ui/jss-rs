use common::{StructField, optioned_type, field_to_name_and_ty };
use proc_macro2::{Ident, TokenStream};
use syn::{DataStruct, Fields};

// @TODO: What if user style need override exists param?
// Maybe try setting override by prepared Vec<TargetEnum>?
fn generate_expression(field: StructField) -> TokenStream {
  let name = field.name;

  quote!{
    if let Some(_) = other.#name {
      result.#name = other.#name;
    }
  }
}

fn get_collect_expressions(ast_struct: DataStruct) -> impl Iterator<Item = TokenStream> {
  let fields = if let Fields::Named(x) = ast_struct.fields {
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

pub fn get_impl_trait_tokens(struct_id: Ident, data_struct: DataStruct) -> TokenStream {
  let expressions = get_collect_expressions(data_struct);

  quote! {
    use std::ops::Add;

    impl Add for #struct_id {
      type Output = #struct_id;

      fn add(self, other: #struct_id) -> #struct_id {
        let mut result = self.clone();
        #(#expressions)*
        result
      }
    }
  }
}
