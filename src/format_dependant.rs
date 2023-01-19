use std::ffi::OsStr;
use std::path::Path;
use serde::{Deserialize, Serialize};
use serde::de::Error;
use crate::{ConfigOptions, utils};



pub fn get_extension<'a>() -> &'a str {
    #[cfg(feature = "json5")] { ".json5" }
    #[cfg(feature = "toml")]  { ".toml" }
    #[cfg(feature = "yaml")]  { ".yaml" }
}



pub fn get_default_data() -> String {
    String::from({
        #[cfg(feature = "json5")] { "{}" }
        #[cfg(feature = "toml")]  { "" }
        #[cfg(feature = "yaml")]  { "" }
    })
}



// Creates a new string from an existing data object
#[cfg(feature = "json5")]
pub fn to_string<D>(value: &D, options: &ConfigOptions) -> Result<String, serde_json::Error> where D: Serialize {
    match options.pretty {
        true => serde_json::to_string_pretty(value),
        false => {
            // Changing the error type to return the same Result type
            let res = json5::to_string(value);
            match res {
                Ok(string) => Ok(string),
                Err(err) => Err(serde_json::Error::custom(err.to_string()))
            }
        }
    }
}
#[cfg(feature = "toml")]
pub fn to_string<D>(value: &D, options: &ConfigOptions) -> Result<String, toml::ser::Error> where D: Serialize {
    match options.pretty {
        true  => toml::to_string_pretty(value),
        false => {
            let string = toml::to_string(value);
            if string.is_err() {
                return Err(string.err().unwrap());
            }
            Ok(utils::compress_string(string.unwrap()))
        },
    }
}
#[cfg(feature = "yaml")]
pub fn to_string<D>(value: &D, options: &ConfigOptions) -> Result<String, serde_yaml::Error> where D: Serialize {
    let string = serde_yaml::to_string(value);
    if string.is_err() {
        return Err(string.err().unwrap());
    }
    let mut string = string.unwrap();
    match options.pretty {
        true  => Ok(string),
        false => Ok(utils::compress_string(string))
    }
}



// Creates a new data object from a string
#[cfg(feature = "json5")]
pub fn from_string<'a, D>(value: &'a str) -> json5::Result<D> where D: Deserialize<'a> {
    json5::from_str::<D>(value)
}
#[cfg(feature = "toml")]
pub fn from_string<'a, D>(value: &'a str) -> Result<D, toml::de::Error> where D: Deserialize<'a> {
    toml::from_str::<D>(value)
}
#[cfg(feature = "yaml")]
pub fn from_string<'a, D>(value: &'a str) -> Result<D, serde_yaml::Error> where D: Deserialize<'a> {
    serde_yaml::from_str::<D>(value)
}

