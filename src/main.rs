use std::{fs::File, io::ErrorKind};

use clap::Parser;
use mystical_sphere::{
    cli::Cli,
    config::{create_default_cfg, parse_config_at_path, Config},
    sphere::Sphere,
    DEFAULT_CONFIG_CONTENTS, DEFAULT_CONFIG_PATH,
};
use shellexpand::tilde;

fn main() {
    let args = Cli::parse();
    let config_path: String = tilde(
        &args
            .config
            .unwrap_or_else(|| DEFAULT_CONFIG_PATH.to_string()),
    )
    .into();
    let config: Config = match File::open(&config_path) {
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            create_default_cfg(&config_path);
            toml::from_slice(DEFAULT_CONFIG_CONTENTS).unwrap()
        }
        Err(e) => panic!("Error: {:?}", e),
        Ok(_) => parse_config_at_path(&config_path),
    };
    let mut rng = rand::thread_rng();
    let sphere = Sphere::new(config.answers);
    let answer = sphere.get_answer(&mut rng);
    println!("Answer: {}", answer.text);
}
