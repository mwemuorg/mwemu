use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetThreadLocale(emu: &mut emu::Emu) {
    let locale = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!SetThreadLocale {}", locale);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
