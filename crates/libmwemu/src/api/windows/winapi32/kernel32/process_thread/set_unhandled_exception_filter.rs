use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetUnhandledExceptionFilter(emu: &mut emu::Emu) {
    let callback = WinApiAbi::arg(emu, 0);

    log_red!(
        emu,
        "kernel32!SetUnhandledExceptionFilter  callback: 0x{:x}",
        callback
    );

    emu.regs_mut().rax = emu.uef();
    emu.set_uef(callback);

    emu.stack_pop32(false);
}
