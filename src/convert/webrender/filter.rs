use webrender::api::FilterOp;
use properties::Filter;

impl Into<FilterOp> for Filter {
    fn into(self) -> FilterOp {
        use self::Filter::*;

        match self {
            Brightness(v) => FilterOp::Brightness(v),
            Grayscale(v) => FilterOp::Grayscale(v),
            HueRotate(v) => FilterOp::HueRotate(v),
            Saturate(v) => FilterOp::Saturate(v),
            Contrast(v) => FilterOp::Contrast(v),
            Invert(v) => FilterOp::Invert(v),
            Sepia(v) => FilterOp::Sepia(v),
            Blur(v) => FilterOp::Blur(v),
            None => FilterOp::Blur(0.),
        }
    }
}
