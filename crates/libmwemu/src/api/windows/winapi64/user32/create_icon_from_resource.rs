use crate::emu;
use crate::winapi::helper;

pub fn CreateIconFromResource(emu: &mut emu::Emu) {
    let presbits = emu.regs().rcx;
    let dw_res_size = emu.regs().rdx as u32;
    let is_icon = emu.regs().r8 as u32;
    let dw_ver = emu.regs().r9 as u32;
    log_red!(
        emu,
        "user32!CreateIconFromResource data:0x{:x} size:{} icon:{} ver:0x{:x}",
        presbits,
        dw_res_size,
        is_icon,
        dw_ver
    );
    let hndl = helper::handler_create("user32://CreateIconFromResource");
    emu.regs_mut().rax = hndl;
}
