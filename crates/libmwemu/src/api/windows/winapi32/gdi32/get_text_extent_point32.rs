use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetTextExtentPoint32A(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let len = arg(emu, 2);
    let size_ptr = arg(emu, 3) as u64;
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, len * 7); // cx
        emu.maps.write_dword(size_ptr + 4, 16); // cy
    }
    log_red!(emu, "gdi32!GetTextExtentPoint32A len:{}", len);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}

pub fn GetTextExtentPoint32W(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let len = arg(emu, 2);
    let size_ptr = arg(emu, 3) as u64;
    if size_ptr != 0 {
        emu.maps.write_dword(size_ptr, len * 7);
        emu.maps.write_dword(size_ptr + 4, 16);
    }
    log_red!(emu, "gdi32!GetTextExtentPoint32W len:{}", len);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 4);
}
