use thiserror::Error;

#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Path '{0}' alredy exist")]
    PathAlredyExist(String),
    #[error("Unsupported Os")]
    OsUnsupported(),
    #[error("Url of {0} not found")]
    UrlNotFound(String),
    #[error("IO error")]
    IOError(#[from] std::io::Error),
    #[error("Deserialization Error")]
    SerdeError(#[from] serde_json::Error),
    #[error("Error canonicalizing path {0}")]
    CanonicalizingError(String),
}
#[derive(Error, Debug)]
pub enum FillingError {
    #[error("Text hasn't pattron")]
    NoPattron(),
    #[error("Malformed text")]
    Malformed(),
    #[error("No key found {0}")]
    NoKeyFound(String)
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
    NoNativeClassifier()
}
