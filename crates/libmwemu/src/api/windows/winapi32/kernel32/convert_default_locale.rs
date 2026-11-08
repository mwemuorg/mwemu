use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ConvertDefaultLocale(emu: &mut emu::Emu) {
    let locale = arg(emu, 0) as u64;

    log_red!(emu, "kernel32!ConvertDefaultLocale locale:0x{:x}", locale);
    let src = arg(emu, 0) as u64;
    if src != 0 && emu.maps.is_mapped(src) {
        let _input = emu.maps.read_string(src);
    }

    let result = match locale {
        0x0800 => 0x0409,
        0x0400 => 0x0409,
        0x0C00 => 0x0409,
        other => other,
    };
    emu.set_last_error(0);
    emu.regs_mut().rax = result;
    stdcall_ret(emu, 1);
}
