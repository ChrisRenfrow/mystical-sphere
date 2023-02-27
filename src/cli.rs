use clap::Parser;

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to alternate config
    #[arg(short, long, default_value = "~/.config/mystical-sphere/config.toml")]
    pub config: String,
    /// If present, the program will only output a random answer and quit
    #[arg(short, long)]
    pub quiet: bool,
}
