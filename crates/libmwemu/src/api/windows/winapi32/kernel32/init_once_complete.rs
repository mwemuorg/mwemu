use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn InitOnceComplete(emu: &mut emu::Emu) {
    let lp_init_once = arg(emu, 0) as u64;
    let dw_flags = arg(emu, 1);
    let lp_context = arg(emu, 2) as u64;

    log_red!(
        emu,
        "kernel32!InitOnceComplete once:0x{:x} flags:0x{:x} ctx:0x{:x}",
        lp_init_once,
        dw_flags,
        lp_context
    );

    // Mark the INIT_ONCE as completed (value 2 = initialized)
    if lp_init_once != 0 {
        emu.maps.write_qword(lp_init_once, 2);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 3);
}
