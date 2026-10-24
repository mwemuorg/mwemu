use crate::api::windows::common::kernel32::locale_info_value;
use crate::emu;
use crate::winapi::winapi32::kernel32::set_last_error;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    let lctype = arg(emu, 1) as u64;
    let buf = arg(emu, 2) as u64;
    let cch = arg(emu, 3) as usize;
    let value = locale_info_value(lctype).unwrap_or_else(|| {
        log::warn!(
            "{} GetLocaleInfoW unhandled lctype: 0x{:x}",
            emu.pos,
            lctype
        );
        "."
    });
    let required = value.len() + 1; // with the terminator
    log_red!(
        emu,
        "kernel32!GetLocaleInfoW lctype: 0x{:x} ='{}'",
        lctype,
        value
    );
    if cch == 0 {
        emu.regs_mut().rax = required as u64;
    } else if buf == 0 {
        set_last_error(emu, constants::ERROR_INVALID_PARAMETER as u32);
        emu.regs_mut().rax = 0;
    } else if cch < required {
        set_last_error(emu, constants::ERROR_INSUFFICIENT_BUFFER as u32);
        emu.regs_mut().rax = 0;
    } else {
        emu.maps.write_wide_string(buf, value);
        emu.regs_mut().rax = required as u64;
    }
    stdcall_ret(emu, 4);
}
