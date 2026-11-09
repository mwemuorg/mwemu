use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualAlloc(emu: &mut emu::Emu) {
    memory::virtual_alloc(emu, false, false);
}
