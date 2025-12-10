## A tutorial on how to convert your data to the newest version!
This file targets helping you switch to the newest version, from the last version.

___Note:___ You're able to look at the [commit history of this file](https://github.com/FlooferLand/fast_config/commits/main/CONVERSION_TUTORIAL.md) to see past versions of this file, for older versions of the crate.

This tutorial currently targets conversion from **1.2** to **1.3**.

# Changes
___Note:___ Always check the [GitHub version](https://github.com/FlooferLand/fast_config/blob/main/CONVERSION_TUTORIAL.md) of this file,
as it's the most up to date.
---

1.3 makes it so you now need to derive `FastConfig` for your data struct

This was _(thankfully done)_ by [@vaytea](https://github.com/vaytea) as having to type `config.data` every single time you wanted to write or read to fields was very annoying

### Guide

#### Old:
```rust
use fast_config::Config;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct MyData {
    pub student_debt: i32
}

// Create the data with default values
let data = MyData {
    student_debt: 20
};

// Create a new config struct with our data struct
let mut config = Config::new("./config/myconfig.json5", data).unwrap();

// Read/write to the data
println!("I am ${} in debt", config.data.student_debt);
config.data.student_debt = i32::MAX;
println!("Oh no, i am now ${} in debt!!", config.data.student_debt);

// Save it back to the disk
config.save().unwrap();
```

### New:
```rust
use fast_config::FastConfig;
use fast_config::Format;
use serde::{Serialize, Deserialize};

// Create a config struct and derive FastConfig
#[derive(Serialize, Deserialize, FastConfig)]
pub struct MyData {
    pub student_debt: i32,
}

// Create the data with default values
let mut data = MyData {
   student_debt: 20
};

// Save to create the file
data.save("./config/myconfig.json5", Format::JSON5).unwrap();

// Load from the file
data.load("./config/myconfig.json5", Format::JSON5).unwrap();

// Read/write to the data
println!("I am {}$ in debt", data.student_debt);
data.student_debt = i32::MAX;
println!("Oh no, i am now {}$ in debt!!", data.student_debt);

// Save it back to the disk
data.save("./config/myconfig.json5", Format::JSON5).unwrap();
```
