use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetSecurityInfo(emu: &mut emu::Emu) {
    let p0 = emu.regs().rcx;
    let p1 = emu.regs().rdx;
    let p2 = emu.regs().r8;
    let p3 = emu.regs().r9;
    let p4 = WinApiAbi::arg(emu, 4);
    let p5 = WinApiAbi::arg(emu, 5);
    let p6 = WinApiAbi::arg(emu, 6);
    let p7 = WinApiAbi::arg(emu, 7);
    log_red!(
        emu,
        "advapi32!SetSecurityInfo p1:0x{:x} p2:0x{:x} p3:0x{:x}",
        p1,
        p2,
        p3
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
