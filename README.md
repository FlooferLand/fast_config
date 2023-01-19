fast_config
=============
---

[<img alt="github" src="https://img.shields.io/badge/github-fast_config-lightgray.svg?logo=github&style=for-the-badge"/>](https://github.com/FlooferLand/fast_config)
[<img alt="crates.io" src="https://img.shields.io/crates/v/fast_config?style=for-the-badge"/>](https://crates.io/crates/fast_config)
<br style="display: block; margin: 0 0; content: '---'" />
[<img alt="license" src="https://img.shields.io/github/license/FlooferLand/fast_config?style=flat"/>](https://github.com/FlooferLand/fast_config/blob/main/LICENSE)
[<img alt="code size" src="https://img.shields.io/github/languages/code-size/FlooferLand/fast_config?style=flat"/>](https://www.youtube.com/watch?v=dQw4w9WgXcQ)
[<img alt="issues" src="https://img.shields.io/github/issues/FlooferLand/fast_config?label=open%20issues&style=flat"/>](https://github.com/FlooferLand/fast_config/issues)

A small and easy-to-use Rust crate to handle config files.
Currently only supports: JSON5, TOML, and YAML.
But more formats are planned to be added later.

---

### UPDATE:
I've accidentally stumbled across a crate with the exact same name my crate had,
made by someone with way more experience than me,
who's crate also does the exact same things but better.

I swear I checked Crates.io right before naming my project `simple_config`

Either way, this project is now called `fast_config`, because it's the only name available.
Feel free to check out the other, way better `simple_config` [here](https://crates.io/crates/simple_config).
*(I don't have any affiliation with it, only bad luck by crates.io somehow bugging out when i chose the original name)*

**I will still continue work on this project, for use in my own projects,
but I wouldn't recommend other people using it XD**

---

<br/>

<br/>

<br/>

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
