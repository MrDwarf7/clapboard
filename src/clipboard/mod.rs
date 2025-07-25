pub mod recorder;
pub mod selector;

use indexmap::IndexMap;

use crate::cli::RecordingMode;
use crate::configuration::Configuration;
use crate::prelude::{PathBuf, Result};

// TODO: @optimization: We're using cache_dir in 2 sectors, we can probably do dep. injection
//  and cleanup the actual data structure (for memory wise) better.

// TODO: @traits - ClipboardAction trait for common functionality between Recording and Selecting modes

#[derive(Debug, Clone)]
pub enum ClipboardMode {
    Recording(RecordingConfig),
    Selecting(SelectingConfig),
}

#[derive(Debug, Clone)]
pub struct RecordingConfig {
    pub listeners:    Vec<ListenerType>,
    pub cache_dir:    PathBuf,
    pub history_size: usize,
}
impl RecordingConfig {
    pub fn new(listeners: Vec<ListenerType>, cache_dir: PathBuf, history_size: usize) -> Self {
        Self {
            listeners,
            cache_dir,
            history_size,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ListenerType {
    Primary,
    Clipboard,
}

#[derive(Debug, Clone)]
pub struct SelectingConfig {
    pub cache_dir:     PathBuf,
    pub launcher:      String,
    pub launcher_args: Vec<String>,
    pub favorites:     IndexMap<String, String>,
}

impl SelectingConfig {
    #[rustfmt::skip]
    pub fn new(cache_dir: PathBuf, launcher: String, launcher_args: Vec<String>, favorites: IndexMap<String, String>) -> Self {
        Self { cache_dir, launcher, launcher_args, favorites }
    }
}

// TODO: @naive_impl - using String for timestamps (unix timestamp via SystemTime better)
#[derive(Debug, Clone)]
pub struct ClipboardEntry {
    pub display_text: String,
    pub timestamp:    String,
    pub content_type: ContentType,
}

// TODO: @naive_impl - Decide on shape of data, or potentially move Favorite's out if wanting to do more with it
//  in the future.
#[derive(Debug, Clone)]
pub enum ContentType {
    Text(String),
    // Technically would allow us to store images etc. (like 'Copy Image' from browser or apps etc.)
    Binary { timestamp: String }, // data: Vec<u8>, // ??
    Favorite(String),             // Favorite { name: String, timestamp: String },
}

impl ClipboardMode {
    pub fn mode(
        recording_mode: Option<RecordingMode>, // TODO: @naive_impl - we probably don't need to Option this
        config: &Configuration,
    ) -> Result<Self> {
        match recording_mode {
            Some(mode) => {
                Ok(ClipboardMode::Recording(RecordingConfig::new(
                    mode.into(),
                    config.defaults.cache_dir.clone(),
                    config.core.history_size,
                )))
            }
            None => {
                // TODO: @CRITICAL - We need to sort these and correctly handle type differences BEFORE we get here if possible.
                //  We cannot go directly From<HashMap> to IndexMap....
                let mut index_map = IndexMap::new();
                for (key, value) in config.favourites.items.iter() {
                    index_map.insert(key.clone(), value.clone());
                }
                Ok(ClipboardMode::Selecting(SelectingConfig::new(
                    config.defaults.cache_dir.clone(),
                    config.core.launcher.clone(),
                    config.core.launcher_args.clone(),
                    index_map,
                )))
            }
        }
    }
}

impl From<RecordingMode> for Vec<ListenerType> {
    fn from(mode: RecordingMode) -> Self {
        match mode {
            RecordingMode::Primary => vec![ListenerType::Primary],
            RecordingMode::Clipboard => vec![ListenerType::Clipboard],
            RecordingMode::Both => vec![ListenerType::Primary, ListenerType::Clipboard],
        }
    }
}

impl From<&ListenerType> for &'static str {
    fn from(listener: &ListenerType) -> Self {
        match listener {
            ListenerType::Primary => "primary",
            ListenerType::Clipboard => "clipboard",
        }
    }
}

// TODO: @naive_impl - We will refactor this soon
impl From<&str> for ListenerType {
    fn from(s: &str) -> Self {
        match s {
            "primary" | "0" => Self::Primary,
            _ => Self::Clipboard,
        }
    }
}
