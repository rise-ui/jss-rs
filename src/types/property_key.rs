use utils::{apperance_keys_contains, layout_keys_contains};
use types::{Case, PropertyError};
use inflector::Inflector;

/// Middleware enum with property type of current style field
#[derive(Clone, Debug, PartialEq)]
pub enum PropertyParseType {
    /// Calculator expression parser. For properties that use calc in runtime. Key started from "~"
    Expression,
    /// Default parser middleware. Key without prefix for standart properties
    Default,
    /// Custom parser middleware. Key started from "@"
    Custom,
}

/// Middleware enum with property type of current stylesheet field
#[derive(Clone, Debug, PartialEq)]
pub enum FieldParseType {
    /// Stylesheet global variable started from "$"
    Variable,
    /// Default parser middleware. Key without prefix for standart style name
    Default,
    /// Custom parser middleware. Key started from "@"
    Custom,
}

/// Expanded info about property key - needed middleware, case, etc..
#[derive(Clone, Debug, PartialEq)]
pub struct PropertyKeyInfo {
    // @todo: next adding args type for property
    pub key_type: PropertyParseType,
    pub source: String,
    pub name: String,
    pub case: Case,
}

impl PropertyKeyInfo {
    /// Create a new info instanse, this method return error if key isnt valid
    pub fn new(key: &str) -> Result<PropertyKeyInfo, PropertyError> {
        let prefix = match &key[..1] {
            "~" => Ok(PropertyParseType::Expression),
            "@" => Ok(PropertyParseType::Custom),
            _ => {
                let snake_key = key.to_snake_case();

                if apperance_keys_contains(snake_key.as_str()) || layout_keys_contains(snake_key.as_str()) {
                    Ok(PropertyParseType::Default)
                } else {
                    Err(PropertyError::InvalidKey {
                        key: key.to_string(),
                    })
                }
            }
        }?;

        let name = if prefix != PropertyParseType::Default {
            key[1..].to_string()
        } else {
            key.to_string()
        };

        Ok(PropertyKeyInfo {
            case: Case::new(name.as_str()),
            source: key.to_string(),
            key_type: prefix,
            name,
        })
    }
}

pub struct StylesheetFieldInfo {
    pub key_type: FieldParseType,
    pub source: String,
    pub name: String,
    pub case: Case,
}

impl StylesheetFieldInfo {
    pub fn new(key: &str) -> StylesheetFieldInfo {
        let prefix = match &key[..1] {
            "$" => FieldParseType::Variable,
            "@" => FieldParseType::Custom,
            _ => FieldParseType::Default,
        };

        let name = if prefix != FieldParseType::Default {
            key[1..].to_string()
        } else {
            key.to_string()
        };

        StylesheetFieldInfo {
            case: Case::new(name.as_str()),
            source: key.to_string(),
            key_type: prefix,
            name,
        }
    } 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_key_info() {
        let default = extract!(Ok(_), PropertyKeyInfo::new("background"));
        let custom = extract!(Ok(_), PropertyKeyInfo::new("@custom"));
        let expr = extract!(Ok(_), PropertyKeyInfo::new("~expr_value"));

        let exp_default = Some(PropertyKeyInfo {
            key_type: PropertyParseType::Default,
            source: "background".to_string(),
            name: "background".to_string(),
            case: Case::Snake,
        });
        let exp_custom = Some(PropertyKeyInfo {
            key_type: PropertyParseType::Custom,
            source: "@custom".to_string(),
            name: "custom".to_string(),
            case: Case::Snake,
        });
        let exp_expr = Some(PropertyKeyInfo {
            key_type: PropertyParseType::Expression,
            source: "~expr_value".to_string(),
            name: "expr_value".to_string(),
            case: Case::Snake,
        });

        assert_eq!(default, exp_default);
        assert_eq!(custom, exp_custom);
        assert_eq!(expr, exp_expr);
    }
}
