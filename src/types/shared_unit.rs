use types::{PropertyError, ParseError, Layout, PropertyValue, LayoutKey};
use serde::de::{self, Deserialize, Deserializer};
use serde::ser::{Serialize, Serializer};
use yoga::{FlexStyle, StyleUnit};
use ordered_float::OrderedFloat;
use std::str::FromStr;
use serde_json::Value;
use eval::Expr;
use std::f32;

impl_union_into_layout!(SharedUnit);

/// Basic SharedUnit for unit based fields
/// contains variant for exact value or runtime expression
#[derive(Debug, Clone, PartialEq)]
pub enum SharedUnit {
    /// Default Yoga Unit
    StyleUnit(StyleUnit),
    /// Calculator expression (for runtime value)
    CalcExpr(Expr),
}

impl Serialize for SharedUnit {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            SharedUnit::StyleUnit(unit) => match unit {
                StyleUnit::Point(number) => {
                    let result = format!("{}px", number);
                    serializer.serialize_str(result.as_str())
                }

                StyleUnit::Percent(percent) => {
                    let result = format!("{}%", percent);
                    serializer.serialize_str(result.as_str())
                }

                StyleUnit::Auto => serializer.serialize_str("auto"),

                StyleUnit::UndefinedValue => serializer.serialize_str("undefined"),
            },

            SharedUnit::CalcExpr(expression) => expression.serialize(serializer),
        }
    }
}

impl<'de> Deserialize<'de> for SharedUnit {
    fn deserialize<D>(deserializer: D) -> Result<SharedUnit, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Value::deserialize(deserializer)? {
            Value::String(value) => {
                if value.len() > 0 {
                    if let Some(unit) = parse_length(value.as_str()) {
                        let shared_unit = SharedUnit::StyleUnit(unit);
                        Ok(shared_unit)
                    } else {
                        let expression = Expr::new(value).compile().map_err(|error| {
                            de::Error::custom(ParseError::PropertyError {
                                error: PropertyError::InvalidExpression {
                                    key: "SharedUnit".to_string(),
                                    error,
                                },
                            })
                        })?;

                        Ok(SharedUnit::CalcExpr(expression))
                    }
                } else {
                    Err(de::Error::custom(ParseError::PropertyError {
                        error: PropertyError::InvalidType {
                            expected: "non empty value".to_string(),
                            property: "SharedUnit".to_string(),
                        },
                    }))
                }
            }

            Value::Number(value) => {
                let point = value.as_f64().ok_or_else(|| {
                    de::Error::custom(ParseError::PropertyError {
                        error: PropertyError::InvalidType {
                            expected: "float or integer".to_string(),
                            property: "SharedUnit".to_string(),
                        },
                    })
                })?;

                let point: OrderedFloat<f32> = (point as f32).into();
                Ok(SharedUnit::StyleUnit(StyleUnit::Point(point)))
            }

            _ => Err(de::Error::custom(ParseError::PropertyError {
                error: PropertyError::InvalidType {
                    expected: "number or expression string".to_string(),
                    property: "SharedUnit".to_string(),
                },
            })),
        }
    }
}

/// Convertor: create FlexStyle by property key and unit value
pub fn pair_to_flex(key: LayoutKey, value: StyleUnit) -> FlexStyle {
    match key {
        LayoutKey::Bottom => FlexStyle::Bottom(value),
        LayoutKey::End => FlexStyle::End(value),
        LayoutKey::FlexBasis => FlexStyle::FlexBasis(value),
        LayoutKey::Height => FlexStyle::Height(value),
        LayoutKey::Left => FlexStyle::Left(value),
        LayoutKey::Margin => FlexStyle::Margin(value),
        LayoutKey::MarginBottom => FlexStyle::MarginBottom(value),
        LayoutKey::MarginEnd => FlexStyle::MarginEnd(value),
        LayoutKey::MarginHorizontal => FlexStyle::MarginHorizontal(value),
        LayoutKey::MarginLeft => FlexStyle::MarginLeft(value),
        LayoutKey::MarginRight => FlexStyle::MarginRight(value),
        LayoutKey::MarginStart => FlexStyle::MarginStart(value),
        LayoutKey::MarginTop => FlexStyle::MarginTop(value),
        LayoutKey::MarginVertical => FlexStyle::MarginVertical(value),
        LayoutKey::MaxHeight => FlexStyle::MaxHeight(value),
        LayoutKey::MaxWidth => FlexStyle::MaxWidth(value),
        LayoutKey::MinHeight => FlexStyle::MinHeight(value),
        LayoutKey::MinWidth => FlexStyle::MinWidth(value),
        LayoutKey::Padding => FlexStyle::Padding(value),
        LayoutKey::PaddingBottom => FlexStyle::PaddingBottom(value),
        LayoutKey::PaddingEnd => FlexStyle::PaddingEnd(value),
        LayoutKey::PaddingHorizontal => FlexStyle::PaddingHorizontal(value),
        LayoutKey::PaddingLeft => FlexStyle::PaddingLeft(value),
        LayoutKey::PaddingRight => FlexStyle::PaddingRight(value),
        LayoutKey::PaddingStart => FlexStyle::PaddingStart(value),
        LayoutKey::PaddingTop => FlexStyle::PaddingTop(value),
        LayoutKey::PaddingVertical => FlexStyle::PaddingVertical(value),
        LayoutKey::Right => FlexStyle::Right(value),
        LayoutKey::Start => FlexStyle::Start(value),
        LayoutKey::Top => FlexStyle::Top(value),
        LayoutKey::Width => FlexStyle::Width(value),
        _ => unreachable!(),
    }
}

// @note: Adapted clone of this https://doc.servo.org/style/attr/fn.parse_length.html
pub fn parse_length(mut value: &str) -> Option<StyleUnit> {
    let space_matches: &[_] = &[' ', '\t', '\n', '\u{c}', '\r'];
    value = value.trim_left_matches(space_matches);

    if value.is_empty() {
        return None;
    }

    if value == "auto" {
        return Some(StyleUnit::Auto);
    }

    if value.starts_with('+') {
        value = &value[1..]
    }

    match value.chars().nth(0) {
        Some('0'...'9') => {}
        _ => return None,
    }

    let mut end_index = value.len();
    let (mut found_full_stop, mut found_percent) = (false, false);
    for (i, ch) in value.chars().enumerate() {
        match ch {
            '0'...'9' => continue,
            '%' => {
                found_percent = true;
                end_index = i;
                break;
            }
            '.' if !found_full_stop => {
                found_full_stop = true;
                continue;
            }
            _ => {
                end_index = i;
                break;
            }
        }
    }
    value = &value[..end_index];

    let result: Result<f32, _> = FromStr::from_str(value);

    if found_percent {
        result.ok().and_then(|number| {
            let unit = StyleUnit::Percent((number / 100.0).into());
            Some(unit)
        })
    } else {
        result.ok().and_then(|number| {
            let unit = StyleUnit::Point(number.into());
            Some(unit)
        })
    }
}
