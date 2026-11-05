use crate::emu;

pub fn SetThreadContext(emu: &mut emu::Emu) {
    let hndl = emu.regs().rcx;
    let ctx_ptr = emu.regs().rdx;

    log_red!(
        emu,
        "kernel32!SetThreadContext handle:0x{:x} ctx:0x{:x}",
        hndl,
        ctx_ptr
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = 1; // TRUE
}
