use std::error::Error;

/// Error type returned by emulation operations (load, run, memory access).
#[derive(Debug)]
pub struct MwemuError {
    pub message: String,
}

impl MwemuError {
    /// Create a new error with the given message.
    pub fn new(message: &str) -> MwemuError {
        MwemuError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for MwemuError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "MWEMU Error: {}", self.message)
    }
}

impl Error for MwemuError {}
