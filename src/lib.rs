pub mod cli;
pub mod config;
pub mod sphere;

pub const DEFAULT_CONFIG_CONTENTS: &[u8] = include_bytes!("../default.config.toml");
