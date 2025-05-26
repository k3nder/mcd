use std::{fs, path::Path};

use dwldutil::{DLFile, DLHashes, Downloader};

use crate::{api::{assets::Assets, client::Client}, errors::FetchError};

pub const BASE_URL: &str = "https://resources.download.minecraft.net";

pub struct ResourceUtil<'a> {
    pub url: &'a str
}
impl<'a> ResourceUtil<'a> {
    pub fn new() -> ResourceUtil<'a> {
        ResourceUtil {
            url: BASE_URL
        }
    }
    pub fn index_of(&self, client: &Client, path: &str) -> Result<Assets, FetchError> {
        let indexes = client.asset_index.clone();
        let dl = Downloader::new().add_file(
                DLFile::new()
                    .with_url(&indexes.url)
                    .with_path(path),
            );
            dl.start();
            let content = std::fs::read_to_string(path)?;
            Ok(serde_json::from_str(&content.as_str())?)
    }
    pub fn fetch(&self, assets: &Assets, destination: &str) -> Result<Vec<DLFile>, FetchError> {
        let obj_path: String = format!("{}/objects", destination);
        //let storage = DLStorage::new(obj_path.as_str());
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
                        .with_hashes(DLHashes::new().sha1(hash));
                        //.with_cas(storage.clone());
                    files.push(file);
                }
                let obj_path = format!("{}/{}/{}", obj_path, hash[0..2].to_owned(), hash.clone());
                if !Path::new(&obj_path).exists() {
                                    let file = DLFile::new()
                                        .with_url(&url)
                                        .with_path(&obj_path)
                                        .with_size(value.size)
                                        .with_hashes(DLHashes::new().sha1(hash));
                                        //.with_cas(storage.clone());
                                    files.push(file);
                                }
            }
            Ok(files)
    }
}
