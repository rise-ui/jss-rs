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
//! use jss::traits::*;
//! use jss::types::*;
//!
//! fn main() -> Result<(), failure::Error> {
//!     let source = r#"
//!   {
//!     "borderTopRightRadius": 10,
//!     "borderTopStyle": "solid",
//!     "alignContent": "center",
//!     "borderTopWidth": 10,
//!     
//!     "filter": [
//!       "blur(20)"
//!     ],
//!     "transform": [
//!       "translate(10px,10%)",
//!       "rotate(40deg,15rad)"
//!     ]
//!   }
//!   "#;
//!
//!     let style = StyleBuilder::default().source(source).parse()?;
//!     println!("{:#?}", style);
//!
//!     Ok(())
//! }
//! ```
//!
//! Or if you need YAML...
//!
//! ``` rust
//! extern crate failure;
//! extern crate jss;
//!
//! use jss::traits::*;
//! use jss::types::*;
//!
//! fn main() -> Result<(), failure::Error> {
//!     let source = r#"
//! ---
//! borderTopRightRadius: 10
//! borderTopStyle: solid
//! alignContent: center
//! borderTopWidth: 10
//! filter:
//! - blur(20)
//! transform:
//! - translate(10px,10%)
//! - rotate(40deg,15rad)
//!   "#;
//!
//!     let style = StyleBuilder::default().source(source).source_type(SourceFormat::Yaml).parse()?;
//!     println!("{:#?}", style);
//!
//!     Ok(())
//! }
//! ```

#![feature(custom_attribute)]
#![feature(concat_idents)]
#![feature(never_type)]
#![feature(try_from)]
#![feature(test)]

#[macro_use]
extern crate failure_derive;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate jss_derive;

#[macro_use]
extern crate enum_extract;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate failure;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate maplit;
#[macro_use]
mod macros;

extern crate erased_serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate yaml_rust;
extern crate serde;

extern crate ordered_float;
extern crate euclid;

extern crate css_color_parser;
extern crate interpolation;
extern crate inflector;
extern crate palette;
extern crate regex;
extern crate yoga;
extern crate eval;
extern crate test;

pub extern crate webrender;

pub mod animation;
pub mod convert;
pub mod parser;
pub mod properties;
pub mod traits;
pub mod types;
pub mod utils;
