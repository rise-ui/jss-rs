//! # JSS
//! Implementation of JSS for Rust. Use css styles without css with identical properties.
//! *Part of Rise-UI project.*
//!
//! ### Usage
//! Add to your Cargo.toml.
//! ```toml
//! [dependencies]
//! jss = { git = "https://github.com/rise-ui/jss-rs" }
//! ```
//!
//! ```rust
//! extern crate jss;
//! ```
//!
//! #### Examples
//! ``` rust
//! // Simple stylesheet parsing example
//! extern crate failure;
//! extern crate jss;
//!
//! use failure::Error;
//! use jss::*;
//!
//! fn main() -> Result<(), Error> {
//!   let style = r#"{
//!     "element": {
//!       "align_content": "center",
//!       "border_top_right_radius": 10,
//!       "border_top_style": "solid",
//!       "filter": [
//!         "blur(20)"
//!       ]
//!     },
//!
//!     "element:hover": {
//!       "align_content": "flex_start",
//!       "background": "rgba(130,130,130,0)"
//!     }
//!   }"#;
//!
//!   let result = parse_json_stylesheet(style)?;
//!   println!("Stylesheet: \n{:#?}", result);
//!
//!   Ok(())
//! }
//! ```

#![feature(test)]
extern crate test;

#[macro_use]
extern crate jss_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate enum_extract;

extern crate euclid;
extern crate failure;
extern crate ordered_float;
extern crate serde_json;
extern crate webrender;

extern crate css_color_parser;
extern crate regex;
extern crate serde;
extern crate yoga;

pub mod common;
pub mod properties;

pub use common::{
  parse_json_style, parse_json_stylesheet, ElementStyle, PrepareStyleExt, Style, Stylesheet,
};
pub use properties::Apperance;

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
      let parsed = parse_json_style(style).unwrap();
      parsed.get_prepared_styles();
    });
  }
}
