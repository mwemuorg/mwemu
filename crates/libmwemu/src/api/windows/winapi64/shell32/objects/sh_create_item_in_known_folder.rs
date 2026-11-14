use crate::emu;
use crate::winapi::helper;

pub fn SHCreateItemInKnownFolder(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "shell32!SHCreateItemInKnownFolder arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let handle = helper::handler_create("shcreateiteminknownfolder");
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
