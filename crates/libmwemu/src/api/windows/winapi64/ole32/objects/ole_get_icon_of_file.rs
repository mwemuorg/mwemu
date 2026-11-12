use crate::emu;
use crate::winapi::helper;

pub fn OleGetIconOfFile(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "ole32!OleGetIconOfFile arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let hndl = helper::handler_create("ole32://OleGetIconOfFile");
    emu.set_last_error(0);
    emu.regs_mut().rax = hndl;
}
