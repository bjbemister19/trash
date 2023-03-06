mod args;
mod mv;

use std::fs;
use std::path::Path;

use args::Args;
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

fn get_files(args: &Args) -> Vec<Move> {
    let cwd = Args::cwd();
    let trash_dir = Args::trash_dir();
    args.files()
        .iter()
        .map(|path| make_absolute(&cwd, &path))
        .map(|src| {
            let dest = gen_out_path(&src, &trash_dir, 1);
            Move::new(&src, &dest)
        })
        .collect()
}

fn init_trash(args: &Args) {
    fs::create_dir_all(Args::trash_dir()).unwrap();
}

fn main() {
    let args = Args::parse();
    init_trash(&args);

    let files_to_move = get_files(&args);
    let moved: Vec<Option<&Move>> = files_to_move.iter().map(|file| file.exec()).collect();
    print!("{:?}", moved);
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
