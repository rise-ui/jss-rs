#![feature(concat_idents)]
#![feature(never_type)]
#![feature(try_from)]

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
mod macros;

extern crate serde_json;
extern crate serde_yaml;
extern crate serde;

extern crate ordered_float;
extern crate euclid;

extern crate css_color_parser;
extern crate inflector;
extern crate maplit;
extern crate regex;
extern crate yoga;

pub mod parser;
pub mod properties;
pub mod traits;
pub mod types;
pub mod utils;
