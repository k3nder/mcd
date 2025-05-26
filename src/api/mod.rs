use std::{fs::File, io::Read, path::Path};

use client::Client;
use dwldutil::{DLFile, Downloader};
use manifest::{Manifest, Version};
use thiserror::Error;

#[warn(dead_code)]
pub mod client;
pub mod manifest;
pub mod assets;

pub const MANIFEST_URL: &str = "https://launchermeta.mojang.com/mc/game/version_manifest.json";

pub struct ApiClientUtil {
    pub manifest: Manifest,
}
impl ApiClientUtil {
    pub fn new(manifest_path: &str) -> Result<Self, ApiClientError> {
        if !Path::new(manifest_path).exists() {
            let file = DLFile::new()
                .with_url(MANIFEST_URL)
                .with_path(manifest_path);
            let downloader = Downloader::new().add_file(file);
            downloader.start();
        }
        let mut str = String::new();
        File::open(manifest_path)?.read_to_string(&mut str)?;
        let manifest: Manifest = serde_json::from_str(&str)?;
        Ok(ApiClientUtil { manifest })
    }
    pub fn fetch(&self, version: &str, path: &str) -> Result<Client, ApiClientError> {
        if Path::new(path).exists() {
            return Ok(Self::rl(path)?)
        }
        let version = if let Some(version) = self.manifest.get(version) {
            version
        } else {
            return Err(ApiClientError::VersionNotExist(version.to_owned()));
        };
        Self::request(version, path);
        Ok(Self::rl(path)?)
    }
    pub fn load(&self, path: &str) -> Result<Client, ApiClientError> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut client: Client = serde_json::from_str(content.as_str())?;
        if let Some(inherits_from) = client.inherits_from {
            let mut comb = self.fetch(&inherits_from, path)?;
            comb.libraries.append(&mut client.libraries);
            comb.id = client.id;
            comb.main_class = client.main_class;
            return Ok(comb);
        }
        Ok(client)
    }
    fn request(version: &Version, path: &str) {
        let file = DLFile::new().with_url(&version.url).with_path(path);
        Downloader::new().add_file(file).start();
    }
    fn rl(path: &str) -> Result<Client, std::io::Error> {
        let mut str = String::new();
        File::open(path)?.read_to_string(&mut str)?;
        Ok(serde_json::from_str(&str)?)
    }
}
#[derive(Error, Debug)]
pub enum ApiClientError {
    #[error("IOError")]
    IOError(#[from] std::io::Error),
    #[error("Deserialize error")]
    JsonError(#[from] serde_json::Error),
    #[error("File exists {0}")]
    FileExist(String),
    #[error("Version {0} not exist")]
    VersionNotExist(String),
}
