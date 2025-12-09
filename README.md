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
![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/FlooferLand/fast_config/main_test.yml)

A small, safe, lightweight, and easy-to-use Rust crate to read and write to config files.

Currently supports:
[JSON](https://crates.io/crates/serde_json), [JSON5](https://crates.io/crates/json5), [TOML](https://crates.io/crates/toml), and [YAML](https://crates.io/crates/serde_yml).

But more [Serde](https://serde.rs/)-supported formats *(such as RON)* are planned to be added later.

### Useful teleports:
- [Migrating to a newer version of the crate](https://github.com/FlooferLand/fast_config/blob/main/CONVERSION_TUTORIAL.md)
- [Code examples](#examples)
- [Getting Started](#getting-started)
- [Things that need work (for contributors!)](./CONTRIBUTORS.md)

## What is this crate?

`fast_config` was made to be a faster to set up, more light-weight, statically typed alternative to [config](https://crates.io/crates/config).

It also manages to have its own benefits compared to some other config-reading crates
as there is full support for writing/saving config files,
and it also provides you with *some* options regarding styling your config files

---

### Why this crate?
- It's small and fast *(uses compile-time features to remove/add code)*
- It's safe and robust *(uses Rust's structs to store data, instead of HashMaps)*
- Ridiculously simple to use *(only takes 3 lines of short code to make a config file, write/read something, and save it)*

### Why not this crate?
1. It doesn't work if you don't know the way your data will be formatted<br>
   *(for example if you want your users to be able to have any keys ranging from `key0` to `key9000` in an object)*
2. It cannot currently understand the RON file format
3. It cannot currently save comments in config files.

---

**2** and **3** _are_ going to be addressed with future updates, however.

### ⚠ Documentation and tests are still being made! ⚠
This crate is now stable, I however haven't battle-tested this in any humongous projects,
so while there will NOT be any panics or crashes, some weird things might happen at scale.

Documentation might be a little weird or incomplete at the current moment, too.

Feel free to contribute any fixes by [opening up an issue](https://github.com/FlooferLand/fast_config/issues) if you find
anything that isn't working as expected!

---

## Examples:

### Basic Usage

```rust
use fast_config::FastConfig;
use fast_config::Format;
use serde::Serialize;
use serde::Deserialize;

// Create a config struct and derive FastConfig
#[derive(Serialize, Deserialize, FastConfig)]
pub struct MyData {
    pub student_debt: i32,
}

let config_path = "test/myconfig.json5";
// Create data with default values
let mut data = MyData {
   student_debt: 20
};
// Save to create the file
data.save(&config_path, Format::JSON5).unwrap();
// Load the data from the file
data.load(&config_path, Format::JSON5).unwrap();

// Read/write to the data
println!("I am {}$ in debt", data.student_debt);
data.student_debt = i32::MAX;
println!("Oh no, i am now {}$ in debt!!", data.student_debt);

// Save it back to disk
data.save(&config_path, Format::JSON5).unwrap();
# // Clean up
# std::fs::remove_dir_all("test").unwrap();
```

### Creating Config from File

```rust
# use fast_config::FastConfig;
# use fast_config::Format;
# use serde::Serialize;
# use serde::Deserialize;
# #[derive(Serialize, Deserialize, FastConfig)]
# pub struct MyData { pub value: i32 }
# // First, create and save a config file
# let mut temp = MyData { value: 42 };
# let config_path = "example_config.json";
# temp.save(config_path, Format::JSON).unwrap();
// Create config directly from a file path
let data = MyData::new(config_path, Format::JSON).unwrap();
# // Clean up
# std::fs::remove_file(config_path).unwrap();
```

### String Serialization

```rust
# use fast_config::FastConfig;
# use fast_config::Format;
# use serde::Serialize;
# use serde::Deserialize;
# #[derive(Serialize, Deserialize, FastConfig)]
# pub struct MyData { pub value: i32 }
# let data = MyData { value: 42 };

// Convert config to string
let json_string = data.to_string(Format::JSON).unwrap();
let pretty_json = data.to_string_pretty(Format::JSON).unwrap();

// Create config from string
let loaded = MyData::from_string(&json_string, Format::JSON).unwrap();
```

### Pretty Formatting

```rust
# use fast_config::FastConfig;
# use fast_config::Format;
# use serde::Serialize;
# use serde::Deserialize;
# #[derive(Serialize, Deserialize, FastConfig)]
# pub struct MyData { pub value: i32 }
# let data = MyData { value: 42 };

// Save with pretty formatting (indented, readable)
data.save_pretty("config.json", Format::JSON).unwrap();
# // Clean up
# std::fs::remove_file("config.json").unwrap();
```

## Getting started

1. Add the crate to your project:
   ```bash
   cargo add fast_config
   ```
   - Also add `serde` with derive features:
   ```bash
   cargo add serde --features derive
   ```

2. Enable the feature(s) for the format(s) you'd like to use in your `Cargo.toml`:
   ```toml
   [dependencies]
   fast_config = { version = "...", features = ["json", "json5", "toml", "yaml", "derive"] }
   ```
   - Available formats: `json`, `json5`, `toml`, `yaml`
   - Enable the `derive` feature to use the `#[derive(FastConfig)]` macro

3. Create a struct to hold your data and derive the necessary traits:
   ```rust
   use serde::Serialize;
   use serde::Deserialize;
   use fast_config::FastConfig;
   
   #[derive(Serialize, Deserialize, FastConfig)]
   pub struct MyConfig {
       pub setting: String,
   }
   ```

4. Use the trait methods directly on your struct:
   ```rust
   # use fast_config::FastConfig;
   # use fast_config::Format;
   # use serde::Serialize;
   # use serde::Deserialize;
   # #[derive(Serialize, Deserialize, FastConfig)]
   # pub struct MyConfig { pub setting: String }
   # // Clean up any existing file first
   # let _ = std::fs::remove_file("example_getting_started.json");
   let mut config = MyConfig { setting: "default".into() };
   let config_path = "example_getting_started.json";
   config.save(config_path, Format::JSON).unwrap();
   config.load(config_path, Format::JSON).unwrap();
   # // Clean up
   # std::fs::remove_file(config_path).unwrap();
   ```

---

## API Reference

### The `FastConfig` Trait

The `FastConfig` trait provides methods for loading, saving, and serializing config data. When you derive `FastConfig` on your struct, these methods become available:

#### File Operations

- **`load(path, format)`** - Loads config data from a file, replacing the current struct's values
- **`save(path, format)`** - Saves config data to a file (compact format)
- **`save_pretty(path, format)`** - Saves config data to a file with pretty formatting (indented, readable)

#### String Operations

- **`from_string(content, format)`** - Creates a new config instance from a string
- **`to_string(format)`** - Converts config to a compact string representation
- **`to_string_pretty(format)`** - Converts config to a pretty-formatted string

#### Constructor

- **`new(path, format)`** - Creates a new config instance by loading from a file path

### The `#[derive(FastConfig)]` Macro

The derive macro automatically implements the `FastConfig` trait for your struct. It requires that your struct also derives `Serialize` and `Deserialize` from `serde`.

#### Custom Crate Path

If you're re-exporting `fast_config` under a different name, you can specify the crate path:

```rust,ignore
use serde::Serialize;
use serde::Deserialize;
use fast_config::FastConfig;

#[derive(Serialize, Deserialize, FastConfig)]
#[fast_config(crate = "my_crate::fast_config")]
pub struct MyConfig {
    pub value: i32,
}
```

---

View the [tests](./fast_config/src/tests/) directory for more advanced examples.

## Migration Note

The crate now uses a trait-based approach with `#[derive(FastConfig)]`. This makes the API cleaner and more ergonomic - you can now call `save()` and `load()` directly on your config struct instead of wrapping it in a `Config` type.

If you're migrating from an older version, see the [conversion tutorial](./CONVERSION_TUTORIAL.md) for guidance.

---
<br/>
