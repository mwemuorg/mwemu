use serde::{Deserialize, Serialize};

use crate::maps::Maps;

#[derive(Serialize, Deserialize, Default)]
pub struct SerializableMaps {
    maps: Maps,
}

impl From<Maps> for SerializableMaps {
    fn from(maps: Maps) -> Self {
        SerializableMaps { maps }
    }
}

impl From<&Maps> for SerializableMaps {
    fn from(maps: &Maps) -> Self {
        SerializableMaps { maps: maps.clone() }
    }
}

impl From<SerializableMaps> for Maps {
    fn from(serialized: SerializableMaps) -> Self {
        let mut maps = serialized.maps;
        maps.reconnect_shared_views();
        maps
    }
}

impl SerializableMaps {
    pub fn new(maps: Maps) -> Self {
        Self { maps }
    }
}
