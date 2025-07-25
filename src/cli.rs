use std::fmt::Display;
use std::str::FromStr;

use clap::{Parser, ValueEnum};

/// Clapboard, a clipboard manager for Wayland
#[derive(Debug, Parser, Clone)]
#[command(
name = "clapboard",
about = "Clapboard, a clipboard manager for Wayland",
author = "Your Name <",
long_about = "\n
Clapboard is a clipboard manager designed for Wayland,
allowing users to manage their clipboard history and preferences easily.",
version = clap::crate_version!(),
arg_required_else_help = false,
styles=get_styles(),
)]
#[rustfmt::skip]
pub struct Cli {
    /// Recording mode, choose between "primary", "clipboard", or the default "both"
    #[arg(value_enum, name = "recording_mode", short = 'r', long = "recording_mode", help = "Set the record mode", required = false, default_value = "both", value_hint = clap::ValueHint::Other)]
    pub recording_mode: RecordingMode,
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[clap(name = "RecordingMode", rename_all = "lower")]
pub enum RecordingMode {
    #[value(name = "primary", alias = "PRIMARY", alias = "0")]
    Primary,
    #[value(name = "clipboard", alias = "CLIPBOARD", alias = "1")]
    Clipboard,
    #[value(name = "both", alias = "BOTH", alias = "2")]
    Both,
}

impl FromStr for RecordingMode {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" | "0" => Ok(RecordingMode::Primary),
            "clipboard" | "1" => Ok(RecordingMode::Clipboard),
            "both" | "2" => Ok(RecordingMode::Both),
            _ => Err(crate::error::Error::InvalidRecordingMode(s.to_string())),
        }
    }
}

impl Display for RecordingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            RecordingMode::Primary => "primary",
            RecordingMode::Clipboard => "clipboard",
            RecordingMode::Both => "both",
        };
        write!(f, "{s}")
    }
}

/// Returns a set of custom styles for the CLI tool.
///
/// This function defines and returns a set of styles to be used in the CLI tool's help and error messages.
/// The styles include formatting for usage, headers, literals, invalid inputs, errors, valid inputs, and placeholders.
///
/// # Returns
///
/// * `clap::builder::Styles` - Returns a `Styles` instance with the defined custom styles.
///
///
pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))), // When a command is inc. This is the tag collor for 'Usage'
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Blue))), // Main headers in the help menu (e.g., Arguments, Options)
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::BrightWhite))), // Strings for args etc. { -t, --total }
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red)))
                .effects(anstyle::Effects::ITALIC),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Cyan))),
        )
        .placeholder(anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))))
}
