use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn DeactivateActCtx(emu: &mut emu::Emu) {
    let act_ctx = arg(emu, 0) as u64;
    let cookie = arg(emu, 1) as u64;

    log_red!(emu, "kernel32!DeactivateActCtx ctx:0x{:x}", act_ctx);

    if cookie != 0 {
        emu.maps.write_qword(cookie, 0x1234);
    }
    emu.regs_mut().rax = 1; // TRUE
    stdcall_ret(emu, 2);
}
