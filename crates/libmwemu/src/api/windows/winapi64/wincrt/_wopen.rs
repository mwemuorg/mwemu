use crate::emu;
use crate::winapi::helper;

pub fn _wopen(emu: &mut emu::Emu) {
    let path_ptr = emu.regs().rcx;
    let oflag = emu.regs().rdx as u32;
    let path = emu.maps.read_wide_string(path_ptr);
    let hndl = helper::handler_create(&path);
    log_red!(
        emu,
        "wincrt!_wopen '{}' flags:0x{:x} => 0x{:x}",
        path,
        oflag,
        hndl
    );
    emu.regs_mut().rax = hndl;
}
