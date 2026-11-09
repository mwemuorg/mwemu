use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetStringScripts(emu: &mut emu::Emu) {
    let flags = arg(emu, 0);
    let str_ptr = arg(emu, 1) as u64;
    let str_len = arg(emu, 2) as u64 as i32;
    let scripts_ptr = arg(emu, 3) as u64;
    log_red!(
        emu,
        "kernel32!GetStringScripts flags:0x{:x} str:0x{:x} len:{}",
        flags,
        str_ptr,
        str_len
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 4);
}
