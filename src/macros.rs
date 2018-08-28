macro_rules! impl_union_property_conversion {
  ($src:ident) => {
    impl From<$src> for PropertyValue {
      fn from(value: $src) -> PropertyValue {
        PropertyValue::$src(value)
      }
    }

    impl From<$src> for Option<PropertyValue> {
      fn from(value: $src) -> Option<PropertyValue> {
        Some(PropertyValue::$src(value))
      } 
    }
  };
}

// @TODO: fix convertions for usage with types
// unwraped of Apperance or Layout enum
// Like: fn(Align::Center) instead of PropertyValue::Layout(Layout::Align::Center)
macro_rules! impl_partial_union_property_conversion {
  ($child:ident, $parent:ident) => {
    impl From<$child> for $parent {
      fn from(value: $child) -> $parent {
        $parent::$child(value)
      }
    }
    
    impl From<$child> for Option<$parent> {
      fn from(value: $child) -> Option<$parent> {
        Some($parent::$child(value))
      }
    }
  }
}