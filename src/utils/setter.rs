use yoga::FlexStyle;

use types::{
    get_reflect_property_type,
    pair_to_flex,
    PropertyError,
    SharedUnit,
    Appearance,
    Properties,
};

macro_rules! default_setter {
    ($properties:ident, $field:ident, $key:ident, $value:ident) => {
        if $properties.$field.0.contains_key(&$key) {
            let item = $properties.$field.0.get_mut(&$key).unwrap();
            *item = $value;
        } else {
            $properties.$field.0.insert($key, $value);
        }
    };
}

pub fn set_appearance_without_check(properties: &mut Properties, key: String, value: Appearance) {
    default_setter!(properties, appearance, key, value);
}

pub fn set_layout_without_check(properties: &mut Properties, key: String, value: FlexStyle) {
    default_setter!(properties, layout, key, value);
}

pub fn set_layout_unit_without_check(
    properties: &mut Properties,
    key: String,
    value: SharedUnit,
) -> Result<(), PropertyError> {
    match value {
        SharedUnit::StyleUnit(unit) => {
            properties.expressions.0.remove(&key).is_some();

            pair_to_flex(key.clone(), unit).and_then(|unit| {
                if let Some(item) = properties.layout.0.get_mut(&key) {
                    *item = unit;
                    return Ok(());
                }

                properties.layout.0.insert(key, unit).is_some();
                Ok(())
            })
        }

        SharedUnit::CalcExpr(expr) => {
            properties.layout.0.remove(&key).is_some();

            expr.compile()
                .map_err(|error| PropertyError::InvalidExpression {
                    key: key.clone(),
                    error,
                }).and_then(|expr| {
                    if let Some(item) = properties.expressions.0.get_mut(&key) {
                        *item = expr;
                        return Ok(());
                    }

                    properties.expressions.0.insert(key, expr).is_some();
                    Ok(())
                })
        }
    }
}

// Create expected type error by property key
pub fn expected_type_error(property: String) -> PropertyError {
    PropertyError::InvalidType {
        expected: get_reflect_property_type(property.as_str()).to_string(),
        property: property,
    }
}
