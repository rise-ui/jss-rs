//! Property setters without runtime type-checking
//! *Warning*: Be careful when using: functions allow you to change properties
//! without checking - which can lead to untracked errors during execution.
use utils::get_reflect_property_type;
use yoga::FlexStyle;

use types::{
    AppearanceKey,
    PropertyError,
    pair_to_flex,
    PropertyKey,
    SharedUnit,
    Appearance,
    Properties,
    LayoutKey,
};

macro_rules! default_setter {
    ($properties:ident, $field:ident, $key:ident, $value:ident) => {
        $properties.$field.0.insert($key, $value);
    };
}

/// Set basic appearance property without check
pub fn set_appearance_without_check(properties: &mut Properties, key: AppearanceKey, value: Appearance) {
    default_setter!(properties, appearance, key, value);
}

/// Set basic layout property without check
pub fn set_layout_without_check(properties: &mut Properties, key: LayoutKey, value: FlexStyle) {
    default_setter!(properties, layout, key, value);
}

/// Set shared layout property without check (expression or exact unit)
pub fn set_layout_unit_without_check(
    properties: &mut Properties,
    key: LayoutKey,
    value: SharedUnit,
) -> Result<(), PropertyError> {
    match value {
        SharedUnit::StyleUnit(unit) => {
            properties.expressions.0.remove(&key).is_some();
            let unit = pair_to_flex(key.clone(), unit);
            properties.layout.0.insert(key, unit).is_some();

            Ok(())
        }

        SharedUnit::CalcExpr(expression) => {
            properties.layout.0.remove(&key).is_some();

            let expression = if !expression.get_compiled().is_some() {
                expression.compile().map_err(|error| PropertyError::InvalidExpression {
                    key: format!("{:?}", key),
                    error,
                })?
            } else {
                expression
            };

            properties.expressions.0.insert(key, expression).is_some();
            Ok(())
        }
    }
}

/// Create expected type error by property key
pub fn expected_type_error(property: PropertyKey) -> PropertyError {
    PropertyError::InvalidType {
        expected: get_reflect_property_type(&property),
        property: format!("{:?}", property),
    }
}
