#![feature(test)]
extern crate test;

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate prepare_style_derive;
#[cfg(feature = "webrender_support")]
extern crate webrender;

extern crate failure;
extern crate ordered_float;
extern crate serde_json;

extern crate css_color_parser;
extern crate regex;
extern crate serde;
extern crate yoga;

mod common;
mod properties;

pub use common::*;

#[cfg(test)]
mod tests {
  use super::*;
  use test::Bencher;

  #[bench]
  fn parse_and_prepare(bench: &mut Bencher) {
    let style = r#"{
      "align_content": "center"
    }"#;

    bench.iter(|| {
      let parsed = parse_json_style(style.to_string()).unwrap();
      parsed.get_prepared_styles();
    });
  }
}
