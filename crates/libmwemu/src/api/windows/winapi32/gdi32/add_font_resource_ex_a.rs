use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn AddFontResourceExA(emu: &mut emu::Emu) {
    let name_ptr = arg(emu, 0) as u64;
    let name = emu.maps.read_string(name_ptr);
    log_red!(emu, "gdi32!AddFontResourceExA '{}'", name);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
