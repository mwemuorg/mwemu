use crate::emu;
use crate::winapi::helper;

pub fn DeleteMenu(emu: &mut emu::Emu) {
    let hmenu = emu.regs().rcx;
    let u_position = emu.regs().rdx as u32;
    let u_flags = emu.regs().r8 as u32;
    log_red!(
        emu,
        "user32!DeleteMenu menu:0x{:x} pos:{} flags:0x{:x}",
        hmenu,
        u_position,
        u_flags
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.regs_mut().rax = 1;
}
