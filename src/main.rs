mod args;
mod envvar;
mod json;
mod utils;

use crate::envvar::environment_variable::EnvironmentVariable;
use std::io::{self, Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    writeln!(&mut stdout, "test")?;
    stdout.reset()?;
    writeln!(&mut stdout, "")?;

    let vargs = args::Arguments::args_to_vec();
    match args::Arguments::parse(&vargs) {
        Ok(b) => match b {
            true => {
                return Ok(());
            }
            false => {}
        },
        Err(e) => panic!("{}", e),
    }

    let env = envvar::environment_variable::env::Environment::new();

    for i in env.get_path().unwrap() {
        println!("{}", i);
    }

    env.set(&"TESTTEST".to_string(), &"testtest".to_string());

    env.get_list(&"PATH".to_string(), &";".to_string());

    env.set_list(
        &"TESTTESTlist".to_string(),
        &vec!["aa".to_string(), "bb".to_string(), "cc".to_string()],
        &";".to_string(),
    );

    let alr = env.append_list(
        &"TESTTESTlist".to_string(),
        &"dd".to_string(),
        &";".to_string(),
    );

    if alr.is_err() {
        panic!("{}", alr.unwrap_err());
    }

    env.insert_list(
        &"TESTTESTlist".to_string(),
        &"dd".to_string(),
        1,
        &";".to_string(),
    );

    env.remove_list_from(
        &"TESTTESTlist".to_string(),
        &"dd".to_string(),
        &";".to_string(),
    );

    env.remove_list(&"TESTTESTlist".to_string(), 1, &";".to_string());

    // let err = env.remove(&"TESTTEST".to_string());
    // if err.is_err() {
    //     panic!("{}", err.unwrap_err());
    // }

    Ok(())
}
