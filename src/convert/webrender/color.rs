use webrender::api::ColorF;
use properties::Color;

impl Into<ColorF> for Color {
    fn into(self) -> ColorF {
        let color =
            ColorF::new(self.green as f32 / 255.0, self.blue as f32 / 255.0, self.red as f32 / 255.0, self.alpha);

        color
    }
}
