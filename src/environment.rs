use std::env;
use std::path::Path;

pub fn cwd() -> String {
    env::var("PWD").expect("Cannot find current directory")
}

pub fn home_dir() -> String {
    env::var("HOME").ok().expect("Cannot find home directory")
}

pub fn trash_dir() -> String {
    let home = home_dir();
    String::from(Path::new(&home).join(".rtrash").to_str().expect("ulghh"))
}
