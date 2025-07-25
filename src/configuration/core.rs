use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Core {
    #[serde(rename = "launcher", default)]
    pub launcher: String,

    #[serde(rename = "launcher_args", default)]
    pub launcher_args: Vec<String>,

    #[serde(rename = "history_size", default)]
    pub history_size: usize,
}

impl Default for Core {
    fn default() -> Self {
        Self {
            launcher:      "tofi".to_string(),
            launcher_args: vec![
                "--fuzzy-match=true".to_string(),
                "--prompt=Clapboard History: ".to_string(),
            ],
            history_size:  50,
        }
    }
}
