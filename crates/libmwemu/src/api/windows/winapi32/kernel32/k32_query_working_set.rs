use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn K32QueryWorkingSet(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let p_info = arg(emu, 1) as u64;
    let cb = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!K32QueryWorkingSet hProc:0x{:x} pInfo:0x{:x} cb:{}",
        h_process,
        p_info,
        cb
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
