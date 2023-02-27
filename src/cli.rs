use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to alternate config (default is `~/.config/mystical-sphere/config.toml`)
    #[arg(short, long, default_value = "~/.config/mystical-sphere/config.toml")]
    pub config: String,
    /// Whether the program should run interactively or not.
    #[arg(short, long)]
    pub interactive: Option<bool>,
}
