use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn SetViewportOrgEx(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let prev = arg(emu, 3) as u64;
    if prev != 0 {
        emu.maps.write_dword(prev, 0);
        emu.maps.write_dword(prev + 4, 0);
    }
    log_red!(emu, "gdi32!SetViewportOrgEx x:{} y:{}", x, y);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
