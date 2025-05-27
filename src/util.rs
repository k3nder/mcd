use std::collections::HashMap;

use log::debug;

use crate::{api::client::Rule, errors::FillingError, os::system::OperatingSystem};

pub struct FillingUtil {
    pub data: HashMap<String, String>
}
impl FillingUtil {
    pub fn new() -> Self {
        Self {
            data: HashMap::new()
        }
    }
    pub fn with_data(mut self, data: HashMap<String, String>) -> Self {
        self.data = data;
        self
    }
    pub fn insert(&mut self, k: &str, v: String) {
        self.data.insert(k.to_owned(), v);
    }
    pub fn fill(&self, text: String) -> Result<String, FillingError> {
        if !text.contains("${") {
            return Err(FillingError::NoPattron());
        }
        let mut text = text;
        loop {
            if !text.contains("${") {
                return Ok(text);
            }
            let key_start_index = text.find("{").unwrap()+1;
            let key_end_index = text.find("}").unwrap();
            let key: &str = &text[key_start_index..key_end_index];
            debug!("KEY FILL {}", key);
            if !self.data.contains_key(key) {
                return Err(FillingError::NoKeyFound(key.to_owned()))
            }
            text = text.replace(format!("${{{}}}", key).as_str(), self.data.get(key).unwrap());
        }
    }
}

pub fn fill(s: &String, k: String, v: String) -> String {
    if !s.contains(k.as_str()) {
        return s.to_string();
    }
    let ss = s.replace(format!("${k}").as_str(), v.as_str());
    ss.clone()
}

pub fn resolve_rules(rules: &[Rule]) -> bool {
    let sys = OperatingSystem::detect();
    debug!("Finding out OS... {:?}", sys);
    for rule in rules {
        debug!("check... {:?}", rule);
        if !rule.allow(&sys) {
            return false;
        }
    }
    true
}
pub fn resolve_rules_feat(rules: &[Rule], options: &HashMap<String, bool>) -> bool {
    let sys = OperatingSystem::detect();
        debug!("Finding out OS... {:?}", sys);
        for rule in rules {
            debug!("check... {:?}", rule);
            if !rule.allow(&sys) {
                return false;
            }
            if let Some(feat) = rule.features.as_ref() {
                for (k,v) in feat {
                    if !options.contains_key(k) {
                        return false;
                    }
                    if options.get(k).unwrap() != v {
                        return false;
                    }
                }
            }
        }
        true
}
