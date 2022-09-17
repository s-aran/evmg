use crate::envvar::environment_variable::{env, EnvironmentVariable};

mod envvar;

fn main() {
    println!("Hello, world!");
    let env = envvar::environment_variable::env::Environment::new();

    for i in env.get_path().unwrap() {
        println!("{}", i);
    }

    env.set(&"TESTTEST".to_string(), &"testtest".to_string());

    let err = env.remove(&"TESTTEST".to_string());
    if err.is_err() {
        panic!("{}", err.unwrap_err());
    }
}
