use std::{fs, path::Path};

use dwldutil::{DLFile, DLHashes, Downloader, cas::DLStorage};

use crate::{
    api::{assets::Assets, client::Client},
    errors::FetchError,
    log_indicator,
};

pub const BASE_URL: &str = "https://resources.download.minecraft.net";

pub struct ResourceUtil<'a> {
    pub url: &'a str,
}
impl<'a> ResourceUtil<'a> {
    pub fn new() -> ResourceUtil<'a> {
        ResourceUtil { url: BASE_URL }
    }
    pub fn index_of(&self, client: &Client, path: &str) -> Result<Assets, FetchError> {
        let indexes = client.asset_index.clone();
        let dl = Downloader::<log_indicator::LogIndicator>::new()
            .add_file(DLFile::new().with_url(&indexes.url).with_path(path));
        dl.start();
        let content = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&content.as_str())?)
    }
    pub fn fetch(&self, assets: &Assets, destination: &str) -> Result<Vec<DLFile>, FetchError> {
        let obj_path_str = format!("{}/objects", destination);
        let obj_path = Path::new(&obj_path_str);
        if !obj_path.exists() {
            fs::create_dir_all(obj_path)?;
        }

        let obj_path: String = match obj_path.canonicalize() {
            Ok(path) => path.as_path().to_str().unwrap().to_owned(),
            Err(_) => return Err(FetchError::CanonicalizingError(destination.to_owned())),
        };
        let storage = DLStorage::new(obj_path.as_str());
        let mut files = Vec::new();
        for (key, value) in &assets.objects {
            let hash = &value.hash;
            let block = &hash[..2];
            let url = format!("{}/{}/{}", self.url, block, hash);
            let key_path = key.as_str();
            if !std::path::Path::new(format!("{}/virtual/legacy/", destination).as_str()).exists() {
                fs::create_dir_all(format!("{}/virtual/legacy/", destination))?;
            }
            let path = format!("{}/virtual/legacy/{}", destination, key_path);
            if !Path::new(&path).exists() {
                let file = DLFile::new()
                    .with_url(&url)
                    .with_path(&path)
                    .with_size(value.size)
                    .with_hashes(DLHashes::new().sha1(hash))
                    .with_cas(storage.clone());
                files.push(file);
            }
        }
        Ok(files)
    }
}
