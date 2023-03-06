use clap::ArgMatches;
use clap::{command, value_parser, Arg, ArgAction};

use std::env;
use std::path::Path;

#[derive(Debug)]
pub struct Args {
    matches: ArgMatches,
}

impl Args {
    pub fn parse() -> Args {
        let matches = command!() // requires `cargo` feature
            .arg(Arg::new("empty").long("empty").required(false))
            .arg(
                Arg::new("files")
                    .action(ArgAction::Append)
                    .value_parser(value_parser!(String)),
            )
            .get_matches();

        Args { matches: matches }
    }

    pub fn cwd() -> String {
        env::var("PWD").expect("Cannot find current directory")
    }

    pub fn home_dir() -> String {
        env::var("HOME").ok().expect("Cannot find home directory")
    }

    pub fn trash_dir() -> String {
        let home = Args::home_dir();
        String::from(Path::new(&home).join(".rtrash").to_str().expect("ulghh"))
    }

    pub fn matches(&self) -> &ArgMatches {
        &self.matches
    }

    pub fn files(&self) -> Vec<String> {
        self.matches()
            .get_many("files")
            .expect("expected files to delete")
            .cloned()
            .collect()
    }
}
