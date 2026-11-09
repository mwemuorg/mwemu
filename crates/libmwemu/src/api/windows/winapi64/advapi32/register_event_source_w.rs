use crate::emu;
use crate::winapi::helper;

pub fn RegisterEventSourceW(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "advapi32!RegisterEventSourceW arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let hndl = helper::handler_create("advapi32://RegisterEventSourceW");
    emu.regs_mut().rax = hndl;
}
