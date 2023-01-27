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
	pub found_formats: Vec<ConfigFormat>
}
impl UnknownFormatError {
	pub fn new(found_formats: Vec<ConfigFormat>) -> Self {
		Self { found_formats }
	}
}

/// The main result error type of the crate
/// 
/// [`ConfigError::Fatal`] - Returned when your program has little chances of being recovered (ex: I/O errors) 
#[derive(Error, Debug)]
pub enum ConfigError {
	/// Occurs when a file isn't composed of valid UTF-8 characters.
	/// - Stores the path to the erroring file
	#[error("InvalidFileEncoding: Failed to read file data of \"{:?}\" into a valid UTF-8 string.", .0)]
    InvalidFileEncoding(std::io::Error, PathBuf),

	/// Occurs when the file could not be saved due to filesystem-related errors. <br/>
	/// Usually when one of the parent directories for the config file could not be created/located.
	/// - Stores the [`std::io::Error`] in question
	#[error(transparent)]
	IoError(std::io::Error),

	/// Occurs when Serde fails to serialize/deserialize your data
	#[error(transparent)]
	DataParseError(DataParseError),

	#[error(transparent)]
	UnknownFormat(UnknownFormatError)
}

#[derive(Error, Debug)]
pub enum ConfigSaveError {
	/// Occurs when the file could not be saved due to filesystem-related errors.
	/// - Stores the [`std::io::Error`] in question
	#[error(transparent)]
	IoError(std::io::Error),

	#[error("{}", .0)]
	SerializationError(String)
}

