use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetCursorPos(emu: &mut emu::Emu) {
    let pt = arg(emu, 0) as u64;
    if pt != 0 {
        emu.maps.write_dword(pt, 960);
        emu.maps.write_dword(pt + 4, 540);
    }
    log_red!(emu, "user32!GetCursorPos");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
