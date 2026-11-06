
use crate::emu;
use crate::winapi::helper;
pub fn IsNLSDefinedString(emu: &mut emu::Emu) {
    let func = emu.regs().rcx;
    let flags = emu.regs().rdx;
    let lp_version_info = emu.regs().r8;
    let lp_string = emu.regs().r9;
    log_red!(
        emu,
        "kernel32!IsNLSDefinedString func:{} flags:0x{:x} verInfo:0x{:x} str:0x{:x}",
        func,
        flags,
        lp_version_info,
        lp_string
    );
    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
