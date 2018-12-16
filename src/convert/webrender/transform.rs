use std::collections::HashMap;
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

pub type PropertiesCollection = HashMap<String, PropertyBindingKey<LayoutTransform>>;

pub struct TransformsWrapper {
    pub transforms: Transforms,
    pub context: Context,
}

pub mod transform {
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

impl TransformsWrapper {
    pub fn push_builder(
        &self,
        container: &LayoutPrimitiveInfo,
        tag: (String, u64),

        properties: &mut PropertiesCollection,
        builder: &mut DisplayListBuilder,
    ) {
        let dimensions: (f32, f32) = if let Some(layout) = self.context.dimensions.current {
            (layout.width(), layout.height())
        } else {
            (0.0, 0.0)
        };

        if let Some(transform) = transform::multiply(self.transforms.clone(), dimensions) {
            let binding_key = PropertyBindingKey::new(tag.1);
            let property_key = tag.0;

            // Add dynamic binding property
            properties.insert(property_key, binding_key);

            // Generate clip for transform area
            let property_transform = Some(PropertyBinding::Binding(binding_key, transform));
            let transformed_frame =
                builder.push_reference_frame(&container, TransformStyle::Flat, property_transform, None);
            
            builder.push_clip_id(transformed_frame);
        }
    }
}
