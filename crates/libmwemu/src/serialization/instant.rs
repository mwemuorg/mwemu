use std::time::Instant;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SerializableInstant {
    // Store as duration since UNIX_EPOCH
    timestamp: u64,
}

impl From<Instant> for SerializableInstant {
    fn from(instant: Instant) -> Self {
        // Convert Instant to duration since UNIX_EPOCH
        let duration = instant.duration_since(Instant::now())
            + SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        SerializableInstant {
            timestamp: duration.as_secs(),
        }
    }
}

impl SerializableInstant {
    pub fn to_instant(&self) -> Instant {
        // Convert back to Instant
        let system_now = SystemTime::now();
        let instant_now = Instant::now();

        instant_now
            - system_now
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .saturating_sub(std::time::Duration::from_secs(self.timestamp))
    }
}
