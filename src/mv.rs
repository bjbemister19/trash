#[derive(Debug)]
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
        todo!("implement exec");
    }

    pub fn dry_run(&self) -> Option<&Move> {
        println!("Moving to trash: {:?}", self.src);
        Some(&self)
    }
}
