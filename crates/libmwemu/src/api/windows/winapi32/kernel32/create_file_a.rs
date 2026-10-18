use crate::emu;
use crate::winapi::helper;
use crate::winapi::winapi32::{arg, stdcall_ret};
use crate::windows::constants;

pub fn CreateFileA(emu: &mut emu::Emu) {
    let fname_ptr = arg(emu, 0) as u64;
    let access = arg(emu, 1);
    let fname = emu.maps.read_string(fname_ptr);
    let mut perm = String::new();
    if access & constants::GENERIC_READ != 0 {
        perm.push('r');
    }
    if access & constants::GENERIC_WRITE != 0 {
        perm.push('w');
    }
    log_red!(emu, "kernel32!CreateFileA `{}` {}", fname, perm);
    emu.regs_mut().rax = helper::handler_create(&format!("file://{}", fname));
    stdcall_ret(emu, 7);
}
