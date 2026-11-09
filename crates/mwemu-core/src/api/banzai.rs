use std::collections::HashMap;

use csv::ReaderBuilder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Banzai {
    api_params: HashMap<String, i32>,
}

impl Default for Banzai {
    fn default() -> Self {
        Self::new()
    }
}

impl Banzai {
    pub fn new() -> Self {
        Self {
            api_params: HashMap::new(),
        }
    }

    /// Load the headerless API/DWORD table; later entries retain overwrite semantics.
    pub fn load(&mut self, path: &str) {
        match ReaderBuilder::new().has_headers(false).from_path(path) {
            Ok(mut reader) => {
                for record in reader.deserialize::<(String, i32)>() {
                    match record {
                        Ok((api, words)) => self.add(&api, words),
                        Err(error) => log::warn!("error parsing banzai.csv record: {}", error),
                    }
                }
            }
            Err(error) => log::error!("banzai.csv not found in maps folder: {}", error),
        }
    }

    pub fn get_params(&self, unimplemented_api: &str) -> i32 {
        if self.api_params.contains_key(unimplemented_api) {
            return self.api_params[unimplemented_api];
        }
        {
            log::warn!("banzai list dont have the params of {}", unimplemented_api);
            -1
        }
    }

    pub fn add(&mut self, name: &str, nparams: i32) {
        self.api_params.insert(name.to_string(), nparams);
    }
}
