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

use jss::traits::*;
use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let source = r#"
  {
    "borderTopRightRadius": 10,
    "borderTopStyle": "solid",
    "alignContent": "center",
    "borderTopWidth": 10,
    
    "filter": [
      "blur(20)"
    ],
    "transform": [
      "translate(10px,10%)",
      "rotate(40deg,15rad)"
    ]
  }
  "#;

    let style = StyleBuilder::default().source(source).parse()?;
    println!("{:#?}", style);

    Ok(())
}
```

Or if you need YAML...

``` rust
extern crate failure;
extern crate jss;

use jss::traits::*;
use jss::types::*;

fn main() -> Result<(), failure::Error> {
    let source = r#"
---
borderTopRightRadius: 10
borderTopStyle: solid
alignContent: center
borderTopWidth: 10
filter:
- blur(20)
transform:
- translate(10px,10%)
- rotate(40deg,15rad)
  "#;

    let style = StyleBuilder::default().source(source).source_type(SourceFormat::Yaml).parse()?;
    println!("{:#?}", style);

    Ok(())
}
```

### Roadmap
- [x] Parsing `filters`, `color`, `unit` and other properties from CSS3
- [x] Stylesheet with selectors by status aka "name:hover", "name:active" etc.
- [x] Get formatted props for `yoga-rs` and collect `appearance` styles
- [x] Convert to `webrender` types & layers as optional target (partially implemented)
- [x] Middlewares
  - [x] Parsing Middleware
  - [ ] Runtime Middleware (WIP)
- [ ] Runtime calculator like as `calc()` function in css3
- [x] Parsing `transform` property
- [ ] Parsing `media-query` property 