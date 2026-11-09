use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub(super) fn GetClientRect(emu: &mut emu::Emu) {
    let rect = arg(emu, 1) as u64;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1904);
        emu.maps.write_dword(rect + 12, 1041);
    }
    log_red!(emu, "user32!GetClientRect");
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
