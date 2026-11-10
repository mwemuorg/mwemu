use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualAllocFromApp(emu: &mut emu::Emu) {
    memory::virtual_alloc(emu, false, false);
}
