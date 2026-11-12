use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn IsValidCodePage(emu: &mut emu::Emu) {
    let codepage = WinApiAbi::arg(emu, 0) as u32;

    log_red!(emu, "kernel32!IsValidCodePage {}", codepage);

    emu.stack_pop32(false);
    emu.regs_mut().rax = 1;
}
