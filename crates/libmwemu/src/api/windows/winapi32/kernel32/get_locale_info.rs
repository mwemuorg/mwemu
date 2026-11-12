use crate::api::windows::common::strings::locale_info_value;
use crate::emu;
use crate::winapi::common::strings;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::kernel32::set_last_error;
use crate::winapi::winapi32::stdcall_ret;
use crate::windows::constants::errors::*;

pub fn GetLocaleInfoA(emu: &mut emu::Emu) {
    get_locale_info(emu, "GetLocaleInfoA", false);
}

pub fn GetLocaleInfoW(emu: &mut emu::Emu) {
    get_locale_info(emu, "GetLocaleInfoW", true);
}

fn get_locale_info(emu: &mut emu::Emu, api: &str, wide: bool) {
    let lctype = arg(emu, 1) as u64;
    let buf = arg(emu, 2) as u64;
    let cch = arg(emu, 3) as usize;
    let value = locale_info_value(lctype).unwrap_or_else(|| {
        log::warn!("{} {api} unhandled lctype: 0x{:x}", emu.pos, lctype);
        "."
    });
    let required = value.len() + 1; // with the terminator
    log_red!(emu, "kernel32!{api} lctype: 0x{:x} ='{}'", lctype, value);
    if cch == 0 {
        emu.regs_mut().rax = required as u64;
    } else if buf == 0 {
        set_last_error(emu, ERROR_INVALID_PARAMETER as u32);
        emu.regs_mut().rax = 0;
    } else if cch < required {
        set_last_error(emu, ERROR_INSUFFICIENT_BUFFER as u32);
        emu.regs_mut().rax = 0;
    } else {
        strings::write_string(emu, buf, value, wide);
        emu.regs_mut().rax = required as u64;
    }
    stdcall_ret(emu, 4);
}
