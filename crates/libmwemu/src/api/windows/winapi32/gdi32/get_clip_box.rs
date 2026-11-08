use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetClipBox(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let rect = arg(emu, 1) as u64;
    if rect != 0 {
        emu.maps.write_dword(rect, 0);
        emu.maps.write_dword(rect + 4, 0);
        emu.maps.write_dword(rect + 8, 1920);
        emu.maps.write_dword(rect + 12, 1080);
    }
    log_red!(emu, "gdi32!GetClipBox");
    emu.regs_mut().rax = 1; // SIMPLEREGION
    stdcall_ret(emu, 2);
}
