use clap::{command, value_parser, Arg, ArgAction};

use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug)]
struct Args {
    files: Vec<String>,
    empty: bool,
    home: String,
    trash_dir: String,
}

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

fn get_args() -> Args {
    let matches = command!() // requires `cargo` feature
        .arg(Arg::new("empty").long("empty").required(false))
        .arg(
            Arg::new("files")
                .action(ArgAction::Append)
                .value_parser(value_parser!(String)),
        )
        .get_matches();

    let cwd = env::var("PWD").expect("Cannot find current directory");
    let files: Vec<String> = matches
        .get_many("files")
        .expect("expected files to delete")
        .cloned::<String>()
        .map(|path| make_absolute(&cwd, &path))
        .collect();

    let home = env::var("HOME").ok().expect("Cannot find home directory");
    let trash_dir = String::from(Path::new(&home).join(".rtrash").to_str().expect("ulghh"));
    return Args {
        files: files,
        empty: matches.contains_id("empty"),
        home: home,
        trash_dir: trash_dir,
    };
}

fn move_file_to_trash(args: &Args, src: &str) {
    let dest = make_absolute(&args.trash_dir, &gen_new_name(src, 1));
    println!("{}", dest)
}

fn move_files_to_trash(args: &Args) {
    for src_path in &args.files {
        move_file_to_trash(args, &src_path)
    }
}

fn init_trash(args: &Args) {
    fs::create_dir_all(args.trash_dir.clone()).unwrap();
}

fn main() {
    let args = get_args();
    println!("{:?}", args);

    init_trash(&args);
    if args.empty {
        unimplemented!()
    } else {
        move_files_to_trash(&args);
    }
}

#[cfg(test)]
mod tests {
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
