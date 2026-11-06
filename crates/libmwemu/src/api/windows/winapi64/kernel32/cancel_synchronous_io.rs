
use crate::emu;
use crate::winapi::helper;
pub fn CancelSynchronousIo(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CancelSynchronousIo thread:0x{:x}",
        thread_handle
    );
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
