use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualProtectEx(emu: &mut emu::Emu) {
    memory::virtual_protect(emu, true);
}
