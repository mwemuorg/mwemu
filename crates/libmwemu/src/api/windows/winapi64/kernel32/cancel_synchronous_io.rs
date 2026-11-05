use crate::emu;

pub fn CancelSynchronousIo(emu: &mut emu::Emu) {
    let thread_handle = emu.regs().rcx;
    log_red!(
        emu,
        "kernel32!CancelSynchronousIo thread:0x{:x}",
        thread_handle
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
