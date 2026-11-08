use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn EnumTimeFormatsEx(emu: &mut emu::Emu) {
    let callback = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!EnumTimeFormatsEx callback:0x{:x}", callback);
    let _ = emu.maps.is_mapped(arg(emu, 0) as u64);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
