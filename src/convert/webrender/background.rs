use properties::{Background, GradientStop, Gradient};
use types::Context;
use webrender::api;

pub enum WebrenderBackground {
    Gradient(api::Gradient),
    Color(api::ColorF),
}

pub struct GradientStopWrapper {
    stop: GradientStop,
    context: Context,
}

pub struct GradientWrapper<'a> {
    builder: &'a mut api::DisplayListBuilder,
    gradient: Gradient,
    context: Context,
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

impl Into<WebrenderBackground> for (&mut api::DisplayListBuilder, Context, Background) {
    fn into(self) -> WebrenderBackground {
        match self.2 {
            Background::Color(color) => WebrenderBackground::Color(color.into()),

            Background::Gradient(gradient) => WebrenderBackground::Gradient(
                GradientWrapper {
                    context: self.1,
                    builder: self.0,
                    gradient,
                }
                .into(),
            ),
        }
    }
}
