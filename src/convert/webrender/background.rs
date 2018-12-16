use properties::{Background, GradientStop, Gradient};
use types::Context;
use webrender::api;

#[derive(Debug, Clone, PartialEq)]
pub enum WebrenderBackground {
    Gradient(api::Gradient),
    Color(api::ColorF),
}

pub struct GradientStopWrapper {
    pub stop: GradientStop,
    pub context: Context,
}

pub struct GradientWrapper<'a> {
    pub builder: &'a mut api::DisplayListBuilder,
    pub gradient: Gradient,
    pub context: Context,
}

pub struct BackgroundWrapper<'a> {
    pub builder: &'a mut api::DisplayListBuilder,
    pub background: Background,
    pub context: Context,
}

impl Into<api::GradientStop> for GradientStopWrapper {
    fn into(self) -> api::GradientStop {
        api::GradientStop {
            color: self.stop.color.into(),
            offset: self.stop.offset,
        }
    }
}

impl<'a> Into<api::Gradient> for GradientWrapper<'a> {
    fn into(self) -> api::Gradient {
        let dimensions: (f32, f32) = if let Some(layout) = self.context.dimensions.current {
            (layout.width(), layout.height())
        } else {
            (0.0, 0.0)
        };

        let from = api::LayoutPoint::new(dimensions.0 * self.gradient.from.0, dimensions.1 * self.gradient.from.1);

        let to = api::LayoutPoint::new(dimensions.0 * self.gradient.to.0, dimensions.1 * self.gradient.to.1);

        let stops: Vec<api::GradientStop> = self
            .gradient
            .stops
            .clone()
            .into_iter()
            .map(|stop| {
                GradientStopWrapper {
                    context: self.context.clone(),
                    stop,
                }
                .into()
            })
            .collect::<Vec<_>>();

        self.builder.create_gradient(from, to, stops, api::ExtendMode::Clamp)
    }
}

impl<'a> From<BackgroundWrapper<'a>> for WebrenderBackground {
    fn from(wrapper: BackgroundWrapper<'a>) -> WebrenderBackground {
        match wrapper.background {
            Background::Color(color) => WebrenderBackground::Color(color.into()),

            Background::Gradient(gradient) => WebrenderBackground::Gradient(
                GradientWrapper {
                    context: wrapper.context,
                    builder: wrapper.builder,
                    gradient,
                }
                .into(),
            ),
        }
    }
}
