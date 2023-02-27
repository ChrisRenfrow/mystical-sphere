use std::{
    fs::File,
    io::{self, ErrorKind},
    process::exit,
    thread,
    time::Duration,
};

use clap::Parser;
use colored::*;
use mystical_sphere::{
    cli::Cli,
    config::{create_default_cfg, parse_config_at_path, Config},
    sphere::Sphere,
    DEFAULT_CONFIG_CONTENTS,
};
use shellexpand::tilde;

fn main() {
    let args = Cli::parse();
    let config_path: String = tilde(&args.config).into();
    let config: Config = match File::open(&config_path) {
        Err(ref e) if e.kind() == ErrorKind::NotFound => {
            create_default_cfg(&config_path);
            toml::from_slice(DEFAULT_CONFIG_CONTENTS).unwrap()
        }
        Err(e) => panic!("Error: {:?}", e),
        Ok(_) => parse_config_at_path(&config_path),
    };
    let sphere = Sphere::new(config.answers);

    if args.quiet {
        let mut rng = rand::thread_rng();
        println!("{}", sphere.get_answer(&mut rng));
    } else {
        run(sphere);
    }

    exit(0);
}

enum State {
    Intro,
    Input,
    Answer,
    End,
}

fn run(sphere: Sphere) {
    let mut state = State::Intro;
    let mut rng = rand::thread_rng();

    loop {
        match state {
            State::Intro => {
                println!("I am a mystical sphere. I offer you guidance through the many challenges and decisions one must make in life. But beware, the fates are ever-changing.");
                state = State::Input;
                thread::sleep(Duration::from_millis(3000));
            }
            State::Input => {
                let mut buffer = String::new();
                let stdin = io::stdin();
                println!("To begin, please enter a yes or no question for me to answer. If you do not wish to disclose your question to me, you may hold the question firmly in mind before pressing {}.", "Enter".purple());
                match stdin.read_line(&mut buffer) {
                    Ok(_) => {
                        state = State::Answer;
                    }
                    Err(_) => {
                        eprintln!("Something is wrong, come back when you've cleared your mind.");
                        exit(1);
                    }
                }
            }
            State::Answer => {
                println!("Okay, I'm seeing something...");
                thread::sleep(Duration::from_millis(2000));
                println!("It's getting clearer...");
                thread::sleep(Duration::from_millis(2000));
                println!("{}", sphere.get_answer(&mut rng));
                thread::sleep(Duration::from_millis(2000));
                state = State::End;
            }
            State::End => {
                println!("Would you like to ask another question (y/n)?");
                let mut buffer = String::new();
                let stdin = io::stdin();
                stdin.read_line(&mut buffer).unwrap();
                match buffer.to_string().trim() {
                    "y" => {
                        println!("You wish to tempt the fates again? Very well...");
                        thread::sleep(Duration::from_millis(2000));
                        state = State::Input;
                    }
                    _ => {
                        println!("Very well. May the fates guide you.");
                        exit(0);
                    }
                }
            }
        }
    }
}
