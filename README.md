## JSS Rust
[![Build Status](https://travis-ci.org/rise-ui/jss-rs.svg?branch=master)](https://travis-ci.org/rise-ui/jss-rs)

Implementation of JSS for Rust. Use css styles without css with identical properties.
*Part of Rise-UI project.*

### [Documentation](http://friktor.github.io/jss-rs/jss/index.html)

### Usage
Add to your Cargo.toml.
``` toml
[dependencies]
jss = { git = "https://github.com/rise-ui/jss-rs" }
```

``` rust
extern crate jss;
```

#### Examples
Simple JSON element parsing example

``` rust
extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

fn main() -> Result<(), Error> {
  let style = r#"{
    "borderTopRightRadius": 10,
    "borderTopStyle": "solid",
    "alignContent": "center",
    "borderTop": 10,
    
    "filter": [
      "blur(20)"
    ],

    "transform": [
      "translate(10px,10%)",
      "rotate(40deg,15rad)"
    ]
  }"#;

  let result = Style::parse_element(style, ParseOptions::default())?;
  println!("{:#?}", result);
  Ok(())
}
```

Or if you need YAML...

``` rust
// Simple JSON element parsing example
extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

fn main() -> Result<(), Error> {
  let style = r#"---
borderTopRightRadius: 10
borderTopStyle: solid
alignContent: center
borderTop: 10
filter:
- blur(20)
transform:
- translate(10px,10%)
- rotate(40deg,15rad)
"#;

  let mut options = ParseOptions::default();
  options.from = SourceFormat::Yaml;

  let result = Style::parse_element(style, options)?;
  println!("{:#?}", result);
  Ok(())
}
```

### Roadmap
- [x] Parsing `filters`, `color`, `unit` and other properties from CSS3
- [x] Stylesheet with selectors by status aka "name:hover", "name:active" etc.
- [x] Get formatted props for `yoga-rs` and collect `appearance` styles
- [x] Convert to `webrender` types & layers as optional target (partially implemented)
- [x] Parsing `transform` property
- [ ] Parsing `media-query` property
- [ ] Support `runtime checkers` & `middlewares` in runtime 