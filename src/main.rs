mod cmd;
mod environment;
mod history;
mod mv;
mod mtab_parser;

use std::fs;
use std::path::Path;

use cmd::{Cmd, RemoveArgs};
use history::{Command, History};
use mv::Move;

fn make_absolute(cwd: &str, path: &str) -> String {
    String::from(
        Path::new(cwd)
            .join(path)
            .to_str()
            .expect("Don't support filesystems with invalid UTF8 paths"),
    )
}

fn gen_new_name(path: &str, current_time_unix: u64) -> String {
    let current_filename = Path::new(path)
        .file_name()
        .expect("Path ends in .. or .")
        .to_str()
        .expect("Don't support filesystems with invalid UTF8 paths");

    format!("v1-{}-{}", current_time_unix, current_filename)
}

fn gen_out_path(src: &str, trash_dir: &str, current_time_unix: u64) -> String {
    make_absolute(trash_dir, &gen_new_name(src, current_time_unix))
}

fn get_files(args: &RemoveArgs) -> Vec<Move> {
    let cwd = environment::cwd();
    let trash_dir = environment::trash_dir();
    args.files()
        .iter()
        .map(|path| make_absolute(&cwd, &path))
        .map(|src| {
            let dest = gen_out_path(&src, &trash_dir, 1);
            Move::new(&src, &dest)
        })
        .collect()
}

fn init_trash() {
    fs::create_dir_all(environment::trash_dir()).unwrap();
}

fn main() {
    init_trash();

    let cmd = Cmd::parse();
    // dbg!(&cmd);
    let mut hist = History::load()
        .expect("Fatal Error: Failed to load history file");

    match cmd {
        Cmd::Remove { args } => {
            let files_to_move = get_files(&args);
            let moved: Vec<Option<&Move>> = files_to_move
                .iter()
                .map(|file| {
                    if args.dry_run() {
                        file.dry_run()
                    } else {
                        file.exec()
                    }
                })
                .collect();
            // TODO Update history
            let mut c = Command::new();
            for maybe_moved in moved {
                if let Some(mv) = maybe_moved{
                    c.add_file(mv.clone());
                }
            }
            hist.add_command(c);
            hist.save().unwrap();
        }
        Cmd::Empty { args: _ } => {
            todo!("Empty NYI")
        }
    }
}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn make_relative_path_absolute() {
        let abs_path = make_absolute("/home/brandon", "hello.txt");
        assert_eq!(abs_path, String::from("/home/brandon/hello.txt"))
    }

    #[test]
    fn make_absolute_path_absolute() {
        let abs_path = make_absolute("/home/brandon", "/home/brandon/hello.txt");
        assert_eq!(abs_path, String::from("/home/brandon/hello.txt"))
    }

    #[test]
    fn gen_new_name_relative() {
        let new_name = gen_new_name("file1.txt", 123456789);
        assert_eq!(new_name, "v1-123456789-file1.txt")
    }

    #[test]
    fn gen_new_name_absolute() {
        let new_name = gen_new_name("/home/brandon/file1.txt", 123456789);
        assert_eq!(new_name, "v1-123456789-file1.txt")
    }
}
