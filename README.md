## JSS Rust
Implementation of JSS for Rust. Use css styles without css with identical properties.
*Part of Rise-UI project.*

### Usage
Add to your Cargo.toml.
``` toml
jss = { git = "https://github.com/rise-ui/jss-rs" }
```

#### Examples
``` rust
// Simple stylesheet parsing example
extern crate failure;
extern crate jss;

use failure::Error;
use jss::*;

fn main() -> Result<(), Error> {
  let style = r#"{
    "element": {
      "align_content": "center",
      "border_top_right_radius": 10,
      "border_top_style": "solid",
      "filter": [
        "blur(20)"
      ]
    },

    "element:hover": {
      "align_content": "flex_start",
      "background": "rgba(130,130,130,0)"
    }
  }"#;

  let result = parse_json_stylesheet(style.to_string())?;
  println!("Stylesheet: \n{:#?}", result);

  Ok(())
}
```

### Roadmap
- [x] All layout properties of css3 with yoga compability 
- [x] Merge `ElementStyle`-s by `ops::Add`
- [x] Parsing `filters`, `color`, `unit` from CSS3
- [x] Stylesheet with selectors by status aka "name:hover", "name:active" etc.
- [x] Get formatted props for `yoga-rs` and collect `apperance` styles
- [x] Convert to `webrender` types & layers as optional target (partially implemented)
- [ ] Parsing `transform` property
- [ ] Parsing `media-query` property
- [ ] Support `runtime checkers` & `middlewares` in runtime 