use crate::environment;
use crate::mv::Move;

use std::fs;
use std::path::Path;

use serde_derive::{Deserialize, Serialize};

const HIST_FILE_NAME: &str = ".history";

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    files: Vec<Move>,
}

impl Command {
    pub fn new() -> Command {
        Command { files: Vec::new() }
    }

    pub fn add_file(&mut self, file: Move) {
        self.files.push(file);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct History {
    history: Vec<Command>,
}

impl History {
    fn path() -> String {
        String::from(
            Path::new(&environment::trash_file_dir())
                .join(HIST_FILE_NAME)
                .to_str()
                .expect("Could not join paths"),
        )
    }

    fn new() -> History {
        let history = History {
            history: Vec::new(),
        };

        history
    }

    fn parse(json: &str) -> Result<History, String> {
        serde_json::from_str(json).map_err(|e| e.to_string())
    }

    fn to_json(history: &History) -> Result<String, String> {
        serde_json::to_string_pretty(history).map_err(|e| e.to_string())
    }

    pub fn load() -> Result<History, String> {
        if Path::new(&History::path()).exists() {
            let json = fs::read_to_string(History::path()).map_err(|e| e.to_string())?;
            let hist = History::parse(&json)?;
            return Ok(hist);
        } else {
            let hist = History::new();
            let json = History::to_json(&hist)?;
            fs::write(History::path(), json).map_err(|e| e.to_string())?;
            Ok(hist)
        }
    }

    pub fn add_command(&mut self, cmd: Command) {
        self.history.push(cmd);
    }

    pub fn save(&self) -> Result<(), String> {
        let json = History::to_json(self)?;
        fs::write(History::path(), json).map_err(|e| e.to_string())?;
        Ok(())
    }
}

#[cfg(test)]
mod history_tests {
    use super::*;

    #[test]
    fn verify_path() {
        let home_dir = std::env::var("HOME").ok().expect("Cannot find home directory");
        let expected_path = home_dir + "/.rtrash/" +  HIST_FILE_NAME;
        assert_eq!(expected_path, History::path());
    }

    #[test]
    fn can_parse_new_history() {
        let _ = History::parse(&History::to_json(&History::new()).unwrap()).unwrap();
    }
}
