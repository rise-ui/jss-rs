use std::collections::HashMap;

/// The value of the variable for calculations in runtime
#[derive(Clone, Debug, PartialEq)]
pub enum Variable {
    Map(HashMap<String, f32>),
    Number(f32),
}
