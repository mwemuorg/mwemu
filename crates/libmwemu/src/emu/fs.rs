use std::collections::BTreeMap;

use crate::{emu::Emu, threading::context::ArchThreadState};

impl Emu {
    pub fn fs(&self) -> &BTreeMap<u64, u64> {
        match &self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { fs, .. } => fs,
            _ => panic!("fs() called on aarch64 emu"),
        }
    }

    pub fn fs_mut(&mut self) -> &mut BTreeMap<u64, u64> {
        match &mut self.threads[self.current_thread_id].arch {
            ArchThreadState::X86 { fs, .. } => fs,
            _ => panic!("fs_mut() called on aarch64 emu"),
        }
    }
}
