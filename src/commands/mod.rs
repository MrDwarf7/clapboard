// src/commands/mod.rs
use crate::clipboard::ClipboardMode;
use crate::clipboard::recorder::ClipboardRecorder;
use crate::clipboard::selector::ClipboardSelector;
use crate::prelude::Result;

pub struct CommandOrchestrator;

// TODO: @traits - Implement a common trait for things that are 'Command' like (Recording, Selecting);

impl CommandOrchestrator {
    pub async fn execute(mode: ClipboardMode) -> Result<()> {
        match mode {
            ClipboardMode::Recording(config) => {
                println!("Clapboard recording...");
                let recorder = ClipboardRecorder::new(config);
                recorder.start_recording().await
            }
            ClipboardMode::Selecting(config) => {
                let selector = ClipboardSelector::new(config);
                selector.run_selection().await
            }
        }
    }
}
