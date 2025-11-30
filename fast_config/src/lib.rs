#![doc = include_str!("../../README.md")]
use serde::Deserialize;
use serde::Serialize;

use std::path::Path;

#[cfg(not(any(
    feature = "json",
    feature = "json5",
    feature = "toml",
    feature = "yaml"
)))]
compile_error!("You must enable at least one format feature: `json`, `json5`, `toml`, or `yaml`");

#[cfg(feature = "derive")]
extern crate fast_config_derive;

/// Derive macro available if serde is built with `features = ["derive"]`.
#[cfg(feature = "derive")]
pub use fast_config_derive::FastConfig;

/// Enum used to configure the [`Config`]s file format.
///
/// ## ⚠️ Make sure to enable the feature flag for a format!
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Format {
    #[cfg(feature = "json")]
    JSON,
    #[cfg(feature = "json5")]
    JSON5,
    #[cfg(feature = "toml")]
    TOML,
    #[cfg(feature = "yaml")]
    YAML,
}

/// The main result error type of the crate. <br/>
/// Each type has it's own documentation.
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[cfg(feature = "json")]
    #[error(transparent)]
    Json(#[from] serde_json::Error),

    #[cfg(feature = "json5")]
    #[error(transparent)]
    Json5(#[from] json5::Error),

    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlSerialize(#[from] toml::ser::Error),
    #[cfg(feature = "toml")]
    #[error(transparent)]
    TomlDeserialize(#[from] toml::de::Error),

    #[cfg(feature = "yaml")]
    #[error(transparent)]
    Yaml(#[from] serde_yml::Error),
}

pub trait FastConfig
where
    Self: for<'a> Deserialize<'a> + Serialize + Sized,
{
    fn load(&mut self, path: impl AsRef<Path>, format: Format) -> Result<(), Error>;
    fn save(&self, path: impl AsRef<Path>, format: Format) -> Result<(), Error>;
    fn save_pretty(&self, path: impl AsRef<Path>, format: Format) -> Result<(), Error>;
    #[cfg(any(
        feature = "json",
        feature = "json5",
        feature = "toml",
        feature = "yaml"
    ))]
    fn from_string(content: &str, format: Format) -> Result<Self, Error> {
        let result = match format {
            #[cfg(feature = "json")]
            Format::JSON => serde_json::from_str::<Self>(content)?,
            #[cfg(feature = "json5")]
            Format::JSON5 => json5::from_str::<Self>(content)?,
            #[cfg(feature = "toml")]
            Format::TOML => toml::from_str::<Self>(content)?,
            #[cfg(feature = "yaml")]
            Format::YAML => serde_yml::from_str::<Self>(content)?,
        };
        Ok(result)
    }
    #[cfg(any(
        feature = "json",
        feature = "json5",
        feature = "toml",
        feature = "yaml"
    ))]
    fn to_string(&self, format: Format) -> Result<String, Error> {
        let result = match format {
            #[cfg(feature = "json")]
            Format::JSON => serde_json::to_string(self)?,
            #[cfg(feature = "json5")]
            Format::JSON5 => json5::to_string(self)?,
            #[cfg(feature = "toml")]
            Format::TOML => toml::to_string(self)?,
            #[cfg(feature = "yaml")]
            Format::YAML => serde_yml::to_string(self)?,
        };
        Ok(result)
    }
    #[cfg(any(
        feature = "json",
        feature = "json5",
        feature = "toml",
        feature = "yaml"
    ))]
    fn to_string_pretty(&self, format: Format) -> Result<String, Error> {
        let result = match format {
            #[cfg(feature = "json")]
            Format::JSON => serde_json::to_string_pretty(self)?,
            #[cfg(feature = "json5")]
            Format::JSON5 => json5::to_string(self)?,
            #[cfg(feature = "toml")]
            Format::TOML => toml::to_string_pretty(self)?,
            #[cfg(feature = "yaml")]
            Format::YAML => serde_yml::to_string(self)?,
        };
        Ok(result)
    }
}

#[cfg(test)]
#[cfg(feature = "derive")]
mod tests;