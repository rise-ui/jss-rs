#[macro_export]
macro_rules! style_setters {
  (($self:ident, $name:ident, $property:ident, $enum:ident, $struct_field:ident) { $($field:ident: $value:ident) , * }) => {
    match $name {
      $(
        stringify!($field) => {
          let hash_key = String::from(stringify!($name).to_snake_case());

          if let Some(expected) = extract!($enum::$value(_), $property) {
            let container_value = $enum::$value(expected);

            if $self.$struct_field.0.contains_key(&hash_key) {
              let item = $self.$struct_field.0.get_mut(&hash_key).unwrap();
              *item = container_value;
            } else {
              $self.$struct_field.0.insert(hash_key, container_value);
            }

            Ok(())
          } else {
            Err(PropertyError::InvalidType {
              property: stringify!($field).to_string(),
              expected: stringify!($value).to_string(),
            })
          }
        },
      )*
      _ => unreachable!(),
    }
  };
}
