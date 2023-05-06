mod cmd;
mod environment;
mod history;
mod mtab_parser;
mod mv;
mod os;
mod trash_dir;

use std::{fs};
use std::path::Path;
use std::io;

use cmd::{Cmd, RemoveArgs, EmptyArgs};
use history::{Command, History};
use mv::Move;
use trash_dir::trash_dir;

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

fn gen_out_path(src: &str, current_time_unix: u64) -> String {
    let trash = trash_dir(src);
    if trash.is_some() {
        make_absolute(&trash.unwrap(), &gen_new_name(src, current_time_unix))
    } else {
        panic!("Could not find trash dir");
    }
}

fn get_files(args: &RemoveArgs) -> Vec<Move> {
    let cwd = environment::cwd();
    args.files()
        .iter()
        .map(|path| make_absolute(&cwd, &path))
        .map(|src| {
            let dest = gen_out_path(&src, 1);
            Move::new(&src, &dest)
        })
        .collect()
}

fn init_trash() {
    fs::create_dir_all(environment::trash_file_dir()).unwrap();
}

fn remove(args: RemoveArgs, hist: &mut History){
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
        if let Some(mv) = maybe_moved {
            c.add_file(mv.clone());
        }
    }
    hist.add_command(c);
    hist.save().unwrap();
}

fn empty(_args: EmptyArgs) {
    let directory = fs::read_dir(environment::trash_file_dir()).unwrap()
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>().ok().unwrap();

    for path in directory {
        if path.to_str().unwrap() == History::path() {
            continue;
        }

        if path.is_file() {
            fs::remove_file(path).ok().unwrap()
        } else {
            fs::remove_dir_all(path).ok().unwrap()
        }
    }
}

fn main() {
    init_trash();

    let cmd = Cmd::parse();
    // dbg!(&cmd);
    let mut hist = History::load().expect("Fatal Error: Failed to load history file");

    match cmd {
        Cmd::Remove { args } => {
            remove(args, &mut hist)
        }
        Cmd::Empty { args } => {
            empty(args);
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
