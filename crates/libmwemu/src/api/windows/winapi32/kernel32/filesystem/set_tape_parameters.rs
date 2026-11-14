use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetTapeParameters(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwOperation = WinApiAbi::arg(emu, 1) as u32;
    let _lpTapeInformation = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!SetTapeParameters");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
