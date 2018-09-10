use inflector::Inflector;
use types::Case;

lazy_static! {
    static ref APPERANCE_KEYS: Vec<&'static str> = vec![
        "background",
        "transform",
        "filter",
        "border_top_color",
        "border_right_color",
        "border_left_color",
        "border_bottom_color",
        "border_top_style",
        "border_right_style",
        "border_left_style",
        "border_bottom_style",
        "border_top_right_radius",
        "border_top_left_radius",
        "border_bottom_right_radius",
        "border_bottom_left_radius",
    ];
    static ref LAYOUT_KEYS: Vec<&'static str> = vec![
        "flex_direction",
        "justify_content",
        "position",
        "align_content",
        "align_items",
        "align_self",
        "flex_wrap",
        "display",
        "overflow",
        "aspect_ratio",
        "flex_shrink",
        "flex_grow",
        "flex",
        "bottom",
        "end",
        "flex_basis",
        "height",
        "left",
        "margin",
        "margin_bottom",
        "margin_end",
        "margin_horizontal",
        "margin_left",
        "margin_right",
        "margin_start",
        "margin_top",
        "margin_vertical",
        "max_height",
        "max_width",
        "min_height",
        "min_width",
        "padding",
        "padding_bottom",
        "padding_end",
        "padding_horizontal",
        "padding_left",
        "padding_right",
        "padding_start",
        "padding_top",
        "padding_vertical",
        "right",
        "start",
        "top",
        "width",
        "border_bottom_width",
        "border_right_width",
        "border_left_width",
        "border_top_width",
    ];
}

pub fn apperance_keys_contains(name: &str) -> bool {
    APPERANCE_KEYS.contains(&name)
}

pub fn layout_keys_contains(name: &str) -> bool {
    LAYOUT_KEYS.contains(&name)
}

pub fn is_valid_case(key: &String, case: Case) -> bool {
    match case {
        Case::Snake => key.is_snake_case(),
        Case::Kebab => key.is_kebab_case(),
        Case::Camel => key.is_camel_case(),
        Case::Ignore => true,
    }
}
