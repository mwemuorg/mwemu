use crate::emu;
use crate::winapi::helper;

pub fn LoadAcceleratorsA(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "user32!LoadAcceleratorsA arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let hndl = helper::handler_create("user32://LoadAcceleratorsA");
    emu.regs_mut().rax = hndl;
}
