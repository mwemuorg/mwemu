use crate::emu;
use crate::winapi::helper;
pub fn _lopen(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let mode = emu.regs().rdx as u32;
    let path = emu.maps.read_string(path_ptr);
    log_red!(emu, "kernel32!_lopen path:'{}' mode:0x{:x}", path, mode);
    let handle = helper::handler_create(&format!("file://{}", path));
    emu.regs_mut().rax = handle;
}
