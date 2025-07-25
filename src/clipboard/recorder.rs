use std::io::Read;

use wayland_clipboard_listener::{WlClipboardPasteStream, WlListenType};
use wl_clipboard_rs::paste::{ClipboardType, Seat, get_contents};

use crate::clipboard::{ListenerType, RecordingConfig};
use crate::filesystem::cache::CacheManager;
use crate::prelude::Result;

pub struct ClipboardRecorder {
    config: RecordingConfig,
}

impl ClipboardRecorder {
    pub fn new(config: RecordingConfig) -> Self {
        Self { config }
    }

    pub async fn start_recording(&self) -> Result<()> {
        let tasks: Vec<_> = self
            .config
            .listeners
            .iter()
            .map(|&listener_type| {
                let cache_manager = CacheManager::new(self.config.cache_dir.clone());
                let history_size = self.config.history_size;
                tokio::spawn(async move {
                    Self::listen_to_clipboard(listener_type, cache_manager, history_size).await
                })
            })
            .collect();

        for task in tasks {
            if let Err(e) = task.await {
                eprintln!("Recording task failed: {e}");
            }
        }

        Ok(())
    }

    async fn listen_to_clipboard(
        listener_type: ListenerType,
        cache_manager: CacheManager,
        history_size: usize,
    ) {
        let mut stream = WlClipboardPasteStream::init(match listener_type {
            ListenerType::Primary => WlListenType::ListenOnSelect,
            ListenerType::Clipboard => WlListenType::ListenOnCopy,
        })
        .unwrap();

        for context in stream.paste_stream().flatten().flatten() {
            for mime in &context.mime_types {
                match get_contents(
                    match listener_type {
                        ListenerType::Primary => ClipboardType::Primary,
                        ListenerType::Clipboard => ClipboardType::Regular,
                    },
                    Seat::Unspecified,
                    wl_clipboard_rs::paste::MimeType::Specific(mime),
                ) {
                    Ok((mut reader, _)) => {
                        let mut contents = Vec::new();
                        if let Err(e) = reader.read_to_end(&mut contents) {
                            eprintln!("Failed to read clipboard content: {e}");
                            continue;
                        }
                        if let Err(e) = cache_manager.save_clipboard_content(&contents, mime) {
                            eprintln!("Failed to save clipboard content: {e}");
                        }
                    }
                    Err(err) => {
                        eprintln!("Clipboard {listener_type:?} warning for mime type {mime}: {err}")
                    }
                }
            }
            if let Err(e) = cache_manager.clean_history(history_size) {
                eprintln!("Failed to clean history: {e}");
            }
        }
    }
}
use crate::clipboard::{ListenerType, RecordingConfig};
use crate::prelude::{Error, PathBuf, Result};

pub struct ClipboardRecorder {
    config: RecordingConfig,
}

impl ClipboardRecorder {
    pub fn new(config: RecordingConfig) -> Self {
        Self { config }
    }

    pub async fn start_recording(&self) -> Result<()> {
        let tasks: Vec<_> = self
            .config
            .listeners
            .iter()
            .map(|&listener_type| {
                let cache_dir = self.config.cache_dir.clone();
                let history_size = self.config.history_size;
                tokio::spawn(async move {
                    Self::listen_to_clipboard(listener_type, cache_dir, history_size).await
                })
            })
            .collect();

        for task in tasks {
            if let Err(e) = task.await {
                eprintln!("Recording task failed: {e}");
            }
        }

        Ok(())
    }

    async fn listen_to_clipboard(listener_type: ListenerType, cache_dir: PathBuf, history_size: usize) {
        // TODO: Move the existing listen_to_clipboard logic here
        //  Make it a method of ClipboardRecorder
    }
}
