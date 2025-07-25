use std::fs;
use std::io::Write;
use std::process::{Command, Stdio};

use indexmap::IndexMap;
use wl_clipboard_rs::copy::{MimeType, Options, Source};

use crate::clipboard::{ClipboardEntry, ContentType, SelectingConfig};
use crate::filesystem::cache::CacheManager;
use crate::prelude::{Error, Result};

pub struct ClipboardSelector {
    config:        SelectingConfig,
    cache_manager: CacheManager,
}

impl ClipboardSelector {
    pub fn new(config: SelectingConfig) -> Self {
        let cache_manager = CacheManager::new(config.cache_dir.clone());
        Self {
            config,
            cache_manager,
        }
    }

    pub async fn run_selection(&self) -> Result<()> {
        let entries = self.load_clipboard_entries().await?;
        let selected = self.launch_menu(entries.clone()).await?;

        if let Some(entry_key) = selected {
            self.copy_to_clipboard(&entry_key, &entries).await?;
        }

        Ok(())
    }

    async fn load_clipboard_entries(&self) -> Result<IndexMap<String, ClipboardEntry>> {
        let mut entries = self.cache_manager.load_entries()?;
        self.add_favorite_entries(&mut entries);
        Ok(entries)
    }

    async fn launch_menu(&self, entries: IndexMap<String, ClipboardEntry>) -> Result<Option<String>> {
        let input = entries.keys().cloned().collect::<Vec<_>>().join("\n");

        let output = Command::new(&self.config.launcher)
            .args(&self.config.launcher_args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(|mut child| {
                child.stdin.as_mut().unwrap().write_all(input.as_bytes())?;
                child.wait_with_output()
            })
            .map_err(|e| Error::Generic(format!("Failed to start launcher: {e}")))?;

        let result = String::from_utf8_lossy(&output.stdout)
            .trim_end_matches('\n')
            .trim_end_matches('\r')
            .to_owned();

        if result.is_empty() {
            Ok(None)
        } else {
            Ok(Some(result))
        }
    }

    async fn copy_to_clipboard(
        &self,
        entry_key: &str,
        entries: &IndexMap<String, ClipboardEntry>,
    ) -> Result<()> {
        let entry = entries
            .get(entry_key)
            .ok_or_else(|| Error::Generic("Selected entry not found".to_string()))?;
        let mut opts = Options::new();
        opts.foreground(true);

        match &entry.content_type {
            ContentType::Text(content) | ContentType::Favorite(content) => {
                opts.copy(Source::Bytes(content.as_bytes().to_vec().into()), MimeType::Autodetect)?;
            }
            ContentType::Binary { timestamp } => {
                let cache_subdir = self.config.cache_dir.join(timestamp);
                let mut sources = vec![];
                for entry in fs::read_dir(cache_subdir)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
                        let _mime_type = path.file_name().unwrap().to_string_lossy().replace('.', "/");
                        let contents = fs::read(&path)?;
                        sources.push(Source::Bytes(contents.into()));
                    }
                }
                if !sources.is_empty() {
                    // This is not the right way to do this, but it's the only way with the current library
                    // opts.copy_multi(sources)?;
                }
            }
        }
        Ok(())
    }

    fn add_favorite_entries(&self, entries: &mut IndexMap<String, ClipboardEntry>) {
        for (key, value) in &self.config.favorites {
            entries.insert(
                key.clone(),
                ClipboardEntry {
                    display_text: key.clone(),
                    timestamp:    "favorite".to_string(),
                    content_type: ContentType::Favorite(value.clone()),
                },
            );
        }
    }
}
// src/clipboard/selector.rs
use crate::clipboard::{ClipboardEntry, ContentType, SelectingConfig};
use crate::prelude::{Error, Result};

pub struct ClipboardSelector {
    config: SelectingConfig,
}

impl ClipboardSelector {
    pub fn new(config: SelectingConfig) -> Self {
        Self { config }
    }

    pub async fn run_selection(&self) -> Result<()> {
        let entries = self.load_clipboard_entries().await?;
        let selected = self.launch_menu(entries).await?;

        if let Some(entry) = selected {
            self.copy_to_clipboard(entry).await?;
        }

        Ok(())
    }

    async fn load_clipboard_entries(&self) -> Result<Vec<ClipboardEntry>> {
        let mut entries = Vec::new();

        // Load from cache directory
        // self.load_cached_entries(&mut entries).await?;
        todo!("Implement loading cached entries");

        // Add favorites
        // self.add_favorite_entries(&mut entries);
        todo!("Implement adding favorite entries");

        Ok(entries)
    }

    async fn launch_menu(&self, entries: Vec<ClipboardEntry>) -> Result<Option<ClipboardEntry>> {
        // Extract the launcher logic here
        // Return the selected entry instead of just a string
        todo!()
    }

    async fn copy_to_clipboard(&self, entry: ClipboardEntry) -> Result<()> {
        match entry.content_type {
            ContentType::Text(content) => {
                // Handle text content
            }
            ContentType::Binary { timestamp } => {
                // Handle binary content from cache
            }
            ContentType::Favorite(content) => {
                // Handle favorite content
            }
        }
        Ok(())
    }
}
