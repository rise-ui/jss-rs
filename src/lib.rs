//! ## JSS Rust
//! Implementation of JSS for Rust. Use css styles without css with identical properties.
//! *Part of Rise-UI project.*
//!
//! ### [Documentation](http://friktor.github.io/jss-rs/jss/index.html)
//!
//! ### Usage
//! Add to your Cargo.toml.
//! ``` toml
//! [dependencies]
//! jss = { git = "https://github.com/rise-ui/jss-rs" }
//! ```
//!
//! ``` rust
//! extern crate jss;
//! ```
//!
//! #### Examples
//! Simple JSON element parsing example
//!
//! ``` rust
//! extern crate failure;
//! extern crate jss;
//!
//! use failure::Error;
//! use jss::*;
//!
//! fn main() -> Result<(), Error> {
//!   let style = r#"{
//!     "borderTopRightRadius": 10,
//!     "borderTopStyle": "solid",
//!     "alignContent": "center",
//!     "borderTop": 10,
//!     
//!     "filter": [
//!       "blur(20)"
//!     ],
//!
//!     "transform": [
//!       "translate(10px,10%)",
//!       "rotate(40deg,15rad)"
//!     ]
//!   }"#;
//!
//!   let result = Style::parse_element(style, ParseOptions::default())?;
//!   println!("{:#?}", result);
//!   Ok(())
//! }
//! ```
//!
//! Or if you need YAML...
//!
//! ``` rust
//! // Simple JSON element parsing example
//! extern crate failure;
//! extern crate jss;
//!
//! use failure::Error;
//! use jss::*;
//!
//! fn main() -> Result<(), Error> {
//!   let style = r#"---
//! borderTopRightRadius: 10
//! borderTopStyle: solid
//! alignContent: center
//! borderTop: 10
//! filter:
//! - blur(20)
//! transform:
//! - translate(10px,10%)
//! - rotate(40deg,15rad)
//! "#;
//!
//!   let mut options = ParseOptions::default();
//!   options.from = ParseTarget::Yaml;
//!
//!   let result = Style::parse_element(style, options)?;
//!   println!("{:#?}", result);
//!   Ok(())
//! }
//! ```

#![feature(extern_prelude)]
#![feature(test)]
extern crate test;

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate enum_extract;
#[macro_use]
extern crate jss_derive;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate maplit;
#[macro_use]
extern crate nom;

extern crate css_color_parser;
extern crate ordered_float;
extern crate serde_yaml;
extern crate serde_json;
extern crate webrender;
extern crate inflector;
extern crate euclid;
extern crate regex;
extern crate serde;
extern crate yoga;

pub mod common;
pub mod properties;

pub use properties::Apperance;
pub use common::*;
