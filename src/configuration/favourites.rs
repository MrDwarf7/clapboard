use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Favourites {
    #[serde(rename = "items", default, flatten)]
    pub items: HashMap<String, String>,
}

#[allow(clippy::derivable_impls)]
impl Default for Favourites {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

#[cfg(test)]
mod configuration_tests {
    use super::*;

    #[test]
    fn test_favourites_default() {
        let favourites = Favourites::default();
        assert!(favourites.items.is_empty());
    }

    #[test]
    fn test_favourites_serialization() {
        let favourites = Favourites {
            items: HashMap::from([("example".to_string(), "https://example.com".to_string())]),
        };
        let serialized = toml::to_string(&favourites).expect("Failed to serialize favourites");
        assert!(serialized.contains("example = \"https://example.com\""));
    }
}
