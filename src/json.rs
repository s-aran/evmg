pub mod config {
    use std::collections::BTreeMap;

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
        pub map: BTreeMap<String, ValueDetail>,
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;

    use crate::json::config::{Config, ValueDetail};

    #[test]
    fn test_serialize_normal() {
        let mut data = Config {
            version: 1,
            map: BTreeMap::new(),
        };

        data.map.insert(
            "a".to_string(),
            ValueDetail {
                value: "aa".to_string(),
                overwrite: false,
                delimiter: "|".to_string(),
                insert: -1,
            },
        );

        data.map.insert(
            "b".to_string(),
            ValueDetail {
                value: "bb".to_string(),
                overwrite: true,
                delimiter: "()".to_string(),
                insert: 1,
            },
        );

        let to_string_result = serde_json::to_string(&data);
        assert!(to_string_result.is_ok());

        let json_str = to_string_result.unwrap();
        assert_eq!(
            r#"{"version":1,"a":{"value":"aa","overwrite":false,"delimiter":"|","insert":-1},"b":{"value":"bb","overwrite":true,"delimiter":"()","insert":1}}"#,
            json_str
        )
    }

    #[test]
    fn test_deserialize_normal() {
        let json_str = r#"{"version":1,"a":{"value":"aa","overwrite":false,"delimiter":"|","insert":-1},"b":{"value":"bb","overwrite":true,"delimiter":"()","insert":1}}"#;
        let from_str_result = serde_json::from_str::<Config>(&json_str);
        assert!(from_str_result.is_ok());

        let data = from_str_result.unwrap();

        assert_eq!(1, data.version);
        assert!(data
            .map
            .keys()
            .collect::<Vec<&String>>()
            .contains(&&"a".to_string()));
        let get_a_result = data.map.get("a");
        assert!(get_a_result.is_some());
        let a = get_a_result.unwrap();
    }
}
