use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct AssetsObject {
    pub hash: String,
    pub size: u64
}
#[derive(Deserialize)]
pub struct Assets {
    pub objects: HashMap<String, AssetsObject>
}
