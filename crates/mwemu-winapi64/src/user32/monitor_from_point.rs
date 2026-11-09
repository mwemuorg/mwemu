use crate::emu;
use crate::winapi::helper;

pub fn MonitorFromPoint(emu: &mut emu::Emu) {
    let pt_x = emu.regs().rcx as i32;
    let pt_y = emu.regs().rdx as i32;
    let flags = emu.regs().r8 as u32;
    let hmon = helper::handler_create("monitor");
    log_red!(
        emu,
        "user32!MonitorFromPoint ({},{}) flags:0x{:x} => 0x{:x}",
        pt_x,
        pt_y,
        flags,
        hmon
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = hmon;
}
