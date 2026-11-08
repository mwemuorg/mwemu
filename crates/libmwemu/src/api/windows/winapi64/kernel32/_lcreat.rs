use crate::emu;
use crate::winapi::helper;
pub fn _lcreat(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let attrib = emu.regs().rdx as u32;
    let path = emu.maps.read_string(path_ptr);
    log_red!(
        emu,
        "kernel32!_lcreat path:'{}' attrib:0x{:x}",
        path,
        attrib
    );
    let handle = helper::handler_create(&format!("file://{}", path));
    emu.regs_mut().rax = handle;
}
