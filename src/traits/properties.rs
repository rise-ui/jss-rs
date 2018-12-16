use yoga::{FlexStyle, Layout as Dimensions};
use types::PropertiesAppearance;
use std::fmt::Debug;

use types::{
    ProcessingError,
    PropertyError,
    PropertyValue,
    DimensionType,
    Appearance,
    Variable,
    Layout,
};

/// Operation over component - set and remove style properties
pub trait TStyle: Debug + PartialEq + Clone {
    /// Set appearance styles such like `Background`, `BorderStyle` etc..
    fn set_appearance_style<T: Into<Appearance>>(&mut self, &str, T) -> Result<(), PropertyError>;
    /// Set layout styles such like Flex properties, `Margin`, `Padding` etc..
    fn set_layout_style<T: Into<Layout>>(&mut self, &str, T) -> Result<(), PropertyError>;
    /// Unified setter for any property
    fn set_style<T: Into<PropertyValue>>(&mut self, &str, T) -> Result<(), PropertyError>;
    /// Remove property from style
    fn remove_style(&mut self, &str);
}

/// Manipulate runtime context of current style - work with active element dimensions,
/// set variables for runtime calculation and other data needed in runtime
pub trait TStyleContext: Debug + PartialEq + Clone {
    /// Set dimensions with size and position node on layout
    fn set_dimension(&mut self, DimensionType, Option<Dimensions>);
    /// Set variables for runtime calculator
    /// after apply you can use that var inside calc expression
    fn set_variable(&mut self, String, Variable);
    /// The same thing as `set_variable` but for enumeration items
    fn set_variables<T>(&mut self, T)
    where
        T: IntoIterator<Item = (String, Variable)>;
}

/// Collecting/Save/Get finalized properties after calculation runtime
/// middlewares, expressions, custom functions and other things.
/// That return processed properties and warning errors
pub trait TStyleCollect: Debug + PartialEq + Clone {
    /// Collect finalize layout styles with calculate expressions
    fn calculate_appearance(&mut self) -> Vec<ProcessingError>;
    fn calculate_layout(&mut self) -> Vec<ProcessingError>;
}

/// Enabled states of current style, for create valid style by enabled states
/// Example: "default"+"hover", "default"+"active", "default"+"focus", etc..
/// states ordering by prority - each subsequent state overlaps the properties
/// of the previous state by key/value properties
pub trait TStyleStates: Debug + PartialEq + Clone {
    fn enable_states(&mut self, Vec<String>);
}
