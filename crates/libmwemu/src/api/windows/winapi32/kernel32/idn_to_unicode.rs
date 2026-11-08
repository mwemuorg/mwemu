use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn IdnToUnicode(emu: &mut emu::Emu) {
    let flags = arg(emu, 0) as u64;
    let p_src = arg(emu, 1) as u64;
    let src_len = arg(emu, 2) as u64 as i32;
    let p_dest = arg(emu, 3) as u64;
    let dest_len = arg(emu, 4) as u64 as i32;

    let src = emu.maps.read_wide_string(p_src);
    log_red!(
        emu,
        "kernel32!IdnToUnicode flags:0x{:x} src:'{}'",
        flags,
        src
    );

    if p_dest != 0 && dest_len > 0 {
        emu.maps.write_wide_string(p_dest, &src);
    }
    emu.regs_mut().rax = src.len() as u64 + 1;
    stdcall_ret(emu, 5);
}
