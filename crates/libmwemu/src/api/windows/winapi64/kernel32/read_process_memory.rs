use crate::api::windows::common::memory;
use crate::emu;

pub fn ReadProcessMemory(emu: &mut emu::Emu) {
    memory::process_memory(emu, false);
}
