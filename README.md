fast_config
=============
---

![github](https://img.shields.io/badge/github-fast_config-lightgray.svg?logo=github&style=for-the-badge)
![crates.io](https://img.shields.io/crates/v/fast_config?style=for-the-badge)
<br style="display: block; margin: 0 0; content: '---'" />
![GitHub](https://img.shields.io/github/license/flooferland/fast_config?flat)
![gitHub code size in bytes](https://img.shields.io/github/languages/code-size/flooferland/fast_config?style=flat)
![gitHub issues](https://img.shields.io/github/issues/flooferland/fast_config?label=open%20issues&style=flat)

A small and easy-to-use Rust crate to handle config files.
Currently only supports: JSON5, TOML, and YAML.
But more formats are planned to be added later.

---

## Why this crate?
- It's small and fast *(uses compile-time features to remove any unnecessary code)*
- It's safe and robust *(uses Rust's structs to store data, instead of HashMaps)*
- Ridiculously simple to use *(only takes 3 lines of short code to make a config file, write/read something, and close it)*

### ⚠ Documentation and tests are still being made! ⚠
I'm still working on this crate, and it's in somewhat-early-access.
While I haven't managed to find any bugs myself, documentation might be a little weird or uncomplete at the current moment.

---

## Examples:
```rust
use fast_config::Config;
use serde::{Serialize, Deserialize};

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

---

View the [examples](./examples) directory for more advanced examples.
