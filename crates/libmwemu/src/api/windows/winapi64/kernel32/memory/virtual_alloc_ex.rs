use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualAllocEx(emu: &mut emu::Emu) {
    memory::virtual_alloc(emu, true, false);
}
