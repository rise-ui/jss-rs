use hashbrown::HashMap;
use std::hash::Hash;
use types::Context;

use properties::{
  Transforms,
};

use webrender::api::{
  LayoutPrimitiveInfo,
  DisplayListBuilder,
  PropertyBindingKey,
  PropertyBinding,
  LayoutTransform,
  TransformStyle,
};

pub type PropertiesCollection<T: Hash + Eq> = HashMap<T, PropertyBindingKey<LayoutTransform>>;

#[derive(Debug, Clone, PartialEq)]
pub struct TransformsWrapper {
    pub transforms: Transforms,
    pub context: Context,
}

pub mod transforms {
    use properties::{Length, Angle, Transform, Transforms};
    use webrender::api::LayoutTransform;

    // @TODO: implement with convert sizes Length with percentage
    pub fn translation(translate: (Length, Length)) -> LayoutTransform {
        LayoutTransform::create_translation(0., 0., 0.)
    }

    pub fn rotation(angle: Angle, size: (f32, f32)) -> LayoutTransform {
        let angle: euclid::Angle<f32> = angle.into();
        LayoutTransform::create_rotation(size.0 / 2., size.1 / 2., 0., angle)
    }

    pub fn skew(x: Angle, y: Angle) -> LayoutTransform {
        let alpha: euclid::Angle<f32> = x.into();
        let beta: euclid::Angle<f32> = y.into();

        LayoutTransform::create_skew(alpha, beta)
    }

    pub fn transform(transform: &Transform, size: (f32, f32)) -> LayoutTransform {
        match &transform {
            Transform::Rotate(angle) => rotation(angle.clone(), size.clone()),
            Transform::Translate(translate) => translation(translate.clone()),
            Transform::Skew((x, y)) => skew(x.clone(), y.clone()),
            _ => LayoutTransform::create_translation(0., 0., 0.),
        }
    }

    pub fn multiply(transforms: Transforms, sizes: (f32, f32)) -> Option<LayoutTransform> {
        let transforms: Vec<LayoutTransform> = transforms.iter().map(|t| transform(&t, sizes)).collect();

        if transforms.len() > 0 {
            if transforms.len() > 1 {
                let mut starter_origin = transforms[0];
                for i in 1..transforms.len() {
                    starter_origin = starter_origin.post_mul(&transforms[i]);
                }

                Some(starter_origin)
            } else {
                Some(transforms[0])
            }
        } else {
            None
        }
    }
}