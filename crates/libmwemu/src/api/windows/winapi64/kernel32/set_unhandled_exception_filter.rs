use crate::emu;

pub fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let callback = emu.regs().rcx;

    log_red!(
        emu,
        "kernel32!SetUnhandledExceptionFilter callback:0x{:x}",
        callback
    );

    emu.set_last_error(0);
    emu.regs_mut().rax = emu.uef();
    emu.set_uef(callback);
}
