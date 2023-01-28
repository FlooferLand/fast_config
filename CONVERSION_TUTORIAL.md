## A tutorial on how to convert your data to the newest version!
This file targets helping you switch to the newest version, from the last version.

___Note:___ You're able to look at the commit history of this file to see past versions of this file, for older versions of the crate.

This tutorial currently targets conversion:
- from **0.1.1** to **1.0.0**.

# Changes
___Note:___ Always check the [GitHub version](https://github.com/FlooferLand/fast_config/blob/main/CONVERSION_TUTORIAL.md) of this file,
as it's the most up to date.
---

**1.0.0** has a LOT of changes, making this crate tons safer, stronger,
and more robust. <br/>
A *lot* of syntax has changed for the better, however.
No longer requiring the use of `#[serde(default = "..")]` and unnecessary functions.

_(a proper change log should be written here soon)_

# Steps
### 1. This bulky, ugly chunk of code is no longer needed
```rust,ignore
// Creating the data
#[derive(Serialize, Deserialize)]
struct ExampleData {
    #[serde(default = "ExampleDataDefaults::number")]
    number: i32
}

// Storing the default values for our data
pub struct ExampleDataDefaults;
impl ExampleDataDefaults {
    pub fn number() -> i32 { 0 }
}
```
<br/>
It has been replaced with something much simpler!

```rust,ignore
    // Creating the data
    #[derive(Serialize, Deserialize)]
    struct ExampleData {
        number: i32
    }
    
    // Our main function!
    fn main() {
        let example_data = ExampleData {
            number: 0
        };

        // .. initialize the config here!
    }
```
Yes! Now data can be created dynamically.
It also solves some safety issues the previous method had, regarding runtime errors.

### 2. Everything is safe now!
In the previous, early version.
Everything was an unsafe MESS.
No `Result`s got returned from anywhere.

___Now, however!___

Creating a new `Config` went from looking like this:
```rust,ignore
let config = Config::<ExampleData>::new("./config/myconfig");
```
to looking like this
```rust,ignore
let config = Config::new("./config/myconfig", example_data).unwrap();
```
Very epic!
<img width="30" src="https://media.istockphoto.com/photos/illustration-of-a-oool-yellow-smiley-with-sunglasses-picture-id513921039?s=612x612" alt="[goofy emoji]">

There is an entire `ConfigError` enum you can match on for errors.

Modifying and saving configs works the same way it did before in terms of syntax. <br/>
___The only difference being that the `save` method returns a `Result` now.___

### 3. Some renames / changes

---
- `format` inside `ConfigOptions` requires to be wrapped inside `Some()`
  as it is now an `Option<ConfigFormat>`. Selecting `None` will make it automatically guess the format
  *(guessing formats might fail in some cases, there's a `Result` error for it)*.
---
- `ConfigOptions` has been renamed to `ConfigSetupOptions`,
  as it is internally converted to an `InteralOptions` struct due to safety reasons
---

## That's it!
These are most of the big changes, however, there are also a lot of smaller ones
like the fact a LOT more of the API has been exposed. <br/>
For example, you can now access and set the `path` of your config any time you'd want after creating it.

You can view the [examples](./examples) directory for examples regarding the new syntax.
