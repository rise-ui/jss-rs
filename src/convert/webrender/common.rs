use yoga::FlexStyle;
use webrender::api;

use utils::{
    properties_extract_radius,
    properties_extract_borders,
};

use types::{
    PropertiesAppearance,
    PropertiesLayout,
    AppearanceKey,
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

#[derive(Debug, Clone, PartialEq)]
pub struct WebrenderStyles {
    pub background: WebrenderBackground,
    pub transforms: TransformsWrapper,
    pub borders: WebrenderBorders,
}

pub struct AppearanceWrapper<'a, 'b, 'c> {
    pub builder: &'a mut api::DisplayListBuilder,
    pub context: Context,

    pub appearance: &'c PropertiesAppearance,
    pub layout: &'b Vec<FlexStyle>,
}

impl<'a, 'b, 'c> From<AppearanceWrapper<'a, 'b, 'c>> for WebrenderStyles {
    fn from(wrapper: AppearanceWrapper<'a, 'b, 'c>) -> WebrenderStyles {
        // BACKGROUND STYLE
        let background_raw = wrapper
            .appearance
            .0
            .get(&AppearanceKey::Background)
            .and_then(|value| extract!(Appearance::Background(_), value.clone()));

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
        let border_radius = properties_extract_radius(&wrapper.appearance);
        let border_radius = BorderRadiusWrapper {
            context: wrapper.context.clone(),
            border_radius,
        };

        // BORDERS STYLE
        let borders = properties_extract_borders(&wrapper.appearance, &wrapper.layout);
        let borders: WebrenderBorders = BorderStylesWrapper {
            context: wrapper.context.clone(),
            border_radius,
            borders,
        }
        .into();

        // TRANSFORMS
        let transforms = wrapper
            .appearance
            .0
            .get(&AppearanceKey::Transform)
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
