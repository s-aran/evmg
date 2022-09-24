mod envvar;
pub mod utils;

use crate::envvar::environment_variable::EnvironmentVariable;

fn main() {
    println!("Hello, world!");
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

    // let err = env.remove(&"TESTTEST".to_string());
    // if err.is_err() {
    //     panic!("{}", err.unwrap_err());
    // }
}
