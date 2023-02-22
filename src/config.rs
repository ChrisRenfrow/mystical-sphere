use std::{
    fs::{self, create_dir_all, File},
    io::Write,
    path::Path,
};

use serde::Deserialize;

use crate::DEFAULT_CONFIG_CONTENTS;

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct Config {
    pub answers: CfgAnswers,
}

#[derive(Debug, Deserialize, Eq, PartialEq)]
pub struct CfgAnswers {
    pub affirmative: Vec<String>,
    pub neutral: Vec<String>,
    pub negative: Vec<String>,
}

pub fn create_default_cfg(path: &String) {
    dbg!(&path);
    let dirs = Path::new(&path)
        .ancestors()
        .nth(1)
        .expect("No ancestors, whoops!");
    dbg!(&dirs);
    create_dir_all(dirs).expect("Couldn't create directory for path");
    let mut f = File::create(&path[..]).expect("Couldn't create new file at provided config path.");
    f.write_all(DEFAULT_CONFIG_CONTENTS)
        .expect("Couldn't write default config contents to file at path");
}

pub fn parse_config_at_path(path: &String) -> Config {
    toml::from_slice(&fs::read(path).expect("Couldn't read config file")).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_cfg() {
        let input = "[answers]\naffirmative = [\"foo\"]\nneutral = [\"bar\"]\nnegative = [\"baz\"]";
        let expect = Ok(Config {
            answers: CfgAnswers {
                affirmative: vec!["foo".to_string()],
                neutral: vec!["bar".to_string()],
                negative: vec!["baz".to_string()],
            },
        });
        let actual: Result<Config, _> = toml::from_str(input);

        assert_eq!(expect, actual)
    }
}
