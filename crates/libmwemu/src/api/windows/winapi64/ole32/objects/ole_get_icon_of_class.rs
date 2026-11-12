use crate::emu;
use crate::winapi::helper;

pub fn OleGetIconOfClass(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "ole32!OleGetIconOfClass arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let hndl = helper::handler_create("ole32://OleGetIconOfClass");
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
