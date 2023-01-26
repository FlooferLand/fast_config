use std::ffi::OsString;
use std::path::PathBuf;
use thiserror::Error;
use crate::ConfigFormat;

// - Some of the display/error traits are implemented with
//   `thiserror` to save time and source code readability
// - Other are however implemented manually
//   inside `error_messages.rs`

/// Represents a config-related IO error
#[derive(Error, Debug)]
pub enum IoError {
	#[error("ParentDirectoryCreation: Could not create the directories leading up to the config file!")]
	ParentDirectoryCreation,

	#[error("FileReadError: Could not read the config file! (possible missing file permissions)")]
	FileReadError,

	#[error("FileWriteError: Could not write to the config file! (possible missing file permissions)")]
	FileWriteError
}

/// Represents an error related to serialization/deserialization of your data
#[derive(Debug)]
pub enum DataParseError {
	/// Serialization: From an object, to a string (stringification)
	/// - Stores the format that failed
	Serialize(ConfigFormat),

	/// Deserialization: From a string, to an object (objectification)
	/// - Stores the format that failed, as well as the string that failed conversion
	Deserialize(ConfigFormat, String)
}


/// The main result error type of the crate
/// 
/// [`ConfigError::Fatal`] - Returned when your program has little chances of being recovered (ex: I/O errors) 
#[derive(Error, Debug)]
pub enum ConfigError {
	/// Occurs when conversion from an OsStr to a string fails
	/// - Stores the error string in question
	#[error("InvalidEncoding: Failed to convert OsStr \"{:?}\" into a valid UTF-8 string.", .0)]
    InvalidEncoding(Box<OsString>),

	/// Occurs when a file doesn't have UTF-8 compatible characters in it.
	/// - Stores the path to the erroring file
	#[error("InvalidFileEncoding: Failed to read file data of \"{:?}\" into a valid UTF-8 string.", .0)]
    InvalidFileEncoding(PathBuf),

	/// Occurs when one of the parent directories for the config file
	/// could not be created.
	/// - Stores the error in question
	#[error(transparent)]
	IoError(IoError),

	/// Occurs when Serde fails to serialize/deserialize your data
	#[error(transparent)]
	DataParseError(DataParseError)
}

#[derive(Error, Debug)]
pub enum ConfigSaveError {
	#[error(transparent)]
	IoError(std::io::Error),

	#[error("{}", .0)]
	SerializationError(String)
}

