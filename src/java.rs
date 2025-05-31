use std::{collections::HashMap, path::Path};

use dwldutil::{
    DLFile, DLHashes,
    decompress::{DLDecompressionConfig, DecompressionMethod},
};
use log::debug;

use crate::{errors::FetchError, os::system::OperatingSystem};

pub struct JavaUtil<'a> {
    pub versions: HashMap<&'a str, (&'a str, &'a str, &'a str, usize, DecompressionMethod)>,
    distribution: &'a str,
}
impl<'a> JavaUtil<'a> {
    pub fn new() -> Self {
        let mut versions = HashMap::new();
        versions.insert("adopt-windows-21", ("jdk-21.0.7+6-jre","https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.7%2B6/OpenJDK21U-jre_x64_windows_hotspot_21.0.7_6.zip", "b2850a96293048ed3020f8bfca2d92a785ae9bf80c7d96bbfe3ec4ccf45aef98", 48875360, DecompressionMethod::Zip));
        versions.insert("adopt-linux-21", ("jdk-21.0.7+6-jre", "https://github.com/adoptium/temurin21-binaries/releases/download/jdk-21.0.7%2B6/OpenJDK21U-jre_x64_linux_hotspot_21.0.7_6.tar.gz", "6d48379e00d47e6fdd417e96421e973898ac90765ea8ff2d09ae0af6d5d6a1c6", 51863597, DecompressionMethod::TarGzip));
        versions.insert("adopt-windows-17", ("jdk-17.0.9+9-jre", "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.9%2B9.1/OpenJDK17U-jre_x64_windows_hotspot_17.0.9_9.zip", "6c491d6f8c28c6f451f08110a30348696a04b009f8c58592191046e0fab1477b", 43447242, DecompressionMethod::Zip));
        versions.insert("adopt-linux-17", ("jdk-17.0.9+9-jre", "https://github.com/adoptium/temurin17-binaries/releases/download/jdk-17.0.9%2B9/OpenJDK17U-jre_x64_linux_hotspot_17.0.9_9.tar.gz", "c37f729200b572884b8f8e157852c739be728d61d9a1da0f920104876d324733", 46280224, DecompressionMethod::TarGzip));
        versions.insert("adopt-windows-8", ("jdk8u452-b09-jre","https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u452-b09/OpenJDK8U-jre_x64_windows_hotspot_8u452b09.zip", "802b1277505308290b6f00d8addde93e537d559cea1c826752d0cc46e7b58a5f", 40652901, DecompressionMethod::Zip));
        versions.insert("adopt-linux-8", ("jdk8u452-b09-jre","https://github.com/adoptium/temurin8-binaries/releases/download/jdk8u452-b09/OpenJDK8U-jre_x64_linux_hotspot_8u452b09.tar.gz", "0c76f94e1b400a4da932a3f581b0788af2101819083184f40a6c76ac9b97081f", 41420532, DecompressionMethod::TarGzip));
        let distribution = "adopt";
        JavaUtil {
            versions,
            distribution,
        }
    }
    pub fn set_distribution(&mut self, distribution: &'a str) {
        self.distribution = distribution;
    }
    pub fn fetch(&self, version: usize, path: &str) -> Result<DLFile, FetchError> {
        let os = OperatingSystem::detect();
        if let OperatingSystem::MacOS = os {
            return Err(FetchError::OsUnsupported());
        } else if let OperatingSystem::Other = os {
            return Err(FetchError::OsUnsupported());
        }
        if Path::new(path).exists() {
            return Err(FetchError::PathAlredyExist(path.to_owned()));
        }

        debug!("TARGET OG {}", os.name());
        let key = self.find_key(version, os);
        debug!("SEARCH KEY {}", key);
        if !self.versions.contains_key(&key.as_str()) {
            return Err(FetchError::UrlNotFound(version.to_string()));
        }
        let (_, url, sha256, size, compression) = self.versions.get(key.as_str()).unwrap();
        Ok(
            DLFile::new()
                .with_url(url)
                .with_path(&format!("{}.tmp", path))
                .with_hashes(DLHashes::new().sha256(sha256))
                .with_size(*size as u64)
                .with_decompression_config(
                    DLDecompressionConfig::new(
                        {
                            match compression {
                                DecompressionMethod::TarGzip => DecompressionMethod::TarGzip,
                                DecompressionMethod::Zip => DecompressionMethod::Zip,
                            }
                        },
                        path,
                    )
                    .delete_after(),
                ),
        )
    }
    pub fn id_of(&self, version: usize) -> Option<String> {
        let key = self.find_key(version, OperatingSystem::detect());
        if !self.versions.contains_key(key.clone().as_str()) {
            return None;
        }
        Some(self.versions.get(key.clone().as_str()).unwrap().0.to_owned())
    }
    fn find_key(&self, version: usize, os: OperatingSystem) -> String {
        let key = format!(
            "{}-{}-{}",
            self.distribution,
            os.name(),
            version.to_string()
        );
        key
    }
}
