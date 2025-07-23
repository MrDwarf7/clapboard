use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Core {
    #[serde(rename = "launcher", default)]
    pub launcher:     Vec<String>,
    #[serde(rename = "history_size", default)]
    pub history_size: usize,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            launcher:     vec![
                "tofi".to_string(),
                "--fuzzy-match=true".to_string(),
                "--prompt=Clapboard History: ".to_string(),
            ],
            history_size: 50,
        }
    }
}
