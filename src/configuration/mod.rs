mod core;
mod defaults;
mod favourites;

use clap::crate_name;

use crate::configuration::core::Core;
use crate::configuration::defaults::Defaults;
use crate::configuration::favourites::Favourites;
use crate::prelude::*;
use crate::xdg_dirs::get_xdg_dir;
use crate::{Error, Result};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Configuration {
    /// Internal path to the Self -> aka configuration file.
    #[serde(skip)]
    pub path: PathBuf,

    /// Core configuration settings.
    #[serde(rename = "core", default)]
    pub core: Core,

    /// Default settings for the application. EG cache directory.
    #[serde(rename = "defaults", default)]
    pub defaults: Defaults,

    /// Favourites configuration, containing user-defined favourites.
    #[serde(rename = "favourites", default)]
    pub favourites: Favourites,
}

impl Configuration {
    pub fn try_new() -> Result<Self> {
        // TODO: @configuration_overrides: Make configuration overridable via environment variables
        // if let Some(envs) = std::env::vars().find(|(k, _)| k.starts_with(CONFIGURATION_ENV_PREFIX)) ///// etc etc.
        // will need to use config crate's environment variable support

        let default_base = Configuration::default();

        let builder = config::Config::builder()
            .add_source(config::Config::try_from(&default_base).map_err(Error::ConfigParse)?);

        let config = builder.build();

        match config {
            Ok(cfg) => {
                let mut config: Self = cfg.try_deserialize().map_err(Error::ConfigParse)?;
                config.path = default_base.path;
                Ok(config)
            }
            Err(e) => Err(Error::ConfigParse(e)),
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        // TODO: @default_generation: I can re-write this and handle it a lot more gracefully lol
        let path = get_xdg_dir("config").unwrap_or_else(|_| default_config_path());
        if !path.exists() {
            let parent = path.parent().unwrap(); // Parent has to exist, un call is safe, we panic otherwise
            std::fs::create_dir_all(parent).unwrap_or_else(|_| {
                panic!("Failed to create configuration directory: {}", parent.display());
            });
            std::fs::File::create(&path).unwrap_or_else(|_| {
                panic!("Failed to create configuration file: {}", path.display());
            });

            let default = toml::to_string(&Configuration::default())
                .expect("Failed to serialize default configuration");
            std::fs::write(&path, default).unwrap_or_else(|_| {
                panic!("Failed to write default configuration to file: {}", path.display());
            });
        }

        Self {
            path,
            core: Core::default(),
            defaults: Defaults::default(),
            favourites: Favourites::default(),
        }
    }
}

fn default_config_path() -> PathBuf {
    let mut path = PathBuf::new();
    path.push(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()));
    path.push(".config");
    path.push(crate_name!());
    path.push(CONFIGURATION_FILE_NAME);
    path
}

#[cfg(test)]
mod configuration_tests {
    use std::path::Path;

    use super::*;

    // TODO: @default_generation: Make use of jail's / TmpFile crate for dependencies.dev
    fn cleanup_configuration_file(path: &Path) {
        if path.exists() {
            std::fs::remove_file(path).unwrap();
        }
        if path.parent().unwrap().exists() {
            std::fs::remove_dir_all(path.parent().unwrap()).unwrap();
        }
    }

    #[test]
    fn test_configuration_default() {
        let config = Configuration::default();
        // dbg!(&config.path);
        // Assert the file ends as "config.toml"
        assert!(config.path.ends_with("config.toml"), "Configuration path should end with 'config.toml'");
        cleanup_configuration_file(&config.path);
    }

    #[test]
    fn test_configuration_new() {
        let config = Configuration::try_new();
        // dbg!(&config);
        assert!(config.is_ok(), "Configuration should be created successfully");
        let config = config.unwrap();
        // dbg!(&config.path);
        // Assert the file ends as "config.toml"
        assert!(config.path.ends_with("config.toml"), "Configuration path should end with 'config.toml'");
        cleanup_configuration_file(&config.path);
    }
}
