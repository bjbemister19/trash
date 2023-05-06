use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Move {
    pub src: String,
    pub dest: String,
}

impl Move {
    pub fn new(src: &str, dest: &str) -> Move {
        Move {
            src: String::from(src),
            dest: String::from(dest),
        }
    }

    pub fn exec(&self) -> Option<&Move> {
        fs::rename(&self.src, &self.dest).ok()?;
        Some(&self)
    }

    pub fn dry_run(&self) -> Option<&Move> {
        println!("Moving: src:{:?} dest:{:?}", self.src, self.dest);
        Some(&self)
    }
}
