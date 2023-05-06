pub enum OperatingSystem {
    Windows,
    Linux,
    MacOS,
    Unknown,
}

pub fn get_operating_system() -> OperatingSystem {
    match std::env::consts::OS {
        "windows" => OperatingSystem::Windows,
        "linux" => OperatingSystem::Linux,
        "macos" => OperatingSystem::MacOS,
        _ => OperatingSystem::Unknown,
    }
}
