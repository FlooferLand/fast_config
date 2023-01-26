#![doc = include_str!("../README.md")]

mod error;
mod error_messages;
mod extensions;
mod format_dependant;
mod utils;

use std::fmt::{Display, Formatter};
use std::fs;
use std::io::{Read, Write};
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

// CHECKED?: Make all file formats usable in the same project by making a FileFormat enum
//       and disabling the struct entries based on the enabled features
// ^------- Needs to be further tested!

// TODO: Finish rewriting the documentation for methods / structs
// TODO: Add panic notifiers in the documentation

// TODO: Add in an option to automatically save the config when the Config object is dropped

#[cfg(not(any(feature = "json5", feature = "toml", feature = "yaml")))]
compile_error!("You must install at least one format feature: `json5`, `toml`, or `yaml`");

// Bug testing
#[cfg(test)]
mod tests;


// Separated things
pub use error::*;
pub use error_messages::*;


/// The object you use to configure the file format
/// 
/// You use it in [`ConfigOptions`]
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum ConfigFormat {
    JSON5,
    TOML,
    YAML,
    None
}
impl ConfigFormat {
    fn from_extension(ext: &str) -> Self {
        if ext.len() <= 2 {
            return ConfigFormat::None;
        }

        // Getting rid of the first . if it starts with one
        let ext = {
            if let Some(strip) = ext.strip_prefix('.') {
                strip
            } else {
                ext
            }
        };
        
        // Matching
        match ext.to_lowercase().as_str() {
            "json" | "json5" => ConfigFormat::JSON5,
            "toml"           => ConfigFormat::TOML,
            "yaml" | "yml"   => ConfigFormat::YAML,
            _ => ConfigFormat::None
        }
    }
}
impl Display for ConfigFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let string = match self {
            ConfigFormat::None => {
                log::error!("Format \"None\" cannot be displayed using Display!");
                String::new()
            }
            _ => {
                format!("{self:?}").to_lowercase()
            }
        };
        write!(f, "{string}")
    }
}


/// Used to configure the [`Config`] object
///
/// # Attributes
/// - `pretty` - Makes the contents of the config file more readable.
/// When false, it will try to compact down the config file data so it takes up less storage space.
/// *I recommend you keep it on* as most modern systems have enough space to handle
/// spaces and newline characters, even at scale.
///
/// - `format` - An enum to specify the format language to use *(ex: JSON, TOML, etc.)* <br/>
/// Takes in an enum of type [`ConfigFormat`]
/// It's [`ConfigFormat::None`] by default, but it will also try to guess the format based on
/// the file format and/or enabled features.
///
/// # More options are to be added later!
/// Pass `..` [`Default::default()`] at the end of your construction
/// to prevent yourself from getting errors in the future!
///
/// # Examples:
/// ```no_run
/// use fast_config::{ConfigOptions, ConfigFormat, Config};
/// use serde::{Serialize, Deserialize};
///
/// // Creating a config struct to store our data
/// #[derive(Serialize, Deserialize)]
/// pub struct MyData {
///     pub some_data: i32
/// }
///
/// fn main() {
///     // Creating the options
///     let options = ConfigOptions {
///         pretty: false,
///         format: ConfigFormat::JSON5,
///         .. Default::default()
///     };
///
///     // Creating the data and setting it's default values
///     let data = MyData {
///         some_data: 12345
///     };
///
///     // Creating the config itself
///     let mut config = Config::<MyData>::from_options("./config/myconfig", options, data);
///     // [.. do stuff here]
/// }
/// ```
///
pub struct ConfigOptions {
    pub pretty: bool,
    pub format: ConfigFormat
}
impl Default for ConfigOptions {
    fn default() -> Self {
        Self {
            pretty: true,
            format: ConfigFormat::None
        }
    }
}


/// The main class you use to create/access your configuration files!
///
/// # Construction
/// See [`Config::new`] and [`Config::from_options`] if you wish to construct a new Config!
///
/// # Data
/// This class stores data using a struct you define yourself.
/// This allows for the most amount of performance and safety,
/// while also allowing you to add additional features by adding `impl` blocks on your struct.
///
/// [`Serialize`]: serde::Serialize
/// [`Deserialize`]: serde::Deserialize
///
/// Your struct needs to implement [`serde::Serialize`] and [`serde::Deserialize`].
/// In most cases you can just use `#[derive(Serialize, Deserialize)]` to derive them.
///
///
/// Here is a code example on how you could define the data to pass into the constructors on this class:
/// ```
/// use serde::{Serialize, Deserialize};
///
/// // Creating a config struct to store our data
/// #[derive(Serialize, Deserialize)]
/// struct MyData {
///     pub student_debt: i32,
/// }
///
/// fn main() {
///     // Making our data and setting its default values
///     let data = MyData {
///         student_debt: 20
///     };
///     // ..
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
    /// - `data`: Takes in a struct that inherits [`serde::Serialize`] and [`serde::Deserialize`]
    /// You have to make this struct yourself, construct it, and pass it in.
    /// More info is provided at [`Config`].
    ///
    /// If you'd like to configure this object, you should take a look at using [`Config::from_options`] instead.
    pub fn new(path: impl AsRef<Path>, data: D) -> Result<Config<D>, ConfigError> {
        Self::construct(path, ConfigOptions::default(), data)
    }

    /// Constructs and returns a new config object from a set of custom options.
    ///
    /// If there's not a file at `path`, the file will automatically be generated.
    ///
    /// - `path`: Takes in a path to where the config file is or should be located.
    /// If the file has no extension, the extension will be guessed based on the enabled feature
    /// (or the selected format in your `options`)
    ///
    /// - `options`: Takes in a [`ConfigOptions`],
    /// used to configure the format language, styling of the data, and other things.
    ///
    /// - `data`: Takes in a struct that inherits [`serde::Serialize`] and [`serde::Deserialize`]
    /// You have to make this struct yourself, construct it, and pass it in.
    /// More info is provided at [`Config`].
    pub fn from_options(path: impl AsRef<Path>, options: ConfigOptions, data: D) -> Result<Config<D>, ConfigError> {
        Self::construct(path, options, data)
    }

    // Main, private constructor
    fn construct(path: impl AsRef<Path>, mut options: ConfigOptions, mut data: D) -> Result<Config<D>, ConfigError> {
        let mut path = PathBuf::from(path.as_ref());

        // Guessing the file format
        if options.format == ConfigFormat::None && path.extension().is_some() {
            // - Based on the extension
            let ext = path.extension().unwrap();
            let Some(ext) = ext.to_str() else {
                return Err(ConfigError::InvalidEncoding(Box::new(ext.to_owned())));
            };
            options.format = ConfigFormat::from_extension(ext);
        } else {
            // - Based on the enabled features
            options.format = format_dependant::get_first_enabled_feature();
        }

        // Setting the file format
        if path.extension().is_none() {
            path.set_extension(options.format.to_string());
        }

        // Making sure there's a config file
        if let Ok(mut file) = fs::File::open(&path) {
            // Reading from the file if a file was found
            let mut content = String::new();
            if file.read_to_string(&mut content).is_err() {
                return Err(ConfigError::InvalidFileEncoding(path));
            };

            // Deserialization
            if let Ok(value) = format_dependant::from_string(&content, &options.format) {
                data = value;
            } else {
                return Err(ConfigError::DataParseError(
                    DataParseError::Deserialize(options.format, options.format.to_string())
                ));
            };
        } else {
            // Creating the directories leading up to the config file
            match path.parent() {
                Some(dirs) => {
                    if fs::create_dir_all(dirs).is_err() {
                        return Err(ConfigError::IoError(IoError::ParentDirectoryCreation));
                    }
                },
                None => {}
            }

            // Creating the config file itself
            // (should never fail due to the code above)
            let _ = fs::File::create(&path);
        }

        // Creating the Config object
        Ok(Self {
            data,
            path,
            options
        })
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
    pub fn save(&self) -> Result<(), ConfigSaveError> {
        let to_string = format_dependant::to_string(&self.data, &self.options);
        match to_string {
            Ok(data) => {
                match fs::File::create(&self.path) {
                    Ok(mut file) => {
                        // Writing data to the writer
                        if let Err(err) = write!(file, "{data}") {
                            Err(ConfigSaveError::IoError(err))?
                        }
                    },
                    Err(_) => {
                        // If the file could not be saved, try fixing it
                        // by creating any missing parent directories
                        if let Some(parent_dir) = self.path.parent() {
                            let _ = fs::create_dir_all(parent_dir);
                        }

                        // Create the file
                        if let Err(err) = fs::File::create(&self.path) {
                            Err(ConfigSaveError::IoError(err))?
                        }
                    }
                };
            },
            Err(e) => {
                Err(ConfigSaveError::SerializationError(e))?
                // error!("{e}\n\t^ This error sometimes seems to mean a data type you're using in your custom data struct isn't supported!");
            }
        };
        Ok(())
    }
}
