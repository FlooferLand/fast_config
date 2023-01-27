use serde::{Deserialize, Serialize};
use crate::{ConfigFormat, InternalOptions};
use crate::extensions::{GenericResult, ResultGeneralize};

// Getting the enabled features via code
pub fn get_enabled_features() -> Vec<ConfigFormat> {
    let mut vector = Vec::new();
    #[cfg(feature = "json5")] vector.push(ConfigFormat::JSON5);
    #[cfg(feature = "toml")]  vector.push(ConfigFormat::TOML);
    #[cfg(feature = "yaml")]  vector.push(ConfigFormat::YAML);
    vector
}

// Getting a singular enabled feature
pub fn get_first_enabled_feature() -> ConfigFormat {
    let features = get_enabled_features();
    if let Some(first) = features.first() {
        // If there is one feature
        *first
    } else if features.len() == 0 {
        // If there is no feature
        panic!("No file formats installed or selected. You must enable at least one format feature");
    } else {
        // If there are multiple features
        let first = features[0];
        log::warn!("Too many format features enabled, with no format specified in the extension or the config's settings.");
        log::warn!("Defaulting to picking the first available format.. ({:?})", &first);
        first
    }
}

// Creates a new string from an existing data object (Serialization)
pub fn to_string<D>(value: &D, options: &InternalOptions) -> GenericResult<String> where D: Serialize {
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
        }
    }
}


// Creates a new data object from a string (Deserialization)
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
            serde_yaml::from_str::<D>(value).generalize()
    }
}

