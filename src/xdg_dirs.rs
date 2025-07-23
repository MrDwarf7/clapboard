use std::str::FromStr;

use clap::crate_name;
use xdg::BaseDirectories;

use crate::Error;
use crate::prelude::PathBuf;

/// Get the XDG directory for the specified type **with** the application prefix.
///
/// You can pass in a direct `XdgDirType` or a `String`/`str` that can be parsed into one.
///
///
/// # Example:
///  ```rust,no_run
///  use clapboard::xdg_dirs::{get_xdg_dir, XdgDirType};
///  let config_dir = get_xdg_dir(XdgDirType::Config).unwrap_or_default();
///  // or
///  let data_dir = get_xdg_dir("data").unwrap_or_default();
///  ```
///
pub fn get_xdg_dir<T>(dir_type: T) -> crate::Result<PathBuf>
where
    T: Into<XdgDirType>,
{
    BaseDirectories::with_prefix(crate_name!()).to_xdg_dir(dir_type)
}

pub trait ToXdgDir {
    fn to_xdg_dir<T: Into<XdgDirType>>(&self, dir_type: T) -> crate::Result<PathBuf>;
}

impl ToXdgDir for BaseDirectories {
    fn to_xdg_dir<T>(&self, dir_type: T) -> crate::Result<PathBuf>
    where
        T: Into<XdgDirType>,
    {
        match dir_type.into() {
            XdgDirType::Config => {
                self.place_config_file(crate::prelude::CONFIGURATION_FILE_NAME)
                    .map_err(|e| Error::Generic(format!("Failed to get config directory: {e}")))
            }
            XdgDirType::Data => {
                match self.get_data_home() {
                    Some(path) => Ok(path.join(crate::prelude::CONFIGURATION_FILE_NAME)),
                    None => Err(Error::Generic("Data home directory not available".to_string())),
                }
            }
            XdgDirType::Cache => {
                match self.get_cache_home() {
                    Some(path) => Ok(path.join(crate::prelude::CONFIGURATION_FILE_NAME)),
                    None => Err(Error::Generic("Cache home directory not available".to_string())),
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XdgDirType {
    Config,
    Data,
    Cache,
}

impl From<&str> for XdgDirType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "config" => XdgDirType::Config,
            "data" => XdgDirType::Data,
            "cache" => XdgDirType::Cache,
            _ => panic!("Unknown XDG directory type: {s}"),
        }
    }
}

impl From<String> for XdgDirType {
    fn from(s: String) -> Self {
        match s.to_lowercase().as_str() {
            "config" => XdgDirType::Config,
            "data" => XdgDirType::Data,
            "cache" => XdgDirType::Cache,
            _ => panic!("Unknown XDG directory type: {s}"),
        }
    }
}

impl FromStr for XdgDirType {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "config" => Ok(XdgDirType::Config),
            "data" => Ok(XdgDirType::Data),
            "cache" => Ok(XdgDirType::Cache),
            _ => Err(Error::Generic(format!("Unknown XDG directory type: {s}"))),
        }
    }
}

#[cfg(test)]
mod xdg_dirs_tests {
    use super::*;

    #[test]
    fn test_get_xdg_dir() {
        let config_dir = get_xdg_dir(XdgDirType::Config).unwrap(); // safe here as it's testing the function
        // dbg!(&config_dir);
        // Assert that the given directory contains ".config"
        assert!(config_dir.to_string_lossy().contains(".config"));

        let data_dir = get_xdg_dir("data").unwrap();
        // dbg!(&data_dir);
        // Assert that the given directory contains "data"
        assert!(data_dir.to_string_lossy().contains("data"));

        let cache_dir = get_xdg_dir(XdgDirType::Cache).unwrap();
        // dbg!(&cache_dir);
        // Assert that the given directory contains "cache" or "local"
        assert!(
            cache_dir.to_string_lossy().contains("cache") || cache_dir.to_string_lossy().contains("local")
        );
    }
}
