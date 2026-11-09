use std::collections::{HashMap, VecDeque};

use super::crit_state::CritState;

pub struct GlobalLocks {
    pub crit_map: HashMap<u64, CritState>, // key = pointer to CRITICAL_SECTION
}

impl Default for GlobalLocks {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalLocks {
    pub fn new() -> Self {
        Self {
            crit_map: HashMap::new(),
        }
    }

    pub fn enter(&mut self, cs_ptr: u64, tid: u64) -> bool {
        let cs_state = self.crit_map.entry(cs_ptr).or_insert_with(|| CritState {
            owner_tid: None,
            recursion_count: 0,
            wait_queue: VecDeque::new(),
        });

        match cs_state.owner_tid {
            None => {
                // Lock is free
                cs_state.owner_tid = Some(tid);
                cs_state.recursion_count = 1;
                true // acquired immediately
            }
            Some(owner) if owner == tid => {
                // Recursive enter
                cs_state.recursion_count += 1;
                true // acquired immediately
            }
            Some(_) => {
                // Lock held by another thread — block this one
                if !cs_state.wait_queue.contains(&tid) {
                    cs_state.wait_queue.push_back(tid);
                }
                false // did not acquire (must wait)
            }
        }
    }

    pub fn leave(&mut self, cs_ptr: u64, tid: u64) -> Option<u64> {
        if let Some(cs_state) = self.crit_map.get_mut(&cs_ptr) {
            if cs_state.owner_tid == Some(tid) {
                if cs_state.recursion_count > 1 {
                    cs_state.recursion_count -= 1;
                    None
                } else {
                    // Fully releasing the lock
                    if let Some(next_tid) = cs_state.wait_queue.pop_front() {
                        // Transfer ownership to next waiting thread
                        cs_state.owner_tid = Some(next_tid);
                        cs_state.recursion_count = 1;
                        Some(next_tid) // Return the thread ID to wake up
                    } else {
                        cs_state.owner_tid = None;
                        cs_state.recursion_count = 0;
                        None
                    }
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
