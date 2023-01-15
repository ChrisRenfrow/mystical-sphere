use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Path to alternate config (default is `~/.config/mystical-sphere/config.toml`)
    /// ```
    /// assert_eq!(mystical_sphere::DEFAULT_CONFIG_PATH, "~/.config/mystical-sphere/config.toml");
    /// ```
    pub config: Option<String>,
}
