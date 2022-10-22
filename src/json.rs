pub mod config {
    use std::{
        collections::HashMap,
        fs::File,
        io::{Read, Write},
        path::Path,
    };

    use serde::{Deserialize, Serialize};

    use crate::envvar::{
        self,
        environment_variable::{
            env::{PATH, PATH_DELIMITER},
            EnvironmentVariable,
        },
    };

    fn default_overwrite() -> bool {
        false
    }

    fn default_delimiter() -> String {
        "".to_string()
    }

    fn default_append() -> i32 {
        -1
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ValueDetail {
        pub key: String,
        pub value: String,
        #[serde(default = "default_overwrite")]
        pub overwrite: bool,
        #[serde(default = "default_delimiter")]
        pub delimiter: String,
        #[serde(default = "default_append")]
        pub insert: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub version: u32,
        pub data: Vec<ValueDetail>,
    }

    fn create_value(key: String, value: String) -> ValueDetail {
        let delimiter = if &key.as_str() == &PATH {
            PATH_DELIMITER.to_string()
        } else {
            default_delimiter()
        };
        ValueDetail {
            key,
            value,
            overwrite: default_overwrite(),
            delimiter,
            insert: default_append(),
        }
    }

    pub fn export_envvar(filepath: &Path) -> Result<(), String> {
        let mut data = Config {
            version: 1,
            data: Vec::new(),
        };

        let envvar = envvar::environment_variable::env::Environment::new();
        match envvar.list() {
            Ok(list) => {
                for (v, d) in list {
                    data.data.push(create_value(v, d));
                }
            }
            Err(e) => return Err(e),
        }

        let json = match serde_json::to_string_pretty(&data) {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        };
        let mut file = match File::create(filepath) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };

        let write_result = file.write(json.as_bytes());
        if write_result.is_err() {
            return Err(write_result.unwrap_err().to_string());
        }

        Ok(())
    }

    pub fn import_envvar(
        filepath: &Path,
        dry_run: bool,
        envvar: &mut envvar::environment_variable::env::Environment,
    ) -> Result<(), String> {
        if !filepath.exists() {
            return Err(format!(
                "file not found. path: {}",
                filepath.to_string_lossy()
            ));
        }

        let mut file = match File::open(filepath) {
            Ok(f) => f,
            Err(e) => return Err(e.to_string()),
        };
        let mut json = String::new();
        match file.read_to_string(&mut json) {
            Ok(_) => {}
            Err(e) => return Err(format!("{}", e.to_string())),
        };

        let config = match serde_json::from_str::<Config>(json.as_str()) {
            Ok(c) => c,
            Err(e) => return Err(format!("{}", e.to_string())),
        };

        let mut current = HashMap::<String, String>::new();
        match envvar.list() {
            Ok(l) => {
                for e in l {
                    current.insert(e.0, e.1);
                }
            }
            Err(e) => return Err(e),
        }

        // division
        let mut new_values: Vec<&ValueDetail> = Vec::new();
        let mut overwrite_values: Vec<&ValueDetail> = Vec::new();
        let mut insert_values: Vec<&ValueDetail> = Vec::new();
        let mut ignore_values: Vec<&ValueDetail> = Vec::new();
        for v in config.data.iter() {
            if current.contains_key(&v.key) {
                if v.overwrite {
                    overwrite_values.push(&v);
                    continue;
                }

                if v.delimiter.len() > 0 {
                    insert_values.push(&v);
                    continue;
                }

                ignore_values.push(&v);
                continue;
            }

            new_values.push(&v);
        }

        // preview
        {
            println!("new:");
            for e in new_values.iter() {
                println!("        {}", e.key);
            }
            println!("");

            println!("overwrite:");
            for e in overwrite_values.iter() {
                println!("        {}", e.key);
            }
            println!("");

            println!("append or insert:");
            for e in insert_values.iter() {
                println!("        {}", e.key);
                println!("          index: {}", e.insert);
            }
            println!("");

            println!("ignore:");
            for e in ignore_values.iter() {
                println!("        {}", e.key);
            }
        }

        if !dry_run {
            for e in new_values.iter() {
                match envvar.set(&e.key, &e.value) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }

            for e in overwrite_values.iter() {
                match envvar.set(&e.key, &e.value) {
                    Ok(_) => {}
                    Err(e) => return Err(e),
                }
            }

            for e in insert_values.iter() {
                if e.insert < 0 {
                    match envvar.append_list(&e.key, &e.value, &e.delimiter) {
                        Ok(_) => continue,
                        Err(e) => return Err(e),
                    }
                }

                match envvar.insert_list(&e.key, &e.value, e.insert as usize, &e.delimiter) {
                    Ok(_) => continue,
                    Err(e) => return Err(e),
                }
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::json::config::{Config, ValueDetail};

    #[test]
    fn test_serialize_normal() {
        let mut data = Config {
            version: 1,
            data: Vec::new(),
        };

        data.data.push(ValueDetail {
            key: "a".to_string(),
            value: "aa".to_string(),
            overwrite: false,
            delimiter: "|".to_string(),
            insert: -1,
        });

        data.data.push(ValueDetail {
            key: "b".to_string(),
            value: "bb".to_string(),
            overwrite: true,
            delimiter: "()".to_string(),
            insert: 1,
        });

        let to_string_result = serde_json::to_string(&data);
        assert!(to_string_result.is_ok());

        let json_str = to_string_result.unwrap();
        assert_eq!(
            r#"{"version":1,"data":[{"key":"a","value":"aa","overwrite":false,"delimiter":"|","insert":-1},{"key":"b","value":"bb","overwrite":true,"delimiter":"()","insert":1}]}"#,
            json_str
        )
    }

    #[test]
    fn test_deserialize_normal() {
        let json_str = r#"{"version":1,"data":[{"key":"a","value":"aa","overwrite":false,"delimiter":"|","insert":-1},{"key":"b","value":"bb","overwrite":true,"delimiter":"()","insert":1}]}"#;
        let from_str_result = serde_json::from_str::<Config>(&json_str);
        assert!(from_str_result.is_ok());

        let data = from_str_result.unwrap();

        assert_eq!(1, data.version);
        assert!(
            data.data
                .iter()
                .filter(|e| e.key == "a".to_string())
                .collect::<Vec<&ValueDetail>>()
                .len()
                == 1
        );

        let get_a_result = data.data.get(0);
        assert!(get_a_result.is_some());
        let a = get_a_result.unwrap();
        assert_eq!("aa".to_string(), a.value);
        assert_eq!(false, a.overwrite);
        assert_eq!("|".to_string(), a.delimiter);
        assert_eq!(-1, a.insert);

        assert!(
            data.data
                .iter()
                .filter(|e| e.key == "b".to_string())
                .collect::<Vec<&ValueDetail>>()
                .len()
                == 1
        );

        let get_b_result = data.data.get(1);
        assert!(get_b_result.is_some());
        let b = get_b_result.unwrap();
        assert_eq!("bb".to_string(), b.value);
        assert_eq!(true, b.overwrite);
        assert_eq!("()".to_string(), b.delimiter);

        assert_eq!(2, data.data.len());
    }
}
