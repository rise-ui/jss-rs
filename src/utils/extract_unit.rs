use std::mem::discriminant as enum_eq;
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
    AppearanceKey,
    PropertyKey,
    Appearance,
    SharedUnit,
    LayoutKey,
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
        AppearanceKey::BorderTopLeftRadius,
        AppearanceKey::BorderTopRightRadius,
        AppearanceKey::BorderBottomRightRadius,
        AppearanceKey::BorderBottomLeftRadius,
    ]
    .into_iter()
    .map(|key: &AppearanceKey| -> SharedUnit {
        appearance
            .0
            .get(key)
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

pub fn properties_extract_borders(appearance: &PropertiesAppearance, layout: &Vec<FlexStyle>) -> BorderStyles {
    let border_styles: Vec<BorderStyle> = [
        AppearanceKey::BorderTopStyle,
        AppearanceKey::BorderRightStyle,
        AppearanceKey::BorderBottomStyle,
        AppearanceKey::BorderLeftStyle
    ]
    .into_iter()
    .map(|key: &AppearanceKey| -> BorderStyle {
        appearance
            .0
            .get(key)
            .and_then(|value| extract!(Appearance::BorderStyle(_), value.clone()))
            .unwrap_or(BorderStyle::None)
    })
    .collect();

    let mut border_widths: Vec<f32> = vec![0.0, 0.0, 0.0, 0.0];
    for prop in layout {
        let disc = enum_eq(prop);

        if disc == enum_eq(&FlexStyle::BorderTop(0.0.into())) {
            border_widths[0] = extract!(FlexStyle::BorderTop(_), prop).and_then(|f| Some(f.into_inner())).unwrap();
        } else if disc == enum_eq(&FlexStyle::BorderRight(0.0.into())) {
            border_widths[1] = extract!(FlexStyle::BorderRight(_), prop).and_then(|f| Some(f.into_inner())).unwrap();
        } else if disc == enum_eq(&FlexStyle::BorderBottom(0.0.into())) {
            border_widths[2] = extract!(FlexStyle::BorderBottom(_), prop).and_then(|f| Some(f.into_inner())).unwrap();
        } else if disc == enum_eq(&FlexStyle::BorderLeft(0.0.into())) {
            border_widths[3] = extract!(FlexStyle::BorderLeft(_), prop).and_then(|f| Some(f.into_inner())).unwrap();
        }
    }

    let border_colors: Vec<Color> = [
        AppearanceKey::BorderTopColor,
        AppearanceKey::BorderRightColor,
        AppearanceKey::BorderBottomColor,
        AppearanceKey::BorderLeftColor
    ]
    .into_iter()
    .map(|key: &AppearanceKey| -> Color {
        appearance
            .0
            .get(key)
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
