use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_FAKE_SYNC_HANDLE: AtomicU64 = AtomicU64::new(0x1000);

pub fn next_handle() -> u64 {
    NEXT_FAKE_SYNC_HANDLE.fetch_add(4, Ordering::Relaxed)
}
