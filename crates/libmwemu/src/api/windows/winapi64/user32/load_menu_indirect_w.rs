use crate::emu;
use crate::winapi::helper;

pub fn LoadMenuIndirectW(emu: &mut emu::Emu) {
    let template = emu.regs().rcx;
    log_red!(emu, "user32!LoadMenuIndirectW tmpl:0x{:x}", template);
    let hndl = helper::handler_create("user32://menu");
    emu.regs_mut().rax = hndl;
}
