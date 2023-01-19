//! A small and easy-to-use Rust crate to handle config files.
//! Currently supports JSON5, TOML, and YAML. But more formats are planned to be added later.

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
/// ```
/// use fast_config::{ConfigOptions, Config};
///
/// // .. MyData definition here
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
