// - This module serves as a way to print out useful error messages
//   for both the end user, and the developer.

// #[cfg(debug_assertions)]      - are developer-shown errors
// #[cfg(not(debug_assertions))] - are user-shown errors


use crate::DataParseError;

// - Data Parse Error
//   default error handler
impl std::error::Error for DataParseError {}
impl std::fmt::Display for DataParseError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			// Object to string
			DataParseError::Serialize(_format) => {
				let tip = {
					#[cfg(debug_assertions)] {
						"Your config's data types must all implement serde::Serialize and Deserialize!"
					}
					#[cfg(not(debug_assertions))] {
						"This is likely to be an issue caused by the Serialize implementation of the program you are using."
					}
				};
				write!(f, "Serialization: An error occurred trying to convert the config to a string.\n
					       [tip]: {tip}")
			},
			// String to object
			DataParseError::Deserialize(format, _string) => {
				let tip = {
					#[cfg(debug_assertions)] {
						"Make sure your data structs types/names match up with the config file you're trying to read.\n
						 Alternatively, make sure all of your types implement serde::Deserialize and Serialize!\n
						 -- You might want to:\n
						 1. Check that the format feature you're trying to use is enabled in your `cargo.toml` (JSON, TOML, YAML, etc)\n
						 2. Check that your data is valid (some types like vectors and custom types cannot be converted to Serde by default, you might want to implement Deserialize and Serialize for them manually)\n
						 3. Report this bug to the project's \"Issues\" page if nothing seems to be solving the issue (https://github.com/FlooferLand/fast_config/issues)"
					}
					#[cfg(not(debug_assertions))] {
						"If you edited a config file, make sure you were following the configuration format's syntax rules!"
					}
				};
				write!(f, "Deserialization: An error occurred trying to convert a string into a config object.\n
						   [err] Config file isn't valid according to it's format ({format})\n
						   [tip]: {tip}")
			},
		}
	}
}

