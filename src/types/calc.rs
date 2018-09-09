use serde::de::{self, Deserialize};
use serde::ser::Serialize;

use std::fmt::Debug;
use eval::Expr;
use serde::Serializer;
use serde::Deserializer;

pub struct CalcExpr {
    raw: String,
    expr: Expr,
}

impl Serialize for CalcExpr {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
    {
        serializer.serialize_str(self.raw.as_str())
    }
}

impl<'de> Deserialize<'de> for CalcExpr {
    fn deserialize<D>(deserializer: D) -> Result<CalcExpr, D::Error>
        where
            D: Deserializer<'de>,
    {
        extract!(Value::String(_), Value::deserialize(deserializer)?)
            .and_then(|expr| Expr::new(expr))
            .map_err(de::Error::custom)
    }
}