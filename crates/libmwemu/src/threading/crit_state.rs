use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct CritState {
    pub owner_tid: Option<u64>,    // Thread ID currently owning the lock
    pub recursion_count: usize,    // Recursive enter count
    pub wait_queue: VecDeque<u64>, // Waiting thread IDs
}
