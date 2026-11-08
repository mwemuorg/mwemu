use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn lstrlenA(emu: &mut emu::Emu) {
    let s_ptr = arg(emu, 0) as u64;

    let s = emu.maps.read_string(s_ptr);
    let len = s.len() as u64;

    log_red!(emu, "kernel32!lstrlen '{}' ={}", s, len);

    emu.set_last_error(0);
    emu.regs_mut().rax = len;
    stdcall_ret(emu, 1);
}
