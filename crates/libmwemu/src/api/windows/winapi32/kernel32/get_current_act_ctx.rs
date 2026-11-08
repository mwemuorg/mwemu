use crate::emu;
use crate::winapi::winapi32::{arg, stdcall_ret};

pub fn GetCurrentActCtx(emu: &mut emu::Emu) {
    let act_ctx_ptr = arg(emu, 0) as u64;
    log_red!(emu, "kernel32!GetCurrentActCtx ptr:0x{:x}", act_ctx_ptr);
    if act_ctx_ptr != 0 {
        emu.maps.write_qword(act_ctx_ptr, 0);
    }
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
