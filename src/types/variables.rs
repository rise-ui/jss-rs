use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
    Map(HashMap<String, f32>),
    Number(f32),
}
