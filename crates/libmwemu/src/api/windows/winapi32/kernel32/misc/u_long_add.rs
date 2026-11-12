use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn ULongAdd(emu: &mut emu::Emu) {
    let _ulAugend = WinApiAbi::arg(emu, 0) as u32;
    let _ulAddend = WinApiAbi::arg(emu, 1) as u32;
    let _pulResult = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!ULongAdd");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
