use crate::emu;
use crate::winapi::helper;
pub fn RemoveVectoredExceptionHandler(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!RemoveVectoredExceptionHandler handle:0x{:x}",
        handle
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
