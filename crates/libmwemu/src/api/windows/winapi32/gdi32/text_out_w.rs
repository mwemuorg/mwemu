use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn TextOutW(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let x = arg(emu, 1);
    let y = arg(emu, 2);
    let text_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_wide_string(text_ptr);
    log_red!(emu, "gdi32!TextOutW '{}'", text);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
