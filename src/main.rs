mod args;
mod envvar;
mod json;
mod settings;
mod utils;

#[cfg(target_os = "linux")]
mod shellrc;

use crate::{envvar::environment_variable::EnvironmentVariable, json::config};
#[cfg(target_os = "linux")]
use shellrc::shellrc::ShellRunCommandFileData;
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
        const NAME: Option<&str> = option_env!("CARGO_PKG_NAME");
        const VERSION: Option<&str> = option_env!("CARGO_PKG_VERSION");

        println!("{} {}", NAME.unwrap_or("???"), VERSION.unwrap_or("???"));
        return Ok(());
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
        let mut envvar = envvar::environment_variable::env::Environment::new();
        match config::import_envvar(
            Path::new(settings.import.unwrap()),
            settings.dry_run,
            &mut envvar,
        ) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(1);
            }
        };

        #[cfg(target_os = "linux")]
        {
            let output_path = match settings.shell_rc.output_rc {
                Some(s) => s,
                None => format!(".envvar_{}rc", settings.shell_rc.shell.unwrap()),
            };

            envvar.init_shell(&settings.shell_rc.shell.unwrap().to_string());
            match envvar.write_rc(&Path::new(&output_path)) {
                Ok(_) => return Ok(()),
                Err(e) => {
                    eprintln!("{}", e);
                    std::process::exit(1);
                }
            }
        }
    }

    let env = envvar::environment_variable::env::Environment::new();

    match env.list() {
        Ok(l) => {
            let choice = if settings.no_color {
                ColorChoice::Never
            } else {
                ColorChoice::Always
            };
            let mut stdout = StandardStream::stdout(choice);
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
