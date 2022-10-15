pub mod arguments {
    use crate::settings::settings::Settings;
    #[cfg(target_os = "linux")]
    use crate::settings::settings::ShellRc;

    pub fn args_to_vec() -> Vec<String> {
        std::env::args().collect::<Vec<String>>()
    }

    fn get_key_value_long(
        args: &Vec<String>,
        index: usize,
    ) -> Result<(&str, Option<&str>), String> {
        let r = args.get(index);
        match r {
            Some(e) => {
                let s = e.split("=").collect::<Vec<&str>>();

                let key_result = &s.get(0);
                let value_result = &s.get(1);

                Ok((key_result.unwrap(), value_result.copied()))
            }
            _ => Err(format!("cannot get argument. index: {}", index)),
        }
    }

    pub fn parse(args: &Vec<String>) -> Result<Settings, String> {
        let mut settings = Settings {
            no_args: args.len() <= 1,
            help: false,
            no_color: false,
            dry_run: false,
            verbose: 0,
            version: false,
            export: None,
            import: None,
            set_variable: None,
            system: false,
            #[cfg(target_os = "windows")]
            registry: false,
            #[cfg(target_os = "linux")]
            shell_rc: ShellRc {
                shell: Some("bash"),
                output_rc: None,
            },
        };

        for (i, a) in args.iter().enumerate() {
            if !a.starts_with("--") {
                continue;
            }

            let (k, v) = match get_key_value_long(&args, i) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            match k {
                "--help" => settings.help = true,
                "--no-color" => settings.no_color = true,
                "--dry-run" => settings.dry_run = true,
                "--export" => {
                    settings.export = v;
                    if v.is_none() || v.unwrap().trim().len() <= 0 {
                        return Err(format!("invalid argument: {}", a));
                    }
                }
                "--import" => {
                    settings.import = v;
                    if v.is_none() || v.unwrap().trim().len() <= 0 {
                        return Err(format!("invalid argument: {}", a));
                    }
                }
                #[cfg(target_os = "linux")]
                "--shell" => {
                    settings.shell_rc.shell = v;
                    if v.is_none() || v.unwrap().trim().len() <= 0 {
                        return Err(format!("invalid argument: {}", a));
                    }

                    if settings.shell_rc.output_rc.is_none() {
                        // update default value
                        settings.shell_rc.output_rc =
                            Some(format!(".envvar_{}rc", v.unwrap()).to_string());
                    }
                }
                #[cfg(target_os = "linux")]
                "--rc" => {
                    if v.is_none() || v.unwrap().trim().len() <= 0 {
                        return Err(format!("invalid argument: {}", a));
                    }
                    settings.shell_rc.output_rc = Some(v.unwrap().to_string());
                }
                "--verbose" => match v {
                    Some(n_str) => match n_str.parse::<u32>() {
                        Ok(n) => settings.verbose = n,
                        Err(_) => return Err(format!("invalid argument: {}", a)),
                    },
                    None => settings.verbose = 1,
                },
                "--version" => settings.version = true,
                _ => settings.help = true,
            }
        }

        Ok(settings)
    }
}
