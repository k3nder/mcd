use std::path::Path;

use dwldutil::{DLFile, DLHashes};

use crate::{api::client::{Client, JarFile}, errors::FetchError};

pub enum Type {
    Server,
    Client
}
impl Type {
    pub fn file(&self, client: &Client) -> JarFile {
        match self {
            Type::Client => return client.downloads.client.clone(),
            Type::Server => return client.downloads.server.clone(),
        }
    }
}

pub fn fetch(client: &Client, path: &str, typ: Type) -> Result<DLFile, FetchError> {
    if Path::new(path).exists() { return Err(FetchError::PathAlredyExist(path.to_owned())) }
    Ok(typ.file(client).dl().with_path(path))
}
pub fn fetch_client(client: &Client, path: &str) -> Result<DLFile, FetchError> {
    fetch(client, path, Type::Client)
}
pub fn fetch_server(client: &Client, path: &str) -> Result<DLFile, FetchError> {
    fetch(client, path, Type::Server)
}
trait ToDownload {
    fn dl(&self) -> DLFile;
}
impl ToDownload for JarFile {
    fn dl(&self) -> DLFile {
        DLFile::new()
            .with_url(&self.url)
            .with_size(self.size)
            .with_hashes(DLHashes::new().sha1(&self.sha1))
    }
}
