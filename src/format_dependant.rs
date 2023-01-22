use serde::{Deserialize, Serialize};
use crate::{ConfigFormat, ConfigOptions};
use crate::extensions::{GenericResult, ResultGeneralize};

// Getting the enabled features via code
pub fn get_enabled_features() -> Vec<ConfigFormat> {
    let mut vector = Vec::new();
    #[cfg(feature = "json5")] vector.push(ConfigFormat::JSON5);
    #[cfg(feature = "toml")]  vector.push(ConfigFormat::TOML);
    #[cfg(feature = "yaml")]  vector.push(ConfigFormat::YAML);
    vector
}

// Creates a new string from an existing data object
pub fn to_string<D>(value: &D, options: &ConfigOptions) -> GenericResult<String> where D: Serialize {
    match options.format {
        #[cfg(feature = "json5")]
        ConfigFormat::JSON5 => {
            match options.pretty {
                true => serde_json::to_string_pretty(value).generalize(),
                false => json5::to_string(value).generalize()
            }
        },

        #[cfg(feature = "toml")]
        ConfigFormat::TOML => {
            match options.pretty {
                true => toml::to_string_pretty(value).generalize(),
                false => toml::to_string(value).generalize()
            }
        },

        #[cfg(feature = "yaml")]
        ConfigFormat::YAML => {
            match options.pretty {
                true  => serde_yaml::to_string(value).generalize(),
                false => {
                    let string = serde_yaml::to_string(value);
                    if string.is_err() {
                        return Err(string.err().unwrap().to_string());
                    }
                    Ok(crate::utils::compress_string(string.unwrap()))
                }
            }
        },

        _ => Err("No format selected (to_string)").generalize()
    }
}


// Creates a new data object from a string
pub fn from_string<'a, D>(value: &'a String, format: &ConfigFormat) -> GenericResult<D> where D: Deserialize<'a> {
    let value = value.as_str();
    match format {
        #[cfg(feature = "json5")]
        ConfigFormat::JSON5 =>
            json5::from_str::<D>(value).generalize(),

        #[cfg(feature = "toml")]
        ConfigFormat::TOML =>
            toml::from_str::<D>(value).generalize(),

        #[cfg(feature = "yaml")]
        ConfigFormat::YAML =>
            serde_yaml::from_str::<D>(value).generalize(),

        _ =>
            Err("No format selected (from_string)").generalize()
    }
}

