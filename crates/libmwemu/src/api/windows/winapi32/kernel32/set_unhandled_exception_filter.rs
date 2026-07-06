use crate::emu;

pub fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let callback =
        emu.maps
            .read_dword(emu.regs().get_esp())
            .expect("kernel32!SetUnhandledExceptionFilter cannot read the callback") as u64;

    log_red!(
        emu,
        "kernel32!SetUnhandledExceptionFilter  callback: 0x{:x}",
        callback
    );

    emu.regs_mut().rax = emu.uef();
    emu.set_uef(callback);

    emu.stack_pop32(false);
}
