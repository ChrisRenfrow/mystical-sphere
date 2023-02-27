use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to alternate config (default is `~/.config/mystical-sphere/config.toml`)
    pub config: Option<String>,
}
