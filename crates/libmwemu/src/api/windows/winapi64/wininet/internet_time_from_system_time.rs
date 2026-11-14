use crate::emu;
use crate::winapi::common::strings;

pub fn InternetTimeFromSystemTime(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "wininet!InternetTimeFromSystemTime arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetTimeFromSystemTimeA(emu: &mut emu::Emu) {
    internet_time_from_system_time(emu, "InternetTimeFromSystemTimeA", false);
}

fn internet_time_from_system_time(emu: &mut emu::Emu, api: &str, wide: bool) {
    let time_ptr = emu.regs().rcx;
    let format = emu.regs().rdx as u32;
    let buf_ptr = emu.regs().r8;
    log_red!(emu, "wininet!{api} format:{}", format);
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }
    if buf_ptr != 0 {
        strings::write_string(emu, buf_ptr, "Mon, 01 Jan 2024 00:00:00 GMT", wide);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}

pub fn InternetTimeFromSystemTimeW(emu: &mut emu::Emu) {
    internet_time_from_system_time(emu, "InternetTimeFromSystemTimeW", true);
}
