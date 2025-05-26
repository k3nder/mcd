pub mod system {
    #[derive(Debug)]
    pub enum OperatingSystem {
        Linux,
        Windows,
        MacOS,
        Other,
    }

    impl OperatingSystem {
        pub fn detect() -> Self {
            if cfg!(target_os = "linux") {
                OperatingSystem::Linux
            } else if cfg!(target_os = "windows") {
                OperatingSystem::Windows
            } else if cfg!(target_os = "macos") {
                OperatingSystem::MacOS
            } else {
                OperatingSystem::Other
            }
        }
        pub fn name(&self) -> &str {
            match self {
                OperatingSystem::Linux => "linux",
                OperatingSystem::Windows => "windows",
                OperatingSystem::MacOS => "osx",
                _ => "unknow",
            }
        }
    }
}
