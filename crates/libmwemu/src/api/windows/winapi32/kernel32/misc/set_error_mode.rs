use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn SetErrorMode(emu: &mut emu::Emu) {
    let mode = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!SetErrorMode 0x{:x}", mode);

    emu.stack_pop32(false);

    emu.regs_mut().rax = 0;
}
