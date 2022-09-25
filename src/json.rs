pub mod config {
    use std::collections::HashMap;

    use serde::{Deserialize, Serialize};

    use crate::envvar::environment_variable::env::PATH_DELIMITER;

    fn default_false() -> bool {
        false
    }

    fn default_delimiter() -> String {
        PATH_DELIMITER.to_string()
    }

    fn default_append() -> i32 {
        -1
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct ValueDetail {
        pub value: String,
        #[serde(default = "default_false")]
        pub overwrite: bool,
        #[serde(default = "default_delimiter")]
        pub delimiter: String,
        #[serde(default = "default_append")]
        pub insert: i32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Config {
        pub version: u32,

        #[serde(flatten)]
        pub map: HashMap<String, ValueDetail>,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        envvar::environment_variable::env::PATH_DELIMITER,
        json::config::{Config, ValueDetail},
    };

    #[test]
    fn test1() {
        let mut data = Config {
            version: 1,
            map: HashMap::new(),
        };

        data.map.insert(
            "a".to_string(),
            ValueDetail {
                value: "aa".to_string(),
                overwrite: false,
                delimiter: PATH_DELIMITER.to_string(),
                insert: -1,
            },
        );

        let json_str = serde_json::to_string(&data).unwrap();
        println!("{}", json_str);

        let d2 = serde_json::from_str::<Config>(&json_str).unwrap();
        println!("version: {}", d2.version);
        let avd = d2.map.get("a").unwrap();
        println!("a.value: {}", avd.value);

        assert!(false);
    }
}
