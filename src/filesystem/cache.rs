use std::io::Read;
use std::time::{SystemTime, UNIX_EPOCH};
use std::{fs, io};

use indexmap::IndexMap;

use crate::clipboard::{ClipboardEntry, ContentType};
use crate::prelude::{PathBuf, Result};

pub struct CacheManager {
    pub cache_dir: PathBuf,
}

impl CacheManager {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self { cache_dir }
    }

    pub fn load_entries(&self) -> Result<IndexMap<String, ClipboardEntry>> {
        self.read_cache_directory()
    }

    pub fn clean_history(&self, max_entries: usize) -> io::Result<()> {
        let mut entries: Vec<_> = fs::read_dir(&self.cache_dir)?
            .filter_map(|entry| entry.ok())
            .collect();

        entries.sort_by(|a, b| b.file_name().cmp(&a.file_name()));

        for (index, entry) in entries.into_iter().enumerate() {
            if index > max_entries {
                let path = entry.path();
                if path.is_dir()
                    && !path
                        .file_name()
                        .unwrap_or_default()
                        .to_string_lossy()
                        .starts_with('.')
                {
                    fs::remove_dir_all(&path)?;
                }
            }
        }
        Ok(())
    }

    pub fn save_clipboard_content(&self, content: &[u8], mime_type: &str) -> Result<String> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
            .to_string();
        let cache_subdir = self.cache_dir.join(&timestamp);
        fs::create_dir_all(&cache_subdir)?;

        let file_path = cache_subdir.join(mime_type.replace('/', "."));
        fs::write(file_path, content)?;

        Ok(timestamp)
    }

    fn read_cache_directory(&self) -> Result<IndexMap<String, ClipboardEntry>> {
        let mut data_map = IndexMap::new();
        let entries = fs::read_dir(&self.cache_dir)?.flatten().collect::<Vec<_>>();

        for entry in entries {
            let path = entry.path();
            if path.is_dir() {
                let timestamp = path.file_name().unwrap().to_string_lossy().to_string();
                let text_file = path.join("text.plain");
                if text_file.exists() {
                    let mut content = String::new();
                    std::fs::File::open(text_file)?.read_to_string(&mut content)?;
                    let display_text = content.trim().replace('\n', " ").chars().take(50).collect();
                    data_map.insert(
                        display_text,
                        ClipboardEntry {
                            display_text: content.clone(),
                            timestamp:    timestamp.clone(),
                            content_type: ContentType::Text(content),
                        },
                    );
                } else {
                    data_map.insert(
                        format!("Binary Content @ {timestamp}"),
                        ClipboardEntry {
                            display_text: format!("Binary Content @ {timestamp}"),
                            timestamp:    timestamp.clone(),
                            content_type: ContentType::Binary { timestamp },
                        },
                    );
                }
            }
        }
        Ok(data_map)
    }
}
