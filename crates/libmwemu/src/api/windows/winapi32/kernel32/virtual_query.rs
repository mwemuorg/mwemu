use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualQuery(emu: &mut emu::Emu) {
    memory::virtual_query(emu, false);
}
