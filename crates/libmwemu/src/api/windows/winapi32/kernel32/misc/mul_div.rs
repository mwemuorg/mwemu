use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn MulDiv(emu: &mut emu::Emu) {
    let _nNumber = WinApiAbi::arg(emu, 0) as u32;
    let _nNumerator = WinApiAbi::arg(emu, 1) as u32;
    let _nDenominator = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!MulDiv");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
