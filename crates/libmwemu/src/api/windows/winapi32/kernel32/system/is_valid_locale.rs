use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn IsValidLocale(emu: &mut emu::Emu) {
    let locale = WinApiAbi::arg(emu, 0) as u32;
    let flags = WinApiAbi::arg(emu, 1) as u32;

    log_red!(emu, "kernel32!IsValidLocale");

    emu.regs_mut().rax = 1;
    emu.stack_pop32(false);
    emu.stack_pop32(false);
}
