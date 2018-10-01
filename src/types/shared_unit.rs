use types::{PropertyError, ParseError, Layout, PropertyValue};
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
pub fn pair_to_flex(key: String, value: StyleUnit) -> Result<FlexStyle, PropertyError> {
    match key.as_str() {
        "bottom" => Ok(FlexStyle::Bottom(value)),
        "end" => Ok(FlexStyle::End(value)),
        "flex_basis" => Ok(FlexStyle::FlexBasis(value)),
        "height" => Ok(FlexStyle::Height(value)),
        "left" => Ok(FlexStyle::Left(value)),
        "margin" => Ok(FlexStyle::Margin(value)),
        "margin_bottom" => Ok(FlexStyle::MarginBottom(value)),
        "margin_end" => Ok(FlexStyle::MarginEnd(value)),
        "margin_horizontal" => Ok(FlexStyle::MarginHorizontal(value)),
        "margin_left" => Ok(FlexStyle::MarginLeft(value)),
        "margin_right" => Ok(FlexStyle::MarginRight(value)),
        "margin_start" => Ok(FlexStyle::MarginStart(value)),
        "margin_top" => Ok(FlexStyle::MarginTop(value)),
        "margin_vertical" => Ok(FlexStyle::MarginVertical(value)),
        "max_height" => Ok(FlexStyle::MaxHeight(value)),
        "max_width" => Ok(FlexStyle::MaxWidth(value)),
        "min_height" => Ok(FlexStyle::MinHeight(value)),
        "min_width" => Ok(FlexStyle::MinWidth(value)),
        "padding" => Ok(FlexStyle::Padding(value)),
        "padding_bottom" => Ok(FlexStyle::PaddingBottom(value)),
        "padding_end" => Ok(FlexStyle::PaddingEnd(value)),
        "padding_horizontal" => Ok(FlexStyle::PaddingHorizontal(value)),
        "padding_left" => Ok(FlexStyle::PaddingLeft(value)),
        "padding_right" => Ok(FlexStyle::PaddingRight(value)),
        "padding_start" => Ok(FlexStyle::PaddingStart(value)),
        "padding_top" => Ok(FlexStyle::PaddingTop(value)),
        "padding_vertical" => Ok(FlexStyle::PaddingVertical(value)),
        "right" => Ok(FlexStyle::Right(value)),
        "start" => Ok(FlexStyle::Start(value)),
        "top" => Ok(FlexStyle::Top(value)),
        "width" => Ok(FlexStyle::Width(value)),
        _ => Err(PropertyError::SharedUnitConvert {
            key,
        }),
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
