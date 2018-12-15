use yoga::{StyleUnit, FlexStyle};
use std::f32;

use properties::{
    BorderRadius,
    BorderStyles,
    BorderStyle,
    Border,
    Color,
};

use types::{
    PropertiesAppearance,
    PropertiesLayout,
    Appearance,
    SharedUnit,
};

pub fn extract_unit_value(unit: &StyleUnit) -> f32 {
    use self::StyleUnit::*;

    match unit {
        Percent(value) => value.into_inner(),
        Point(value) => value.into_inner(),
        _ => 0.0,
    }
}

pub fn properties_extract_radius(appearance: &PropertiesAppearance) -> BorderRadius {
    let corners: Vec<SharedUnit> = [
        "border_top_left_radius",
        "border_top_right_radius",
        "border_bottom_right_radius",
        "border_bottom_left_radius",
    ]
    .into_iter()
    .map(|key: &&str| -> SharedUnit {
        appearance
            .0
            .get(&key.to_string())
            .and_then(|value| extract!(Appearance::BorderRadius(_), value.clone()))
            .unwrap_or(SharedUnit::StyleUnit(StyleUnit::Point(0.0.into())))
    })
    .collect();

    BorderRadius {
        bottom_right: corners[2].clone(),
        bottom_left: corners[3].clone(),
        top_right: corners[1].clone(),
        top_left: corners[0].clone(),
    }
}

pub fn properties_extract_borders(appearance: &PropertiesAppearance, layout: &PropertiesLayout) -> BorderStyles {
    let border_styles: Vec<BorderStyle> =
        ["border_top_style", "border_right_style", "border_bottom_style", "border_left_style"]
            .into_iter()
            .map(|key: &&str| -> BorderStyle {
                appearance
                    .0
                    .get(&key.to_string())
                    .and_then(|value| extract!(Appearance::BorderStyle(_), value.clone()))
                    .unwrap_or(BorderStyle::None)
            })
            .collect();

    let border_widths: Vec<f32> =
        ["border_top_width", "border_right_width", "border_bottom_width", "border_left_width"]
            .into_iter()
            .map(|key: &&str| -> f32 {
                layout
                    .0
                    .get(&key.to_string())
                    .and_then(|value| {
                        match key {
                            &"border_top_width" => extract!(FlexStyle::BorderTop(_), value),
                            &"border_right_width" => extract!(FlexStyle::BorderRight(_), value),
                            &"border_bottom_width" => extract!(FlexStyle::BorderBottom(_), value),
                            &"border_left_width" => extract!(FlexStyle::BorderLeft(_), value),
                            _ => None,
                        }
                        .and_then(|r| Some(r.into_inner()))
                    })
                    .unwrap_or_default()
            })
            .collect();

    let border_colors: Vec<Color> =
        ["border_top_color", "border_right_color", "border_bottom_color", "border_left_color"]
            .into_iter()
            .map(|key: &&str| -> Color {
                appearance
                    .0
                    .get(&key.to_string())
                    .and_then(|value| extract!(Appearance::BorderColor(_), value.clone()))
                    .unwrap_or(Color::transparent())
            })
            .collect();

    let borders: Vec<Border> = (0..4)
        .into_iter()
        .map(|index| Border {
            color: border_colors[index],
            width: border_widths[index],
            style: border_styles[index],
        })
        .collect();

    BorderStyles {
        bottom: borders[2].clone(),
        right: borders[1].clone(),
        left: borders[3].clone(),
        top: borders[0].clone(),
    }
}
