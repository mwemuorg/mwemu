use crate::emu;
use crate::winapi::helper;

pub fn RegisterServiceCtrlHandlerW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!RegisterServiceCtrlHandlerW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerW");
    emu.regs_mut().rax = hndl;
}
