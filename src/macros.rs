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

macro_rules! impl_into_for_yoga_property {
    ($src:ident, $target:ident) => {
        impl From<$src> for $target {
            fn from(source: $src) -> $target {
                $target::$src(source)
            }
        }
    };
}

macro_rules! make_initial_style_states {
    ($style:ident, [$($state:ident),*]) => {
        use types::Properties;
        $(
            $style.states.insert(stringify!($state).to_string(), Properties::default());
        )*
    };
}
