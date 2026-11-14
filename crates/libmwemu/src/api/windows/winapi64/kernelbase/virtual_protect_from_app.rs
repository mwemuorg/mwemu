use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualProtectFromApp(emu: &mut emu::Emu) {
    memory::virtual_protect(emu, false);
}
