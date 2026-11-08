use crate::emu;
use crate::winapi::helper;
pub fn FlushInstructionCache(emu: &mut emu::Emu) {
    let h_process = emu.regs().rcx;
    let lp_base = emu.regs().rdx;
    let size = emu.regs().r8;
    log_red!(
        emu,
        "kernel32!FlushInstructionCache hProcess:0x{:x} base:0x{:x} size:{}",
        h_process,
        lp_base,
        size
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
