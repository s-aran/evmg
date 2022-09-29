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
        match config::export_envvar(Path::new(settings.export.unwrap())) {
            Ok(_) => return Ok(()),
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        }
    }

    if settings.import.is_some() {
        println!("import from {}.", settings.import.unwrap());
        return Ok(());
    }

    let env = envvar::environment_variable::env::Environment::new();

    match env.list() {
        Ok(l) => {
            let mut stdout = StandardStream::stdout(ColorChoice::Always);
            for (k, v) in l.iter() {
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Cyan)))?;
                write!(&mut stdout, "{}", k)?;
                stdout.reset()?;
                write!(&mut stdout, "=")?;
                stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)))?;
                writeln!(&mut stdout, "{}", v)?;
            }
            stdout.reset()?;
        }
        Err(e) => panic!("{}", e),
    }

    Ok(())
}
