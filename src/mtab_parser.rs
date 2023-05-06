use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct MountedDrive {
    pub device: String,
    pub mount_point: String,
    pub file_system: String,
    pub options: String,
}

pub static MTAB_PATH: &str = "/etc/mtab";

pub fn parse_mtab_file(file_path: &str) -> Result<Vec<MountedDrive>, Box<dyn std::error::Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut mounted_drives = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if !line.trim().starts_with('#') {
            let fields: Vec<_> = line.split_whitespace().collect();
            if fields.len() >= 4 {
                let device = fields[0].to_string();
                let mount_point = fields[1].to_string();
                let file_system = fields[2].to_string();
                let options = fields[3].to_string();

                let mounted_drive = MountedDrive {
                    device,
                    mount_point,
                    file_system,
                    options,
                };
                mounted_drives.push(mounted_drive);
            }
        }
    }

    Ok(mounted_drives)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;

    #[test]
    fn test_parse_mtab_file() {
        let test_file = "test_parse_mtab_file";
        let mut file = File::create(test_file).unwrap();
        file.write_all(b"/dev/sda1 /mnt/ext4 ext4 rw,relatime 0 0\n")
            .unwrap();
        file.write_all(b"tmpfs /mnt/tmpfs tmpfs rw,relatime 0 0\n")
            .unwrap();
        file.sync_all().unwrap();

        let mounted_drives = parse_mtab_file(test_file).unwrap();
        assert_eq!(mounted_drives.len(), 2);

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_parse_mtab_file_nonexistent_file() {
        let test_file = "test_parse_mtab_file_nonexistent_file";
        let result = parse_mtab_file(test_file);
        assert!(result.is_err()); // Assert that the result is an error
    }

    #[test]
    fn test_parse_mtab_file_empty_file() {
        let test_file = "test_parse_mtab_file_empty_file";
        let file = File::create(test_file).unwrap();
        file.sync_all().unwrap();

        let mounted_drives = parse_mtab_file(test_file).unwrap();
        assert_eq!(mounted_drives.len(), 0);

        std::fs::remove_file(test_file).unwrap();
    }

    #[test]
    fn test_parse_mtab_file_ignore_comments() {
        let test_file = "test_parse_mtab_file_ignore_comments";
        let mut file = File::create(test_file).unwrap();
        file.write_all(b"/dev/sda1 /mnt/ext4 ext4 rw,relatime 0 0\n")
            .unwrap();
        file.write_all(b"# This is a comment\n").unwrap();
        file.write_all(b"tmpfs /mnt/tmpfs tmpfs rw,relatime 0 0\n")
            .unwrap();
        file.sync_all().unwrap();

        let mounted_drives = parse_mtab_file(test_file).unwrap();
        assert_eq!(mounted_drives.len(), 2);

        std::fs::remove_file(test_file).unwrap();
    }
}
