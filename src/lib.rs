//! fast_config
//! =============
//! ---
//!
//! [<img alt="github" src="https://img.shields.io/badge/github-fast_config-lightgray.svg?logo=github&style=for-the-badge"/>](https://github.com/FlooferLand/fast_config)
//! [<img alt="crates.io" src="https://img.shields.io/crates/v/fast_config?logo=rust&style=for-the-badge"/>](https://crates.io/crates/fast_config)
//! [<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-fast_config-lightgray.svg?logo=rust&style=for-the-badge"/>](https://docs.rs/fast_config)
//! <br style="display: block; margin: 0 0; content: '---'" />
//! [<img alt="license" src="https://img.shields.io/github/license/FlooferLand/fast_config?style=flat"/>](https://github.com/FlooferLand/fast_config/blob/main/LICENSE)
//! [<img alt="code size" src="https://img.shields.io/github/languages/code-size/FlooferLand/fast_config?style=flat"/>](https://www.youtube.com/watch?v=dQw4w9WgXcQ)
//! [<img alt="issues" src="https://img.shields.io/github/issues/FlooferLand/fast_config?label=open%20issues&style=flat"/>](https://github.com/FlooferLand/fast_config/issues)
//!
//! A small, lightweight, and easy-to-use Rust crate to handle config files.
//!
//! Currently only supports: JSON5, TOML, and YAML.
//! But more formats are planned to be added later.
//!
//! *[- click here to view code examples](#examples)*
//! <br style="display: block; margin: 0 0; content: '---'" />
//! *[- click here to jump to the Getting Started section](#getting-started)*
//!
//! ---
//!
//! ## What is this crate?
//! This crate was originally called `simple_config`, but I had to change
//! the name due to a crate having the exact same name [*(linked here)*](https://crates.io/crates/simple_config).
//!
//! This crate was made to be a faster to set up, more light-weight, statically typed alternative to [config](https://crates.io/crates/config).
//! But it also accidentally manages to beat [simple_config](https://crates.io/crates/simple_config) in the lightweightness and "fast to set up" category.
//!
//! The lead benefit of this crate is how fast you can get it set up;
//! It also doesn't sacrifice performance for readability *(see some examples at [# Examples](#examples))*
//!
//! ---
//!
//! ### Why this crate?
//! - It's small and fast *(uses compile-time features to remove any unnecessary code)*
//! - It's safe and robust *(uses Rust's structs to store data, instead of HashMaps)*
//! - Ridiculously simple to use *(only takes 3 lines of short code to make a config file, write/read something, and close it)*
//!
//! ### Why not this crate?
//! - You can't currently use different file formats in the same project <br/>
//! This is an intentional design choice to be able to
//! shrink down the code as much as possible. <br/>
//! And since this crate is mostly made for the global-scope config file a program might use,
//! I didn't really see that as necessary. <br/>
//! *Feel free to open an issue if you'd like this to be changed!*
//!
//! ### ⚠ Documentation and tests are still being made! ⚠
//! I'm still working on this crate, and it's in somewhat-early-access.
//! While I haven't managed to find any bugs, documentation might be a little weird or incomplete at the current moment.
//!
//! ---
//!
//! ## Examples:
//! ```no_run
//! use serde::{Serialize, Deserialize};
//! use fast_config::Config;
//!
//! // Creating a config struct to store our data
//! #[derive(Serialize, Deserialize)]
//! pub struct MyData {
//!     #[serde(default = "MyDataDefaults::student_debt")]
//!     pub student_debt: i32
//! }
//!
//! // Storing the default values for our data
//! pub struct MyDataDefaults;
//! impl MyDataDefaults {
//!     pub fn student_debt() -> i32 { 20 }
//! }
//!
//! fn main() {
//!     // Initializing a logging system (needed to show errors)
//!     env_logger::init();
//!
//!     // Creating a new config struct with our data struct (it can also guess the file extension)
//!     let mut config = Config::<MyData>::new("./config/README");
//!
//!     // Read/writing to the data
//!     println!("I am ${} in debt", config.data.student_debt);
//!     config.data.student_debt = i32::MAX;
//!     println!("Oh no, i am now ${} in debt!!", config.data.student_debt);
//!
//!     // Saving it back to the disk
//!     config.save();
//! }
//! ```
//!
//! ## Getting started
//!
//! 1. Add the crate to your project <br/> `cargo add fast_config`
//! ---
//! 2. Enable a feature for the format you'd like to use <br/>
//!    - Currently only `json5`, `toml`, and `yaml` are supported <br/>
//!    - Please note that currently only *one* feature can be enabled at the same time
//! ---
//! 3. Create a struct for your data that derives `serde::Serialize` and `serde::Deserialize`
//! ---
//! 4. Use <br/>
//!    `let my_config = Config::<MyData>::new("..");` <br/>
//!    to create and store your config file(s)!
//!
//! ---
//!
//! View the [examples](./examples) directory for more advanced examples.

// TODO: ^ Use #![doc = include_str!("../README.md")] instead.
//         Currently it can't be used because it runs doc tests

mod format_dependant;
mod utils;

use std::fs;
use std::io::{Read, Write};
use log::error;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

// TODO: Fix this very ugly code
#[cfg(all(feature = "json5", feature = "toml"))]
compile_error!("JSON5 and TOML support is mutually exclusive and cannot be enabled together");
#[cfg(all(feature = "json5", feature = "yaml"))]
compile_error!("JSON5 and YAML support is mutually exclusive and cannot be enabled together");
#[cfg(all(feature = "toml", feature = "yaml"))]
compile_error!("TOML and YAML support is mutually exclusive and cannot be enabled together");

#[cfg(not(any(feature = "json5", feature = "toml", feature = "yaml")))]
compile_error!("You must select at least one format: `json5`, `toml`, or `yaml`");

// Bug testing
#[cfg(test)]
mod tests;

/// Used to configure the [`Config`] object
///
/// # Attributes
/// - `pretty` - Makes the contents of the config file more readable.
/// When false, it will try to compact down the config file data so it takes up less storage space.
/// *I recommend you keep it on* as most modern systems have enough space to handle spaces and newline characters, even at scale.
///
/// # More options are to be added later!
/// Pass `..` [`Default::default()`] at the end of your construction
/// to prevent yourself from getting errors in the future!
///
/// # Examples:
/// ```no_run
/// use fast_config::{ConfigOptions, Config};
/// use serde::{Serialize, Deserialize};
///
/// // Creating a config struct to store our data
/// #[derive(Serialize, Deserialize)]
/// pub struct MyData {
///     #[serde(default = "MyDataDefaults::some_data")]
///     pub some_data: i32
/// }
///
/// // Storing the default values for our data
/// pub struct MyDataDefaults;
/// impl MyDataDefaults {
///     pub fn some_data() -> i32 { 123 }
/// }
///
/// fn main() {
///     let options = ConfigOptions {
///         pretty: false,
///         .. Default::default()
///     };
///
///     let mut config = Config::<MyData>::from_options("./config/myconfig.json5", options);
///     // [.. do stuff here]
/// }
/// ```
///
pub struct ConfigOptions {
    pub pretty: bool
}
impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            pretty: true
        }
    }
}

/// The main class you use to create/access configuration data!
///
/// # Construction
/// See [`Config::new`] and [`Config::from_options`] if you wish to construct a new Config!
///
/// # Data
/// This class stores data using a struct you define yourself.
/// This allows for the most amount of performance and safety in most scenarios,
/// while also allowing you to add additional features by adding `impl` blocks on your struct.
///
/// [`Serialize`]: serde::Serialize
/// [`Deserialize`]: serde::Deserialize
///
/// Your struct needs to implement [`serde::Serialize`] and [`serde::Deserialize`].
/// In most cases you can just use `#[derive(Serialize, Deserialize)]` to derive them.
///
///
/// You can also use `#[serde(default = "..")]` to define the default values for your data.
/// This does however require you pass in a callable *(such as a function)*, seemingly due to a Serde macro limitation.
///
/// Here is a code example on how you could define the data to pass into the constructors on this class:
/// ```
/// use serde::{Serialize, Deserialize};
///
/// // Creating a config struct to store our data
/// #[derive(Serialize, Deserialize)]
/// struct MyData {
///     #[serde(default = "MyDataDefaults::student_debt")]
///     pub student_debt: i32,
/// }
///
/// // Storing the default values for our data
/// pub struct MyDataDefaults;
/// impl MyDataDefaults {
///     pub fn student_debt() -> i32 { 20 }
/// }
/// ```
/// Implementing [`Serialize`] and [`Deserialize`] yourself is quite complicated but will provide the most flexibility.
///
/// *If you wish to implement them yourself I'd recommend reading the Serde docs on it*
///
pub struct Config<D> where for<'a> D: Deserialize<'a> + Serialize {
    pub data: D,
    path: PathBuf,
    pub options: ConfigOptions
}
impl<D> Config<D> where for<'a> D: Deserialize<'a> + Serialize {
    /// Constructs and returns a new config object using the default options.
    ///
    /// If there's not a file at `path`, the file will automatically be generated.
    ///
    /// - `path`: Takes in a path to where the config file is or should be located.
    /// If the file has no extension, the extension will be guessed using the enabled feature
    ///
    /// If you'd like to configure this object, you should take a look at using [`Config::from_options`] instead.
    pub fn new(path: impl AsRef<Path>) -> Self {
        Self::construct(path, ConfigOptions::default())
    }

    /// Constructs and returns a new config object from a set of custom options.
    ///
    /// If there's not a file at `path`, the file will automatically be generated.
    ///
    /// - `path`: Takes in a path to where the config file is or should be located.
    /// If the file has no extension, the extension will be guessed using the enabled feature
    ///
    /// - `options`: Takes in a [`ConfigOptions`], used to configure the styling of the data among other things.
    pub fn from_options(path: impl AsRef<Path>, options: ConfigOptions) -> Self {
        Self::construct(path, options)
    }

    // Main, private constructor
    fn construct(path: impl AsRef<Path>, options: ConfigOptions) -> Self {
        let mut path = PathBuf::from(path.as_ref());

        // Adding an extension if no extension was found
        if path.extension().is_none() {
            path.set_extension(format_dependant::get_extension());
        }

        // Making sure there's a config file
        let mut file_content = String::new();
        if !path.exists() {
            // Creating the directories leading up to the config file
            match path.parent() {
                Some(dirs) => {
                    fs::create_dir_all(dirs)
                        .expect("Could not create the directories leading up to the config file!")
                },
                None => {}
            }

            // Creating the config file itself
            let mut file = fs::File::create(&path).expect("Could not create the config file!");
            file_content = format_dependant::get_default_data();
            write!(file, "{}", file_content).expect("Could not initialize the config file!");
        } else {
            // Reading from the file if a file was found
            let mut file = fs::File::open(&path).expect("Could not open the config file!");
            file.read_to_string(&mut file_content).expect("File content isn't valid UTF-8!");
        }

        // Creating the config data
        let data = format_dependant::from_string(file_content.as_str())
            .expect(
                format!(
                    "Config file isn't valid according to it's format! ({})",
                    format_dependant::get_extension()
                ).as_str()
            );

        // Creating the Config object
        Self { data, path, options }
    }

    /// Saves the config file to the disk.
    ///
    /// It uses the [`Config`]'s object own internal `path` property to get the path required to save the file
    /// so there is no need to pass in the path to save it at.
    ///
    /// If you wish to specify the path to save it at
    /// you can change the path yourself by setting the Config's `path` property.
    /// <br/> <br/>
    /// ## save_at method
    /// There used to be a built-in function called `save_at` while i was developing the crate,
    /// but I ended up removing it due to the fact i didn't see anyone actually using it,
    /// and it might've ended up in some users getting confused, as well as a tiny bit of performance overhead.
    ///
    /// If you'd like this feature to be back feel free to open an issue and I'll add it back right away!
    pub fn save(&self) {
        let to_string = format_dependant::to_string(&self.data, &self.options);
        match to_string {
            Ok(data) => {
                let mut file = fs::File::create(&self.path).expect("Could not open the config file! (saving)");
                write!(file, "{data}").expect("Could not save the config file!");
            },
            Err(e) => {
                error!("{e}\n\t^ This error sometime seems to mean a data type you're using isn't supported!")
            }
        };
    }
}
