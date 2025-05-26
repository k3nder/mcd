use serde::Deserialize;

#[derive(Deserialize)]
pub struct Manifest {
    pub latest: Latest,
    pub versions: Vec<Version>
}
#[derive(Deserialize)]
pub struct Latest {
    pub release: String,
    pub snapshot: String
}
impl Manifest {
    pub fn get(&self, version: &str) -> Option<&Version> {
        self.versions.iter().find(|f| f.id.eq(version))
    }
}
#[derive(Deserialize, Debug)]
pub struct Version {
    pub id: String,
    #[serde(alias = "type")]
    pub version_type: String,
    pub url: String,
    pub time: String,
    #[serde(alias = "releaseTime")]
    pub release_time: String,
}
