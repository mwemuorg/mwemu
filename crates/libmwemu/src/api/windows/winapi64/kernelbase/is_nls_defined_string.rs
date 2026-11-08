use crate::emu;
use crate::winapi::helper;

pub fn IsNLSDefinedString(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    let arg4 = emu.regs().r9;
    let arg5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(
        emu,
        "kernelbase!IsNLSDefinedString arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );

    emu.set_last_error(0);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
}
