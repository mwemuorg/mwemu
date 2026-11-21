use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub(crate) struct SnapshotModule {
    pub base: u64,
    pub size: u32,
    pub name: String,
    pub path: String,
}

/// Toolhelp captures guest state at creation; each list has an independent cursor.
#[derive(Clone, Default, Serialize, Deserialize)]
pub(crate) struct ToolhelpSnapshot {
    pub process: Option<(u32, String)>,
    pub process_threads: u32,
    pub modules: Vec<SnapshotModule>,
    pub threads: Vec<u32>,
    pub owner: u32,
    pub process_next: usize,
    pub module_next: usize,
    pub thread_next: usize,
}
