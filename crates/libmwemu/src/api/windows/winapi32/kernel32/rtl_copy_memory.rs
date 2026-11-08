use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn RtlCopyMemory(emu: &mut emu::Emu) {
    let dest = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;
    let len = arg(emu, 2) as u64 as usize;

    log_red!(
        emu,
        "kernel32!RtlCopyMemory dest:0x{:x} src:0x{:x} len:{}",
        dest,
        src,
        len
    );

    if dest != 0 && src != 0 && len > 0 && len <= 0x100_0000 {
        let data = emu.maps.read_bytes(src, len).to_vec();
        emu.maps.write_bytes(dest, &data);
    }
    stdcall_ret(emu, 3);
}
