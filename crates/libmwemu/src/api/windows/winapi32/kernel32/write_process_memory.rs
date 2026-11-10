use crate::api::windows::common::memory;
use crate::emu;

pub fn WriteProcessMemory(emu: &mut emu::Emu) {
    memory::process_memory(emu, true);
}
