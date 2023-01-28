use std::path::PathBuf;
use thiserror::Error;
use crate::ConfigFormat;

// - Some of the display/error traits are implemented with
//   `thiserror` to save time and source code readability
// - Other are however implemented manually
//   inside `error_messages.rs`

/// Represents an error related to serialization/deserialization of your data
#[derive(Debug)]
pub enum DataParseError {
	/// Serialization: From an object, to a string (stringification)
	/// - Stores the format that failed
	Serialize(ConfigFormat),

	/// Deserialization: From a string, to an object (objectification)
	/// - Stores the format that failed, as well as the data in string form
	Deserialize(ConfigFormat, String)
}

/// Represents an error related to the file format not being able to be found or guessed
#[derive(Debug)]
pub struct UnknownFormatError {
	/// The error message itself
	pub message: Option<String>,

	/// The [`ConfigFormat`]s found in your environment.
	pub found_formats: Vec<ConfigFormat>
}
impl UnknownFormatError {
	pub fn new(message: Option<String>, found_formats: Vec<ConfigFormat>) -> Self {
		Self { message, found_formats }
	}
}

/// The main result error type of the crate. <br/>
/// Each type has it's own documentation.
#[derive(Error, Debug)]
pub enum ConfigError {
	/// Occurs when a file isn't composed of valid UTF-8 characters.
	/// - Stores the path to the erroring file
	#[error("InvalidFileEncoding: Failed to read file data of \"{:?}\" into a valid UTF-8 string.", .0)]
	InvalidFileEncoding(std::io::Error, PathBuf),

	/// Occurs when the file could not be saved due to filesystem-related errors. <br/>
	/// Usually when one of the parent directories for the config file could not
	/// be located or automatically created.
	/// - Stores the [`std::io::Error`] in question
	#[error(transparent)]
	IoError(std::io::Error),

	/// Occurs when Serde fails to serialize/deserialize your data
	/// - Stores an enum containing the section of data parsing failed for
	///   (serialization/deserialization) <br/>
	#[error(transparent)]
	DataParseError(DataParseError),

	/// Occurs when `fast_config` cannot guess what format your data should be.
	/// - Stores some data related to the error
	///
	/// # This error can be avoided by either:
	/// 1. Adding a file extension at the end of the path name
	///    (`json`/`json5`, `toml`, `yaml`/`yml`) <br/>
	///    *(It would be appreciated if you create an issue on the project's Github if
	///      you notice an extension type is missing)*
	/// 2. Passing a `ConfigSetupOptions` struct into `Config::from_options`, and defining
	///    the format there.
	/// 3. Or only having one enabled `format` feature in your `cargo.toml`
	#[error(transparent)]
	UnknownFormat(UnknownFormatError)
}

#[derive(Error, Debug)]
pub enum ConfigSaveError {
	/// Occurs when the file could not be saved due to filesystem-related errors.
	/// - Stores the [`std::io::Error`] in question
	#[error(transparent)]
	IoError(std::io::Error),

	/// Occurs when the save data could not be serialized. <br/>
	/// - Stores an error in string form explaining why serialization failed.
	#[error("{}", .0)]
	SerializationError(String)
}

