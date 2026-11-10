use crate::emu;
use crate::winapi::helper;

pub fn RegisterServiceCtrlHandlerExA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "advapi32!RegisterServiceCtrlHandlerExA arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerExA");
    emu.regs_mut().rax = hndl;
}

pub fn RegisterServiceCtrlHandlerExW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "advapi32!RegisterServiceCtrlHandlerExW arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    let hndl = helper::handler_create("advapi32://RegisterServiceCtrlHandlerExW");
    emu.regs_mut().rax = hndl;
}
