use crate::prelude::{Deserialize, Serialize, *};
use crate::xdg_dirs::get_xdg_dir;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(rename = "cache_dir", default)]
    pub cache_dir: PathBuf,
}

impl Default for Defaults {
    fn default() -> Self {
        let cache_dir = get_xdg_dir("cache").unwrap_or_else(|_| PathBuf::from("/tmp/clapboard_cache"));
        Self { cache_dir }
    }
}
