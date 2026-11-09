use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualProtect(emu: &mut emu::Emu, addr: u64) {
    memory::virtual_protect(emu, false);
}
