use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualFreeEx(emu: &mut emu::Emu) {
    memory::virtual_free(emu, true);
}
