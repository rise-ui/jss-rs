#[cfg(feature = "webrender_support")]
use properties::{Angle, Length, Transform, Transforms, PropertiesCollection};
#[cfg(feature = "webrender_support")]
use webrender::api::{DisplayListBuilder, LayoutPrimitiveInfo, LayoutTransform, PropertyBinding, PropertyBindingKey};
#[cfg(feature = "webrender_support")]
use std::collections::HashMap;
#[cfg(feature = "webrender_support")]
use euclid;

#[cfg(feature = "webrender_support")]
pub type PropertiesCollection = HashMap<String, PropertyBindingKey<LayoutTransform>>;

// @TODO: implement with convert sizes Length with percentage
#[cfg(feature = "webrender_support")]
pub fn make_translation(translate: (Length, Length)) -> LayoutTransform {
  LayoutTransform::create_translation(0., 0., 0.)
}

#[cfg(feature = "webrender_support")]
pub fn make_rotation(angle: Angle, size: (f32, f32)) -> LayoutTransform {
  let angle: euclid::Angle<f32> = angle.into();
  let center_x = size.0 / 2.0;
  let center_y = size.1 / 2.0;

  LayoutTransform::create_rotation(0., 0., 1., angle)
}

#[cfg(feature = "webrender_support")]
pub fn make_skew(x: Angle, y: Angle) -> LayoutTransform {
  let alpha: euclid::Angle<f32> = x.into();
  let beta: euclid::Angle<f32> = y.into();

  LayoutTransform::create_skew(alpha, beta)
}

#[cfg(feature = "webrender_support")]
pub fn transforms_multiply(transforms: Transforms, sizes: (f32, f32)) -> Option<LayoutTransform> {
  let transforms: Vec<LayoutTransform> = transforms.iter().map(|t| into_transform3d(t.clone(), sizes)).collect();

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

#[cfg(feature = "webrender_support")]
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

#[cfg(feature = "webrender_support")]
pub fn into_transform3d(transform: Transform, size: (f32, f32)) -> LayoutTransform {
  match transform {
    Transform::Translate(translate) => make_translation(translate),
    Transform::Rotate(angle) => make_rotation(angle, size),
    Transform::Skew((x, y)) => make_skew(x, y),
    _ => LayoutTransform::create_translation(0., 0., 0.),
  }
}
