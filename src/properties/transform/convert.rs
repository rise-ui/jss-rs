use std::f32::consts::PI;
use euclid;

use properties::{
  PropertiesCollection,
  Transforms,
  Transform,
  Length,
  Angle,
};

use webrender::api::{
  LayoutPrimitiveInfo,
  DisplayListBuilder,
  PropertyBindingKey,
  PropertyBinding,
  LayoutTransform,
};

// @TODO: implement with convert sizes Length with percentage
pub fn make_translation(translate: (Length, Length)) -> LayoutTransform {
  LayoutTransform::create_translation(0., 0., 0.)
}

pub fn make_rotation(angle: Angle, size: (f32, f32)) -> LayoutTransform {
  let angle: euclid::Angle<f32> = angle.into();
  LayoutTransform::create_rotation(0., 0., 1., angle)
}

pub fn make_skew(x: Angle, y: Angle) -> LayoutTransform {
  let alpha: euclid::Angle<f32> = x.into();
  let beta: euclid::Angle<f32> = y.into();

  LayoutTransform::create_skew(alpha, beta)
}

pub fn transforms_multiply(transforms: Transforms, sizes: (f32, f32)) -> Option<LayoutTransform> {
  let transforms: Vec<LayoutTransform> = transforms.iter().map(|t| t.into_layout_transform(sizes)).collect();

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

pub fn transforms_push_to_builder(
  container: &LayoutPrimitiveInfo,
  transforms: Transforms,
  sizes: (f32, f32),
  tag: (String, u64),

  mut properties: PropertiesCollection,
  mut builder: DisplayListBuilder,
) -> (DisplayListBuilder, PropertiesCollection) {
  if let Some(transform) = transforms_multiply(transforms, sizes) {
    let binding_key = PropertyBindingKey::new(tag.1);
    let property_key = tag.0;

    // Add dynamic binding property
    properties.insert(property_key, binding_key);

    // Generate clip for transform area
    let property_transform = Some(PropertyBinding::Binding(binding_key, transform));
    let transformed_frame = builder.push_reference_frame(&container, property_transform, None);
    builder.push_clip_id(transformed_frame);
  }

  (builder, properties)
}

impl Transform {
  pub fn is_none(&self) -> bool {
    self.clone() == Transform::None
  }

  pub fn into_layout_transform(&self, size: (f32, f32)) -> LayoutTransform {
    match &self {
      Transform::Rotate(angle) => make_rotation(angle.clone(), size.clone()),
      Transform::Translate(translate) => make_translation(translate.clone()),
      Transform::Skew((x, y)) => make_skew(x.clone(), y.clone()),
      _ => LayoutTransform::create_translation(0., 0., 0.),
    }
  }
}
