fast_config
=============
---

[<img alt="github" src="https://img.shields.io/badge/github-fast_config-lightgray.svg?logo=github&style=for-the-badge"/>](https://github.com/FlooferLand/fast_config)
[<img alt="crates.io" src="https://img.shields.io/crates/v/fast_config?logo=rust&style=for-the-badge"/>](https://crates.io/crates/fast_config)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fast_config-lightgray.svg?logo=rust&style=for-the-badge"/>](https://docs.rs/fast_config)
<br style="display: block; margin: 0 0; content: '---'" />
[<img alt="license" src="https://img.shields.io/github/license/FlooferLand/fast_config?style=flat"/>](https://github.com/FlooferLand/fast_config/blob/main/LICENSE)
[<img alt="code size" src="https://img.shields.io/github/languages/code-size/FlooferLand/fast_config?style=flat"/>](https://www.youtube.com/watch?v=dQw4w9WgXcQ)
[<img alt="issues" src="https://img.shields.io/github/issues/FlooferLand/fast_config?label=open%20issues&style=flat"/>](https://github.com/FlooferLand/fast_config/issues)

A small, lightweight, and easy-to-use Rust crate to handle config files.

Currently only supports: JSON5, TOML, and YAML.
But more formats are planned to be added later.

*[- click here to view code examples](#examples)*
<br style="display: block; margin: 0 0; content: '---'" />
*[- click here to jump to the Getting Started section](#getting-started)*

---

## What is this crate?
This crate was originally called `simple_config`, but I had to change
the name due to a crate having the exact same name [*(linked here)*](https://crates.io/crates/simple_config).

This crate was made to be a faster to set up, more light-weight, statically typed alternative to [config](https://crates.io/crates/config).
But it also accidentally manages to beat [simple_config](https://crates.io/crates/simple_config) in the lightweightness and "fast to set up" category.

The lead benefit of this crate is how fast you can get it set up;
It also doesn't sacrifice performance for readability *(see some examples at [# Examples](#examples))*

---

### Why this crate?
- It's small and fast *(uses compile-time features to remove any unnecessary code)*
- It's safe and robust *(uses Rust's structs to store data, instead of HashMaps)*
- Ridiculously simple to use *(only takes 3 lines of short code to make a config file, write/read something, and close it)*

### Why not this crate?
- You can't currently use different file formats in the same project <br/>
This is an intentional design choice to be able to
shrink down the code as much as possible. <br/>
And since this crate is mostly made for the global-scope config file a program might use,
I didn't really see that as necessary. <br/>
*Feel free to open an issue if you'd like this to be changed!*

### ⚠ Documentation and tests are still being made! ⚠
I'm still working on this crate, and it's in somewhat-early-access.
While I haven't managed to find any bugs, documentation might be a little weird or incomplete at the current moment.

---

## Examples:
```rust,ignore
use serde::{Serialize, Deserialize};
use fast_config::Config;

// Creating a config struct to store our data
#[derive(Serialize, Deserialize)]
pub struct MyData {
    #[serde(default = "MyDataDefaults::student_debt")]
    pub student_debt: i32
}

// Storing the default values for our data
pub struct MyDataDefaults;
impl MyDataDefaults {
    pub fn student_debt() -> i32 { 20 }
}

fn main() {
    // Initializing a logging system (needed to show errors)
    env_logger::init();

    // Creating a new config struct with our data struct (it can also guess the file extension)
    let mut config = Config::<MyData>::new("./config/myconfig");

    // Read/writing to the data
    println!("I am ${} in debt", config.data.student_debt);
    config.data.student_debt = i32::MAX;
    println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

    // Saving it back to the disk
    config.save();
}
```

## Getting started

1. Add the crate to your project <br/> `cargo add fast_config`
---
2. Enable a feature for the format you'd like to use <br/>
   - Currently only `json5`, `toml`, and `yaml` are supported <br/>
   - Please note that currently only *one* feature can be enabled at the same time
---
3. Create a struct for your data that derives `serde::Serialize` and `serde::Deserialize`
---
4. Use <br/>
   `let my_config = Config::<MyData>::new("..");` <br/>
   to create and store your config file(s)!

---

View the [examples](./examples) directory for more advanced examples.
