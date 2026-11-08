use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RtlFillMemory(emu: &mut emu::Emu) {
    let dest = arg(emu, 0) as u64;
    let len = arg(emu, 1) as u64 as usize;
    let fill = arg(emu, 2) as u64 as u8;

    log_red!(
        emu,
        "kernel32!RtlFillMemory dest:0x{:x} len:{} fill:0x{:02x}",
        dest,
        len,
        fill
    );
    let first_param = arg(emu, 0) as u64;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    if dest != 0 && len > 0 && len <= 0x100_0000 {
        emu.maps.memset(dest, fill, len);
    }
    stdcall_ret(emu, 3);
}
