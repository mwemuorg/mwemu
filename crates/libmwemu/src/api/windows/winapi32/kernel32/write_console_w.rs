use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn WriteConsoleW(emu: &mut emu::Emu) {
    let buf = arg(emu, 1) as u64;
    let count = arg(emu, 2) as usize;
    let written_ptr = arg(emu, 3) as u64;
    let text = emu.maps.read_wide_string_n(buf, count);
    log_red!(emu, "kernel32!WriteConsoleW `{}`", text);
    if written_ptr != 0 {
        emu.maps.write_dword(written_ptr, count as u32);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 5);
}
