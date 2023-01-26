`fast_config`
=============
---

[<img alt="github" src="https://img.shields.io/badge/github-fast_config-brightgreen.svg?logo=github&style=for-the-badge"/>](https://github.com/FlooferLand/fast_config)
[<img alt="crates.io" src="https://img.shields.io/crates/v/fast_config?logo=rust&style=for-the-badge"/>](https://crates.io/crates/fast_config)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fast_config-988.svg?logo=rust&style=for-the-badge"/>](https://docs.rs/fast_config)
<br style="display: block; margin: 0 0; content: '---'" />
[<img alt="license" src="https://img.shields.io/github/license/FlooferLand/fast_config?style=flat"/>](https://github.com/FlooferLand/fast_config/blob/main/LICENSE)
[<img alt="code size" src="https://img.shields.io/github/languages/code-size/FlooferLand/fast_config?style=flat"/>](https://www.youtube.com/watch?v=dQw4w9WgXcQ)
[<img alt="issues" src="https://img.shields.io/github/issues/FlooferLand/fast_config?label=open%20issues&style=flat"/>](https://github.com/FlooferLand/fast_config/issues)
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FlooferLand/fast_config/rust.yml)

A small, safe, lightweight, and easy-to-use Rust crate to read and write to config files.

Currently only supports: JSON5, TOML, and YAML.
But more formats (such as RON) are planned to be added later.

*[- click here to view code examples](#examples)*
<br style="display: block; margin: 0 0; content: '---'" />
*[- click here to jump to the Getting Started section](#getting-started)*

---

## What is this crate?
`fast_config` was made to be a faster to set up, more light-weight, statically typed alternative to [config](https://crates.io/crates/config).
It also manages to have it's own benefits compared to some other config-reading crates,
as there is full support for writing/saving config files
and it also provides you with *some* options regarding styling your config files

---

### Why this crate?
- It's small and fast *(uses compile-time features to remove/add code)*
- It's safe and robust *(uses Rust's structs to store data, instead of HashMaps)*
- Ridiculously simple to use *(only takes 3 lines of short code to make a config file, write/read something, and save it)*

### Why not this crate?
- It's not usable if you don't know the way the data will look like

### ⚠ Documentation and tests are still being made! ⚠
This crate has now entered the 'stable' stage, i however haven't battle-tested this in any big projects,
so while there will NOT be any panics or crashes,
some user-side error handling in particular might be a bit bugged

Documentation might be a little weird or incomplete at the current moment, too.

---

## Examples:
```rust,ignore
use fast_config::Config;
use serde::{Serialize, Deserialize};

// Creating a config struct to store our data
#[derive(Serialize, Deserialize)]
pub struct MyData {
    pub student_debt: i32
}

fn main() {
    // Initializing a logging system (needed to show some warnings/errors)
    env_logger::init();

    // Creating our data (default values)
    let data = MyData {
        student_debt: 20
    };

    // Creating a new config struct with our data struct
    let mut config = Config::new("./config/myconfig.json5", data).unwrap();

    // Read/writing to the data
    println!("I am ${} in debt", config.data.student_debt);
    config.data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

    // Saving it back to the disk
    config.save().unwrap();
}
```

## Getting started

1. Add the crate to your project via <br/> `cargo add fast_config`
---

3. Enable the feature(s) for the format(s) you'd like to use <br/>
   - Currently only `json5`, `toml`, and `yaml` are supported <br/>
---
4. Create a struct to hold your data that derives `serde::Serialize` and `serde::Deserialize`
---
5. Create an instance of your data struct
- Optionally `use` the crate's `Config` type for convenience <br/>
  `use fast_config::Config;`
---
6. Use <br/>
   ```rust,ignore
   let my_config = Config::new("..", your_data).unwrap();
   ```
   to create and store your config file(s)!
Alternatively you could also use `Config::from_settings` to style some things!

---

View the [examples](./examples) directory for more advanced examples.
