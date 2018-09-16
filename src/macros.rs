macro_rules! impl_union_property_conversion {
    ($src:ident) => {
        impl From<$src> for PropertyValue {
            fn from(value: $src) -> PropertyValue {
                PropertyValue::$src(value)
            }
        }
    };
}

macro_rules! impl_union_into_appearance {
    ($src:ident) => {
        impl From<$src> for Appearance {
            fn from(value: $src) -> Appearance {
                Appearance::$src(value)
            }
        }

        impl From<$src> for PropertyValue {
            fn from(value: $src) -> PropertyValue {
                let appearance: Appearance = value.into();
                PropertyValue::Appearance(appearance)
            }
        }
    };
}

macro_rules! impl_union_into_layout {
    ($src:ident) => {
        impl From<$src> for Layout {
            fn from(value: $src) -> Layout {
                Layout::$src(value)
            }
        }

        impl From<$src> for PropertyValue {
            fn from(value: $src) -> PropertyValue {
                let layout: Layout = value.into();
                PropertyValue::Layout(layout)
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
