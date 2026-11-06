
use crate::emu;
use crate::winapi::helper;
pub fn BaseCheckElevation(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    let param2 = emu.regs().rdx;
    let param3 = emu.regs().r8;
    let param4 = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!BaseCheckElevation param1:0x{:x} param2:0x{:x} param3:0x{:x} param4:0x{:x}",
        param1,
        param2,
        param3,
        param4
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
