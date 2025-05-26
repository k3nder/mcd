use serde::Deserialize;
use std::{collections::HashMap};

use crate::os::system::OperatingSystem;

#[derive(Deserialize, Debug, Clone)]
pub struct Client {
    #[serde(alias = "minecraftArguments")]
    pub minecraft_arguments: Option<String>,
    #[serde()]
    pub arguments: Option<Arguments>,
    #[serde(alias = "inheritsFrom")]
    pub inherits_from: Option<String>,
    #[serde(default, alias = "assetIndex")]
    pub asset_index: AssetsIndex,
    #[serde(default)]
    pub assets: String,
    #[serde(default, alias = "compilanceLevel")]
    pub compilance_level: u32,
    #[serde(default)]
    pub downloads: ClientDownloads,
    pub id: String,
    #[serde(default, alias = "javaVersion")]
    pub java_version: JavaVersion,
    pub libraries: Vec<Library>,
    #[serde(alias = "mainClass")]
    pub main_class: String,
    #[serde(alias = "minimumLauncherVersion")]
    pub minimum_launcher_version: Option<u32>,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
    pub time: String,
    #[serde(alias = "type")]
    pub version_type: String,
    #[serde(default)]
    pub logging: Option<LogSettings>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct Arguments {
    pub game: Vec<ArgumentValue>,
    pub jvm: Vec<ArgumentValue>
}
#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
pub enum ArgumentValue {
    Plain(String),
    Complex(ComplexArgument)
}

#[derive(Deserialize, Debug, Clone)]
pub struct ComplexArgument {
    pub rules: Vec<Rule>,
    pub value: ValueField,
}
#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ValueField {
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct AssetsIndex {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(alias = "totalSize")]
    pub total_size: u64,
    pub url: String,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct ClientDownloads {
    pub client: JarFile,
    #[serde(default)]
    pub client_mappings: JarFile,
    #[serde(skip)]
    pub server: JarFile,
    #[serde(default)]
    pub server_mappings: JarFile,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct JarFile {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}
#[derive(Deserialize, Debug, Clone, Default)]
pub struct JavaVersion {
    pub component: String,
    #[serde(alias = "majorVersion")]
    pub major_version: u32,
}
pub fn default_vec_library_rules() -> Vec<Rule> {
    vec![]
}
#[derive(Debug, Deserialize, Clone)]
pub struct LibraryDownloads {
    pub(crate) artifact: Option<LibraryDownloadsArtifacts>,
    pub(crate) classifiers: Option<HashMap<String, LibraryDownloadsArtifacts>>,
}

#[derive(Debug, Deserialize, Clone)]

pub struct LibraryDownloadsArtifacts {
    pub(crate) path: String,
    pub(crate) sha1: String,
    pub size: u64,
    pub(crate) url: String,
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct RuleOs {
    pub(crate) name: Option<String>,
    pub(crate) arch: Option<String>,
    pub(crate) version: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Rule {
    pub(crate) action: String,
    pub(crate) features: Option<HashMap<String, bool>>,
    pub(crate) os: Option<RuleOs>,
}

impl Rule {
    pub fn allow(&self, os: &OperatingSystem) -> bool {
        if let Some(jos) = self.os.as_ref() {
            if let Some(name) = jos.name.as_ref() {
                return (self.action.eq("allow") && self.os.is_none())
                || (self.action.eq("allow") && name.eq(os.name()))
                || (!self.action.eq("allow") && !name.eq(os.name()))
            }
        } else if self.action.eq("allow") {
            return true;
        }
        false
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct Library {
    pub(crate) downloads: Option<LibraryDownloads>,
    pub(crate) name: String,
    pub(crate) rules: Option<Vec<Rule>>,
    #[serde(default)]
    pub(crate) url: String,
    pub natives: Option<LibraryNatives>,
    #[serde(default)]
    md5: String,
    #[serde(default)]
    sha1: String,
    #[serde(default)]
    sha256: String,
    #[serde(default)]
    sha521: String,
    #[serde(default)]
    size: usize,
    pub(crate) extract: Option<LibraryExtract>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct LibraryExtract {
    exclude: Vec<String>,
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LibraryNatives {
    pub osx: Option<String>,
    pub linux: Option<String>,
    pub windows: Option<String>,
}
#[derive(Deserialize, Default, Debug, Clone)]
#[serde(default)]
pub struct LogSettings {
    pub client: LogSettingsClient,
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LogSettingsClient {
    pub argument: String,
    pub file: LogSettingsClientFile,
    #[serde(alias = "type")]
    pub client_type: String,
}
#[derive(Deserialize, Debug, Default, Clone)]
pub struct LogSettingsClientFile {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    pub url: String,
}
impl Client {
    pub fn java(&self) -> usize {
        self.java_version.major_version as usize
    }
}
