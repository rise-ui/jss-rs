use interpolation::*;
use std::fmt;

/// Transition function - use may using ready-made transitions or specify own bezier transition
#[derive(Clone, PartialEq)]
pub enum Easing {
    // In percentage
    CubikBezier(f32, f32, f32, f32),
    Basic(EaseFunction),
}

impl Easing {
    // Process value by self function
    pub fn transition(&self, start: &f32, end: &f32, progress: &f32) -> f32 {
        use self::Easing::*;

        match *self {
            Basic(transition) => lerp(start, end, &progress.calc(transition)),
            // @todo: implement cubik transition
            _ => 0.0,
        }
    }
}

impl fmt::Debug for Easing {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Easing::CubikBezier(x0, x1, x2, x3) => format!("CubikBezier({}, {}, {}, {})", x0, x1, x2, x3),

            Easing::Basic(name) => {
                let func = match name {
                    EaseFunction::QuadraticIn => "QuadraticIn",
                    EaseFunction::QuadraticOut => "QuadraticOut",
                    EaseFunction::QuadraticInOut => "QuadraticInOut",
                    EaseFunction::CubicIn => "CubicIn",
                    EaseFunction::CubicOut => "CubicOut",
                    EaseFunction::CubicInOut => "CubicInOut",
                    EaseFunction::QuarticIn => "QuarticIn",
                    EaseFunction::QuarticOut => "QuarticOut",
                    EaseFunction::QuarticInOut => "QuarticInOut",
                    EaseFunction::QuinticIn => "QuinticIn",
                    EaseFunction::QuinticOut => "QuinticOut",
                    EaseFunction::QuinticInOut => "QuinticInOut",
                    EaseFunction::SineIn => "SineIn",
                    EaseFunction::SineOut => "SineOut",
                    EaseFunction::SineInOut => "SineInOut",
                    EaseFunction::CircularIn => "CircularIn",
                    EaseFunction::CircularOut => "CircularOut",
                    EaseFunction::CircularInOut => "CircularInOut",
                    EaseFunction::ExponentialIn => "ExponentialIn",
                    EaseFunction::ExponentialOut => "ExponentialOut",
                    EaseFunction::ExponentialInOut => "ExponentialInOut",
                    EaseFunction::ElasticIn => "ElasticIn",
                    EaseFunction::ElasticOut => "ElasticOut",
                    EaseFunction::ElasticInOut => "ElasticInOut",
                    EaseFunction::BackIn => "BackIn",
                    EaseFunction::BackOut => "BackOut",
                    EaseFunction::BackInOut => "BackInOut",
                    EaseFunction::BounceIn => "BounceIn",
                    EaseFunction::BounceOut => "BounceOut",
                    EaseFunction::BounceInOut => "BounceInOut",
                };

                format!("EaseFunction({})", func)
            }
        };

        f.write_str(message.as_str())
    }
}
