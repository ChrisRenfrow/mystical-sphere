use std::{
    fs::File,
    io::{self, ErrorKind, Write},
    process::exit,
    thread,
    time::Duration,
};

use clap::Parser;
use colored::Colorize;
use crossterm::{
    cursor, execute,
    style::{self, Print},
    terminal,
};
use mystical_sphere::{
    cli::Cli,
    config::{create_default_cfg, parse_config_at_path, Config},
    sphere::{Answer, Sphere},
    DEFAULT_CONFIG_CONTENTS,
};
use rand::{seq::SliceRandom, Rng};
use shellexpand::tilde;

fn main() -> Result<(), io::Error> {
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
        run(sphere)?;
    }

    exit(0);
}

enum State {
    Intro,
    Input,
    Answer,
    End,
}

fn run(sphere: Sphere) -> Result<(), io::Error> {
    let mut state = State::Intro;
    let mut rng = rand::thread_rng();
    let mut w = io::stdout();

    execute!(
        w,
        style::ResetColor,
        terminal::Clear(terminal::ClearType::All),
        cursor::Show,
        cursor::DisableBlinking,
        cursor::MoveTo(0, 1),
    )?;

    loop {
        match state {
            State::Intro => {
                sphere_say(&mut w, "I am a mystical sphere. I offer you guidance through the many challenges and decisions one must make in life. But beware, the fates are ever-changing.".to_string())?;
                state = State::Input;
            }
            State::Input => {
                let mut buffer = String::new();
                let stdin = io::stdin();
                sphere_say(&mut w, format!("To begin, please enter a yes or no question for me to answer. If you do not wish to disclose your question to me, you may hold the question firmly in mind before pressing {}.", "Enter".purple()).to_string())?;
                execute!(&mut w, cursor::MoveToNextLine(1), cursor::EnableBlinking)?;
                match stdin.read_line(&mut buffer) {
                    Ok(_) => {
                        state = State::Answer;
                    }
                    Err(e) => {
                        eprintln!("Something is wrong, come back when you've cleared your mind.");
                        return Err(e);
                    }
                }
                execute!(&mut w, cursor::DisableBlinking)?;
            }
            State::Answer => {
                let answer = sphere.get_answer(&mut rng);

                sphere_say(&mut w, "Okay, I'm seeing something...".to_string())?;
                sphere_say(&mut w, "It's getting clearer...".to_string())?;
                sphere_reveal_answer(&mut w, &mut rng, answer)?;
                state = State::End;
            }
            State::End => {
                let mut buffer = String::new();
                let stdin = io::stdin();

                sphere_say(
                    &mut w,
                    "Would you like to ask another question (y/n)?".to_string(),
                )?;
                execute!(&mut w, cursor::EnableBlinking)?;
                stdin.read_line(&mut buffer).unwrap();
                execute!(&mut w, cursor::DisableBlinking)?;
                match buffer.to_string().trim() {
                    "y" => {
                        sphere_say(
                            &mut w,
                            "You wish to tempt the fates again? Very well...".to_string(),
                        )?;
                        state = State::Input;
                    }
                    _ => {
                        sphere_say(&mut w, "Very well. May the fates guide you.".to_string())?;
                        return Ok(());
                    }
                }
            }
        }
    }
}

fn sphere_say<W>(w: &mut W, s: String) -> Result<(), io::Error>
where
    W: Write,
{
    for sentence in s.split_inclusive(&['.', '?', '!'][..]) {
        for word in sentence.split_whitespace() {
            for letter in word.chars() {
                execute!(w, Print(letter))?;
                thread::sleep(Duration::from_millis(20));
            }
            execute!(w, Print(" "))?;
            w.flush()?;
            thread::sleep(Duration::from_millis(100));
        }
        thread::sleep(Duration::from_millis(500));
    }
    execute!(w, cursor::MoveToNextLine(1))
}

fn sphere_reveal_answer<W>(w: &mut W, rng: &mut impl Rng, answer: Answer) -> Result<(), io::Error>
where
    W: Write,
{
    const OBSC_CHARS: &[char] = &['░', '▒', '▓', '█', '■', '▄', '▌', '▐', '▀'];
    let answer_str = &answer.text;
    let mut sequence: Vec<usize> = (0..answer_str.len()).collect();
    sequence.shuffle(rng);
    let y_pos = cursor::position().unwrap().1;

    execute!(w, cursor::Hide)?;
    // Disperse obscuring characters
    for i in &sequence {
        execute!(
            w,
            cursor::MoveTo(*i as u16, y_pos),
            Print(format!("{}", OBSC_CHARS[rng.gen_range(0..OBSC_CHARS.len())]).purple())
        )?;
        thread::sleep(Duration::from_millis(rng.gen_range(10..=50)));
    }
    // Dramatic pause
    thread::sleep(Duration::from_millis(1500));
    // Begin revealing characters
    for i in sequence.iter().rev() {
        execute!(
            w,
            cursor::MoveTo(*i as u16, y_pos),
            Print(answer_str.chars().nth(*i).unwrap())
        )?;
        thread::sleep(Duration::from_millis(rng.gen_range(100..=300)));
    }
    thread::sleep(Duration::from_millis(200));
    // Re-print with color now that it's fully revealed.
    execute!(
        w,
        cursor::MoveTo(0, y_pos),
        Print(answer),
        cursor::MoveToNextLine(1),
        cursor::Show
    )?;

    Ok(())
}
