use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct MountedDrive {
    pub device: String,
    pub mount_point: String,
    pub file_system: String,
    pub options: String,
}

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
        let file_path = "test_mtab";
        let mut file = File::create(file_path).unwrap();
        file.write_all(b"/dev/sda1 /mnt/ext4 ext4 rw,relatime 0 0\n").unwrap();
        file.write_all(b"tmpfs /mnt/tmpfs tmpfs rw,relatime 0 0\n").unwrap();
        file.sync_all().unwrap();

        let mounted_drives = parse_mtab_file(file_path).unwrap();
        assert_eq!(mounted_drives.len(), 2);

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_parse_mtab_file_nonexistent_file() {
        let file_path = "nonexistent_file";
        let result = parse_mtab_file(file_path);
        assert!(result.is_err()); // Assert that the result is an error
    }

    #[test]
    fn test_parse_mtab_file_empty_file() {
        let file_path = "empty_mtab";
        let file = File::create(file_path).unwrap();
        file.sync_all().unwrap();

        let mounted_drives = parse_mtab_file(file_path).unwrap();
        assert_eq!(mounted_drives.len(), 0);

        std::fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_parse_mtab_file_ignore_comments() -> Result<(), Box<dyn std::error::Error>> {
        let file_path = "test_mtab";
        let mut file = File::create(file_path)?;
        file.write_all(b"/dev/sda1 /mnt/ext4 ext4 rw,relatime 0 0\n")?;
        file.write_all(b"# This is a comment\n")?;
        file.write_all(b"tmpfs /mnt/tmpfs tmpfs rw,relatime 0 0\n")?;
        file.sync_all()?;

        let mounted_drives = parse_mtab_file(file_path)?;
        assert_eq!(mounted_drives.len(), 2);

        std::fs::remove_file(file_path)?;

        Ok(())
    }
}
