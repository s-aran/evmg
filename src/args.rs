pub mod Arguments {
    use std::env;

    use windows_sys::Win32::System::Registry::KEY_SET_VALUE;

    pub fn args_to_vec() -> Vec<String> {
        std::env::args().collect::<Vec<String>>()
    }

    fn get_key_value(args: &Vec<String>, index: usize) -> Result<(&str, Option<&str>), String> {
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

    pub fn parse(args: &Vec<String>) -> Result<bool, String> {
        for (i, a) in args.iter().enumerate() {
            if !a.starts_with("--") {
                continue;
            }

            let (k, v) = match get_key_value(args, i) {
                Ok(e) => e,
                Err(e) => return Err(e),
            };

            match k {
                "--export" => match v {
                    Some(e) => {
                        println!("specified export to {}", e);
                        return Ok(true);
                    }
                    None => println!("invalid argument: {}", a),
                },
                "--version" => {
                    println!("0.1");
                    return Ok(true);
                }
                _ => {
                    let v = println!("help");
                }
            }
        }

        Err("unexpected arguments".to_string())
    }
}
