use clapboard::clipboard::ClipboardMode;
use clapboard::commands::CommandOrchestrator;
use clapboard::configuration::Configuration;
use clapboard::*;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::new();
    let conf = Configuration::try_new().unwrap_or_default();
    let mode = ClipboardMode::mode(cli.recording_mode.into(), &conf)?;
    CommandOrchestrator::execute(mode).await
}
