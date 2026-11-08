use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn GetDeviceCaps(emu: &mut emu::Emu) {
    let hdc = arg(emu, 0);
    let index = arg(emu, 1) as i32;
    let result: u64 = match index {
        8 => 1920,   // HORZRES
        10 => 1080,  // VERTRES
        12 => 96,    // LOGPIXELSX
        14 => 32,    // BITSPIXEL
        88 => 96,    // LOGPIXELSX
        90 => 96,    // LOGPIXELSY
        117 => 1080, // DESKTOPVERTRES
        118 => 1920, // DESKTOPHORZRES
        _ => 0,
    };
    log_red!(emu, "gdi32!GetDeviceCaps index:{} = {}", index, result);
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 2);
}
