use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn K32GetWsChangesEx(emu: &mut emu::Emu) {
    let h_process = arg(emu, 0) as u64;
    let p_info = arg(emu, 1) as u64;
    let p_cb = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!K32GetWsChangesEx hProc:0x{:x} pInfo:0x{:x} pCb:0x{:x}",
        h_process,
        p_info,
        p_cb
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    stdcall_ret(emu, 3);
}
