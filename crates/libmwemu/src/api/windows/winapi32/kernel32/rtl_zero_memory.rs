use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RtlZeroMemory(emu: &mut emu::Emu) {
    let dest = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64 as usize;

    log_red!(emu, "kernel32!RtlZeroMemory dest:0x{:x} len:{}", dest, len);
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if dest != 0 && len > 0 && len <= 0x100_0000 {
        emu.maps.memset(dest, 0, len);
    }
    stdcall_ret(emu, 2);
}
