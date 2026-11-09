use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn uaw_wcslen(emu: &mut emu::Emu) {
    let wstr = arg(emu, 0) as u64;

    let s = emu.maps.read_wide_string(wstr);
    let len = s.encode_utf16().count();

    log_red!(emu, "kernel32!uaw_wcslen str:0x{:x} len:{}", wstr, len);

    emu.set_last_error(0);
    emu.regs_mut().rax = len as u64;
    stdcall_ret(emu, 1);
}
