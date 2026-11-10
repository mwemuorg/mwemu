use crate::api::windows::common::memory;
use crate::emu;

pub fn VirtualQueryEx_api(emu: &mut emu::Emu, addr: u64) {
    memory::virtual_query(emu, true);
}
