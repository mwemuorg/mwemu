use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualFree(emu: &mut emu::Emu) {
    memory::virtual_free(emu, false);
}
