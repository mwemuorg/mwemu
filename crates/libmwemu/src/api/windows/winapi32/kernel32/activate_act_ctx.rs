use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn ActivateActCtx(emu: &mut emu::Emu) {
    let h_act_ctx = arg(emu, 0) as u64;
    let lp_cookie = arg(emu, 1) as u64;
    log_red!(
        emu,
        "kernel32!ActivateActCtx ctx:0x{:x} cookie:0x{:x}",
        h_act_ctx,
        lp_cookie
    );
    if lp_cookie != 0 {
        emu.maps.write_qword(lp_cookie, 0x1001);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 2);
}
