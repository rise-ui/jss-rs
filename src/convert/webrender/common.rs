use yoga::FlexStyle;
use webrender::api;

use utils::{
    properties_extract_radius,
    properties_extract_borders,
};

use types::{
    PropertiesAppearance,
    PropertiesLayout,
    Appearance,
    Context,
};

use convert::{
    BorderRadiusWrapper,
    BorderStylesWrapper,
    WebrenderBackground,
    BackgroundWrapper,
    TransformsWrapper,
    WebrenderBorders,
};

pub struct WebrenderStyles {
    pub background: WebrenderBackground,
    pub transforms: TransformsWrapper,
    pub borders: WebrenderBorders,
}

pub struct AppearanceWrapper<'a, 'b> {
    pub builder: &'a mut api::DisplayListBuilder,
    pub layout: &'b PropertiesLayout,

    pub properties: PropertiesAppearance,
    pub context: Context,
}

impl<'a, 'b> From<AppearanceWrapper<'a, 'b>> for WebrenderStyles {
    fn from(wrapper: AppearanceWrapper<'a, 'b>) -> WebrenderStyles {
        // BACKGROUND STYLE
        let background_raw =
            wrapper.properties.0.get("background").and_then(|value| extract!(Appearance::Background(_), value.clone()));

        let background: WebrenderBackground = if let Some(value) = background_raw {
            BackgroundWrapper {
                context: wrapper.context.clone(),
                builder: wrapper.builder,
                background: value,
            }
            .into()
        } else {
            WebrenderBackground::Color(api::ColorF::TRANSPARENT)
        };

        // BORDER RADIUS
        let border_radius = properties_extract_radius(&wrapper.properties);
        let border_radius = BorderRadiusWrapper {
            context: wrapper.context.clone(),
            border_radius,
        };

        // BORDERS STYLE
        let borders = properties_extract_borders(&wrapper.properties, &wrapper.layout);
        let borders: WebrenderBorders = BorderStylesWrapper {
            context: wrapper.context.clone(),
            border_radius,
            borders,
        }
        .into();

        // TRANSFORMS
        let transforms = wrapper
            .properties
            .0
            .get("transforms")
            .and_then(|value| extract!(Appearance::Transforms(_), value.clone()))
            .unwrap_or_default();

        let transforms = TransformsWrapper {
            context: wrapper.context.clone(),
            transforms,
        };

        // finalize
        WebrenderStyles {
            transforms,
            background,
            borders,
        }
    }
}
