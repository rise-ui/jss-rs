use properties::{Border, BorderRadius, BorderStyle, BorderStyles};
use types::{Context, SharedUnit};
use yoga::StyleUnit;
use webrender::api;

#[derive(Debug, Clone, PartialEq)]
pub struct WebrenderBorders {
    pub border_radius: api::BorderRadius,
    pub widths: api::LayoutSideOffsets,
    pub details: api::BorderDetails,
}

pub struct BorderStylesWrapper {
    pub border_radius: BorderRadiusWrapper,
    pub borders: BorderStyles,
    pub context: Context,
}

pub struct BorderRadiusWrapper {
    pub border_radius: BorderRadius,
    pub context: Context,
}

fn get_layout_size_corners(unit: SharedUnit, dimensions: (f32, f32)) -> api::LayoutSize {
    let corner = extract!(SharedUnit::StyleUnit(_), unit).and_then(|unit| {
        let radius = match unit {
            // @TODO: adding support percent radius by dimensions element
            StyleUnit::Percent(value) => api::LayoutSize::new(value.into_inner(), value.into_inner()),
            StyleUnit::Point(value) => api::LayoutSize::new(value.into_inner(), value.into_inner()),
            _ => api::LayoutSize::new(0.0, 0.0),
        };

        Some(radius)
    });

    corner.unwrap_or(api::LayoutSize::new(0.0, 0.0))
}

impl Into<api::BorderSide> for Border {
    fn into(self) -> api::BorderSide {
        api::BorderSide {
            color: self.color.into(),
            style: self.style.into(),
        }
    }
}

impl Into<api::BorderRadius> for BorderRadiusWrapper {
    fn into(self) -> api::BorderRadius {
        let dimensions: (f32, f32) = if let Some(layout) = self.context.dimensions.current {
            (layout.width(), layout.height())
        } else {
            (0.0, 0.0)
        };

        api::BorderRadius {
            bottom_right: get_layout_size_corners(self.border_radius.bottom_right, dimensions.clone()),
            bottom_left: get_layout_size_corners(self.border_radius.bottom_left, dimensions.clone()),
            top_right: get_layout_size_corners(self.border_radius.top_right, dimensions.clone()),
            top_left: get_layout_size_corners(self.border_radius.top_left, dimensions.clone()),
        }
    }
}

impl Into<api::BorderStyle> for BorderStyle {
    fn into(self) -> api::BorderStyle {
        use self::BorderStyle::*;

        match self {
            Outset => api::BorderStyle::Outset,
            Double => api::BorderStyle::Double,
            Dotted => api::BorderStyle::Dotted,
            Dashed => api::BorderStyle::Dashed,
            Hidden => api::BorderStyle::Hidden,
            Groove => api::BorderStyle::Groove,
            Solid => api::BorderStyle::Solid,
            Ridge => api::BorderStyle::Ridge,
            Inset => api::BorderStyle::Inset,
            None => api::BorderStyle::None,
        }
    }
}

impl From<BorderStylesWrapper> for WebrenderBorders {
    fn from(value: BorderStylesWrapper) -> WebrenderBorders {
        let border_radius: api::BorderRadius = value.border_radius.into();

        let widths = api::LayoutSideOffsets::new(
            value.borders.top.width,
            value.borders.right.width,
            value.borders.bottom.width,
            value.borders.left.width,
        );

        let details = api::BorderDetails::Normal(api::NormalBorder {
            bottom: value.borders.bottom.into(),
            right: value.borders.right.into(),
            left: value.borders.left.into(),
            top: value.borders.top.into(),

            radius: border_radius.clone(),
            do_aa: true,
        });

        WebrenderBorders {
            border_radius,
            details,
            widths,
        }
    }
}
