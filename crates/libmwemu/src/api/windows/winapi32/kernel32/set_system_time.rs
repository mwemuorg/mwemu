use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn SetSystemTime(emu: &mut emu::Emu) {
    let p0 = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!SetSystemTime");
    if arg(emu, 0) as u64 != 0 && emu.maps.is_mapped(arg(emu, 0) as u64) {
        let _ = emu.maps.read_word(arg(emu, 0) as u64).unwrap_or(0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
