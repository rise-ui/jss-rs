use yoga::{FlexStyle, StyleUnit};
use std::convert::TryInto;
use types::PropertyError;
use eval::Expr;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SharedUnit {
    /// Default Yoga Unit
    StyleUnit(StyleUnit),
    /// Calculator expression (for runtime value)
    CalcExpr(Expr),
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
