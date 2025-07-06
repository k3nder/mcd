use std::collections::HashSet;

use dwldutil::decompress::{DLDecompressionConfig, DecompressionMethod};
use dwldutil::{DLFile, DLHashes};
use thiserror::Error;
use tracing::{debug, warn};

use crate::api::client::{Client, Library, LibraryDownloads, LibraryNatives};
use crate::errors::FetchError;
use crate::os::system::OperatingSystem;
use crate::util::{fill, resolve_rules};

pub struct MavenLibrary {
    pub group_id: String,
    pub artifact_id: String,
    pub version: String,
    pub repository: String,
}

impl MavenLibrary {
    pub fn parse(name: String, repository: String) -> Self {
        let tokens: Vec<&str> = name.split(":").collect();

        MavenLibrary {
            repository,
            group_id: tokens.get(0).unwrap().to_string(),
            artifact_id: tokens.get(1).unwrap().to_string(),
            version: tokens.get(2).unwrap().to_string(),
        }
    }

    pub fn all_url(&self) -> String {
        let group = self.group_id.replace(".", "/");
        format!(
            "{}{}/{}/{}/{}",
            self.repository,
            group,
            self.artifact_id,
            self.version,
            self.cl_name()
        )
    }

    pub fn cl_name(&self) -> String {
        format!("{}-{}.jar", self.artifact_id, self.version)
    }
    pub fn name(&self) -> String {
        format!(
            "{}/{}/{}/{}",
            self.group_id,
            self.artifact_id,
            self.version,
            self.cl_name()
        )
    }
}

pub struct LibsUtil;

impl LibsUtil {
    pub fn new() -> LibsUtil {
        LibsUtil {}
    }
    pub fn fetch(
        &self,
        destination: &str,
        binary_destination: &str,
        client: &Client,
    ) -> Result<(Vec<DLFile>, Vec<String>), FetchError> {
        let libs: &Vec<Library> = client.libraries.as_ref();
        let mut filtered_files: Vec<DLFile> = Vec::new();
        let mut classpath: HashSet<String> = HashSet::new();
        for lib in libs {
            let natives = &&lib.clone().natives;
            if let Some(downloads) = &lib.clone().downloads {
                // artifact
                match Self::filter_artifact(destination, lib, downloads) {
                    Ok(file) => {
                        classpath.insert(file.path.clone());
                        filtered_files.push(file);
                    }
                    Err(e) => warn!("Error downloading artifact: {}", e),
                }
                // classfiers
                debug!("Downloading as classifier...");
                match Self::filter_classifier(destination, binary_destination, natives, downloads) {
                    Ok(file) => filtered_files.push(file),
                    Err(e) => warn!("Error downloading classifier: {}", e),
                }
            } else {
                let lib = MavenLibrary::parse(lib.clone().name, lib.clone().url);
                let file = DLFile::new()
                    .with_url(lib.all_url().as_str())
                    .with_path(format!("{}/{}", destination, lib.name()).as_str());

                classpath.insert(file.path.clone());
                filtered_files.push(file);
            }
        }
        Ok((filtered_files, classpath.into_iter().collect()))
    }

    fn filter_classifier(
        destination: &str,
        binary_destination: &str,
        natives: &Option<LibraryNatives>,
        downloads: &LibraryDownloads,
    ) -> Result<DLFile, ClassifierError> {
        let classifier = &downloads.classifiers;
        if !classifier.is_none() {
            let native_key = Self::get_natives_value(natives);
            debug!("Find native classifier... {}", native_key.as_str());
            if let Some(native) = &classifier.clone().unwrap().get(&native_key) {
                debug!("Download allowed...");
                return Self::add_classifier(destination, binary_destination, native);
            } else {
                return Err(ClassifierError::NoNativeClassifier());
            }
        } else {
            return Err(ClassifierError::NoClassifier());
        }
    }

    fn add_classifier(
        destination: &str,
        binary_destination: &str,
        native: &&crate::api::client::LibraryDownloadsArtifacts,
    ) -> Result<DLFile, ClassifierError> {
        let file = format!("{}/{}", destination, native.path);
        Ok(DLFile::new()
            .with_url(&native.url)
            .with_path(&file)
            .with_size(native.size)
            .with_hashes(DLHashes::new().sha1(native.sha1.clone().as_str()))
            .with_decompression_config(
                DLDecompressionConfig::new(DecompressionMethod::Zip, binary_destination)
                    .with_delete_after(false),
            ))
    }

    fn filter_artifact(
        destination: &str,
        lib: &Library,
        downloads: &LibraryDownloads,
    ) -> Result<DLFile, ArtifactError> {
        if let Some(a) = &downloads.artifact {
            let file = format!("{}/{}", destination, a.path);
            if let Some(r) = &lib.rules {
                if resolve_rules(r) {
                    return Ok(DLFile::new()
                        .with_url(&a.clone().url)
                        .with_path(&file)
                        .with_hashes(DLHashes::new().sha1(a.sha1.clone().as_str()))
                        .with_size(a.clone().size));
                } else {
                    return Err(ArtifactError::NotAllowedByOs());
                }
            } else {
                debug!("Allow by no rules... {}", file);
                return Ok(DLFile::new()
                    .with_url(&a.clone().url)
                    .with_hashes(DLHashes::new().sha1(a.sha1.clone().as_str()))
                    .with_path(&file)
                    .with_size(a.clone().size));
            }
        }
        Err(ArtifactError::NotFound())
    }

    fn get_natives_value(n: &Option<LibraryNatives>) -> String {
        if let Some(n) = n {
            let os = OperatingSystem::detect();
            match os {
                OperatingSystem::Windows => {
                    if let Some(raw) = &n.clone().windows {
                        return fill(raw, "arch".to_string(), "x64".to_string()).to_string();
                    }
                }
                OperatingSystem::Linux => {
                    if let Some(raw) = &n.clone().linux {
                        return fill(raw, "arch".to_string(), "x64".to_string()).to_string();
                    }
                }
                _ => {}
            }
        }
        return String::new();
    }
}
#[derive(Error, Debug)]
pub enum ArtifactError {
    #[error("Not allowed by OS")]
    NotAllowedByOs(),
    #[error("Artifact not found")]
    NotFound(),
}
#[derive(Error, Debug)]
pub enum ClassifierError {
    #[error("No classifier on lib")]
    NoClassifier(),
    #[error("No native classifier")]
    NoNativeClassifier(),
}
