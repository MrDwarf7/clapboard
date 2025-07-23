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
    /// Record mode, choose between "primary", "clipboard", or the default "both"
    // #[arg(short, long, default_missing_value = "both")]
    #[arg(value_enum,name = "record_mode",short = 'r',long = "record",help = "Set the record mode",required = false,default_value = "BOTH",value_hint = clap::ValueHint::Other)]
    pub recording_mode: Option<RecordMode>,
}

impl Default for Cli {
    fn default() -> Self {
        // Prefer deferring to the new impl. incase we want to add additional logic.
        Self::new()
    }
}

impl Cli {
    pub fn new() -> Self {
        Self::parse()
    }
}

#[derive(Debug, ValueEnum, Clone, Copy, PartialEq, Eq)]
#[clap(name = "RecordMode", rename_all = "lower")]
pub enum RecordMode {
    #[value(name = "primary", alias = "PRIMARY", alias = "0")]
    Primary,
    #[value(name = "clipboard", alias = "CLIPBOARD", alias = "1")]
    Clipboard,
    #[value(name = "both", alias = "BOTH", alias = "2")]
    Both,
}

impl FromStr for RecordMode {
    type Err = crate::error::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "primary" | "0" => Ok(RecordMode::Primary),
            "clipboard" | "1" => Ok(RecordMode::Clipboard),
            "both" | "2" => Ok(RecordMode::Both),
            _ => Err(crate::error::Error::InvalidRecordMode(s.to_string())),
        }
    }
}

impl From<RecordMode> for Vec<String> {
    fn from(mode: RecordMode) -> Self {
        match mode {
            RecordMode::Primary => vec!["primary".to_string()],
            RecordMode::Clipboard => vec!["clipboard".to_string()],
            RecordMode::Both => vec!["primary".to_string(), "clipboard".to_string()],
        }
    }
}

impl From<RecordMode> for Vec<&str> {
    fn from(mode: RecordMode) -> Self {
        match mode {
            RecordMode::Primary => vec!["primary"],
            RecordMode::Clipboard => vec!["clipboard"],
            RecordMode::Both => vec!["primary", "clipboard"],
        }
    }
}

impl From<RecordMode> for String {
    fn from(mode: RecordMode) -> Self {
        match mode {
            RecordMode::Primary => "primary".to_string(),
            RecordMode::Clipboard => "clipboard".to_string(),
            RecordMode::Both => "both".to_string(),
        }
    }
}

impl Display for RecordMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", String::from(*self))
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
