use crate::emu;

pub fn SetThreadContext_api(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    let context = emu.regs().rdx;
    log_red!(
        emu,
        "kernel32!SetThreadContext h:0x{:x} ctx:0x{:x}",
        handle,
        context
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
