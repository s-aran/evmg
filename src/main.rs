mod args;
mod envvar;
mod json;
mod settings;
mod utils;

use crate::{envvar::environment_variable::EnvironmentVariable, json::config};
use std::{
    io::{self, Write},
    path::Path,
};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

fn main() -> io::Result<()> {
    println!("Hello, world!");

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Blue)))?;
    writeln!(&mut stdout, "test")?;
    stdout.reset()?;
    writeln!(&mut stdout, "")?;

    let vargs = args::arguments::args_to_vec();
    let settings = match args::arguments::parse(&vargs) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if settings.help {
        utils::utils::show_help();
        return Ok(());
    }

    if settings.version {
        println!("0.1");
        return Ok(());
    }

    if settings.dry_run {
        println!("dry-run mode: on");
    }

    if settings.export.is_some() {
        println!("export to {}.", settings.export.unwrap());
        config::export_envvar(Path::new(settings.export.unwrap()));
        return Ok(());
    }

    if settings.import.is_some() {
        println!("import from {}.", settings.import.unwrap());
        return Ok(());
    }

    if settings.no_args {
        println!("no args!!");
    }

    println!("verbose: {}", settings.verbose);

    let env = envvar::environment_variable::env::Environment::new();

    match env.list() {
        Ok(_) => {}
        Err(e) => panic!("{}", e),
    }

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
