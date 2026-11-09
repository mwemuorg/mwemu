use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn uaw_wcscpy(emu: &mut emu::Emu) {
    let dest = arg(emu, 0) as u64;
    let src = arg(emu, 1) as u64;

    let s = emu.maps.read_wide_string(src);
    log_red!(emu, "kernel32!uaw_wcscpy dest:0x{:x} src:'{}'", dest, s);

    if dest != 0 {
        emu.maps.write_wide_string(dest, &s);
    }
    emu.regs_mut().rax = dest;
    stdcall_ret(emu, 2);
}
