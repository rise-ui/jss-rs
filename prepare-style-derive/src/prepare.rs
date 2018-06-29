use common::{StructField, optioned_type, field_to_name_and_ty };
use proc_macro2::{Ident, Span, TokenStream};
use syn::{DataStruct, Fields};
use inflector::Inflector;
use regex::Regex;

lazy_static! {
  static ref RADIUS_EDGE_RE: Regex = Regex::new(r"^border_(?P<edge>\w+)_radius$").unwrap();
  static ref STYLE_EDGE_RE: Regex = Regex::new(r"^border_(?P<edge>\w+)_style$").unwrap();
  static ref COLOR_EDGE_RE: Regex = Regex::new(r"^border_(?P<edge>\w+)_color$").unwrap();
}

fn generate_expression(field: StructField) -> TokenStream {
  let ftype = field.ftype;
  let name = field.name;

  let value_block = Ident::new(&format!("value_{}", name), Span::call_site());
  let enum_type = Ident::new(&name.to_string().to_pascal_case(), Span::call_site());

  let type_str = &*ftype.to_string();
  let name_str = &*name.to_string();

  match type_str {
    "Background" => {
      quote! {
        if let Some(#value_block) = &self.#name {
          apperance.background = Some(#value_block.clone());
        }
      }
    }

    "BorderStyle" => {
      let edge = STYLE_EDGE_RE.replace_all(name_str, "$edge");
      let edge = edge.into_owned();
      let edge = Ident::new(&*edge, Span::call_site());

      quote! {
        if let Some(#value_block) = &self.#name {
          border_styles.#edge.style = #value_block.clone();
        }
      }
    }

    "Color" => {
      let edge = COLOR_EDGE_RE.replace_all(name_str, "$edge");
      let edge = edge.into_owned();
      let edge = Ident::new(&*edge, Span::call_site());

      quote! {
        if let Some(#value_block) = &self.#name {
          border_styles.#edge.color = #value_block.clone();
        }
      }
    }

    "i32" => {
      let edge = RADIUS_EDGE_RE.replace_all(name_str, "$edge");
      let edge = edge.into_owned();
      let edge = Ident::new(&*edge, Span::call_site());

      quote! {
        if let Some(#value_block) = &self.#name {
          border_radius.#edge = #value_block.clone();
        }
      }
    }

    "Filters" => {
      quote! {
        if let Some(#value_block) = &self.#name {
          apperance.filter = Some(#value_block.clone());
        }
      }
    }

    "Transforms" => {
      quote!{
        if let Some(#value_block) = &self.#name {
          transforms = #value_block.clone();
        }
      }
    }

    _ => {
      quote! {
        if let Some(#value_block) = &self.#name {
          layout.push(FlexStyle::#enum_type(#value_block.clone()));
        }
      }
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
    impl PrepareStyleExt for #struct_id {
      fn get_prepared_styles(&self) -> (Apperance, Vec<FlexStyle>) {
        let mut apperance = Apperance::default();
        let mut layout: Vec<FlexStyle> = vec![];
        let mut transforms = vec![];

        let mut border_styles = BorderStyles::default();
        let mut border_radius = BorderRadius::default();

        #(#expressions)*

        transforms = transforms.iter().cloned()
          .filter(|field| !field.is_none())
          .collect();

        apperance.border_styles = Some(border_styles);
        apperance.border_radius = Some(border_radius);
        apperance.transform = Some(transforms);

        (apperance, layout)
      }
    }
  }
}
