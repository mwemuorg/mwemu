use std::collections::HashMap;

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

    pub fn get_params(&self, unimplemented_api: &str) -> i32 {
        if self.api_params.contains_key(unimplemented_api) {
            return self.api_params[unimplemented_api];
        }
        panic!("banzai list dont have the params of {}", unimplemented_api);
    }

    pub fn add(&mut self, name: &str, nparams: i32) {
        self.api_params.insert(name.to_string(), nparams);
    }
}
