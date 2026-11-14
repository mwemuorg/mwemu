use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetSecurityInfo(emu: &mut emu::Emu) {
    let p0 = WinApiAbi::arg(emu, 0);
    let p1 = WinApiAbi::arg(emu, 1);
    let p2 = WinApiAbi::arg(emu, 2);
    let p3 = WinApiAbi::arg(emu, 3);
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
    stdcall_ret(emu, 7);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
