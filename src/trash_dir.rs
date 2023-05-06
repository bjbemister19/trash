use crate::os::{get_operating_system, OperatingSystem};
use crate::mtab_parser::{parse_mtab_file, MTAB_PATH};
use crate::environment::home_dir;
use std::path::Path;

fn get_trash_dir_for_file_linux(file: &str, mtab_path: &str) -> Option<String> {
    let mounted_drives = parse_mtab_file(mtab_path).ok()?;
    let mut mounted_paths = mounted_drives.iter().map(|mounted_drive| mounted_drive.mount_point.clone()).collect::<Vec<String>>();
    mounted_paths.push(home_dir());
    mounted_paths.sort_by(|a,b| b.len().cmp(&a.len()));
    for path in mounted_paths {
        if file.starts_with(&path) {
            return Some(String::from(Path::new(&path).join(".rtrash").to_str().expect("ulghhhh")));
        }
    }
    None
}

fn get_trash_dir_for_file_macos() -> String {
    let home = home_dir();
    String::from(Path::new(&home).join(".rtrash").to_str().expect("ulghh"))
}

pub fn trash_dir(file: &str) -> Option<String> {
    match get_operating_system() {
        OperatingSystem::Linux => get_trash_dir_for_file_linux(file, MTAB_PATH),
        OperatingSystem::MacOS => Some(get_trash_dir_for_file_macos()),
        _ => panic!("Unsupported OS")
    }
}

mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_mounted_drives_works(){
        let file_path = "test_mounted_drives_works";
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"/dev/sda1 /mnt/ext4 ext4 rw,relatime 0 0\n")
            .unwrap();
        file.write_all(b"tmpfs /mnt/tmpfs tmpfs rw,relatime 0 0\n")
            .unwrap();
        file.write_all(b"tmpfs /mnt/tmpfs/submount tmpfs rw,relatime 0 0\n")
        .unwrap();
        file.sync_all().unwrap();

        assert_eq!("/mnt/tmpfs/.rtrash", get_trash_dir_for_file_linux("/mnt/tmpfs/hello.txt", file_path).unwrap());
        assert_eq!("/mnt/tmpfs/submount/.rtrash", get_trash_dir_for_file_linux("/mnt/tmpfs/submount/blahh/hello.txt", file_path).unwrap());
        assert_eq!(None, get_trash_dir_for_file_linux("/noexist", file_path));

        std::fs::remove_file(file_path).unwrap();
    }
}