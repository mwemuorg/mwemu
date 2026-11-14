use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn EraseTape(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwEraseType = WinApiAbi::arg(emu, 1) as u32;
    let _bImmediate = WinApiAbi::arg(emu, 2) as u32;

    log_red!(emu, "kernel32!EraseTape");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
