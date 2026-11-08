use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetConsoleTitleA(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    let s = if p0 != 0 && p0 > 0xffff {
        emu.maps.read_string(p0)
    } else {
        format!("0x{:x}", p0)
    };
    log_red!(emu, "kernel32!SetConsoleTitleA '{}'", s);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
