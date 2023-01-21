#![doc = include_str!("../README.md")]

mod extensions;
mod format_dependant;
mod utils;

use std::fs;
use std::io::{Read, Write};
use log::error;
use serde::{Serialize, Deserialize};
use std::path::{Path, PathBuf};

// CHECKED?: Make all file formats usable in the same project by making a FileFormat enum
//       and disabling the struct entries based on the enabled features

// TODO: Finish rewriting the documentation for methods / structs

// TODO: Add in an option to automatically save the config when the Config object is dropped

// TODO: Add in a way to check which feature should be enabled based on the file extension
//       .. If there is no file extension, guess the user wants to use the only enabled feature
//          so users could only have one feature enabled (like JSON5) and it'll guess automatically

#[cfg(not(any(feature = "json5", feature = "toml", feature = "yaml")))]
compile_error!("You must select at least one format: `json5`, `toml`, or `yaml`");

// Bug testing
#[cfg(test)]
mod tests;

#[cfg(test)]
use strum_macros::EnumIter;

#[cfg_attr(test, derive(EnumIter))]
pub enum ConfigFormat {
    JSON5,
    TOML,
    YAML,
    None
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
/// It's [`ConfigFormat::None`] by default, but it will try to guess the format based on
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
///     let options = ConfigOptions {
///         pretty: false,
///         format: ConfigFormat::JSON5,
///         .. Default::default()
///     };
///
///     let data = MyData {
///         some_data: 12345
///     };
///
///     let mut config = Config::<MyData>::from_options("./config/myconfig.json5", options, data);
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
    pub fn new(path: impl AsRef<Path>, data: D) -> Self {
        Self::construct(path, ConfigOptions::default(), data)
    }

    /// Constructs and returns a new config object from a set of custom options.
    ///
    /// If there's not a file at `path`, the file will automatically be generated.
    ///
    /// - `path`: Takes in a path to where the config file is or should be located.
    /// If the file has no extension, the extension will be guessed using the enabled feature
    ///
    /// - `options`: Takes in a [`ConfigOptions`], used to configure the styling of the data among other things.
    ///
    /// - `data`: Takes in a struct that inherits [`serde::Serialize`] and [`serde::Deserialize`]
    /// You have to make this struct yourself, construct it, and pass it in.
    /// More info is provided at [`Config`].
    pub fn from_options(path: impl AsRef<Path>, options: ConfigOptions, data: D) -> Self {
        Self::construct(path, options, data)
    }

    // Main, private constructor
    fn construct(path: impl AsRef<Path>, mut options: ConfigOptions, mut data: D) -> Self {
        let mut path = PathBuf::from(path.as_ref());

        // Adding an extension if no extension was found
        if path.extension().is_none() {
            path.set_extension(format_dependant::get_extension(&options.format));
        }

        // Guessing the format based on the extension (if there is any)
        if let Some(ext) = path.extension() {
            let ext = ext.to_str().unwrap_or("").to_lowercase();
            options.format = match ext.as_str() {
                "json" | "json5" => ConfigFormat::JSON5,
                "toml" => ConfigFormat::TOML,
                "yaml" => ConfigFormat::YAML,
                _ => ConfigFormat::None
            }
        }

        // Making sure there's a config file
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
            fs::File::create(&path).expect("Could not create the config file!");
        } else {
            // Reading from the file if a file was found
            let mut content = String::new();
            let mut file = fs::File::open(&path).expect("Could not open the config file!");
            file.read_to_string(&mut content).expect("File content isn't valid UTF-8!");
            data = format_dependant::from_string(&content, &options.format).expect(
                format!(
                    "Config file isn't valid according to it's format! ({})",
                    format_dependant::get_extension(&options.format)
                ).as_str()
            );
        }

        // Creating the Config object
        Self {
            data,
            path,
            options
        }
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
                let mut file = match fs::File::create(&self.path) {
                    Ok(file) => file,
                    Err(_) => {
                        // If the file could not be saved
                        if let Some(parent_dir) = self.path.parent() {
                            let _ = fs::create_dir_all(parent_dir);
                        }
                        fs::File::create(&self.path)
                            .expect("Could not open the config file while saving!\n- Does the path to the file still exist?")
                    }
                };
                write!(file, "{data}").expect("Could not save the config file!");
            },
            Err(e) => {
                error!("{e}\n\t^ This error sometimes seems to mean a data type you're using in your custom data struct isn't supported!")
            }
        };
    }
}
