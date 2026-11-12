use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn WriteTapemark(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwTapemarkType = WinApiAbi::arg(emu, 1) as u32;
    let _dwTapemarkCount = WinApiAbi::arg(emu, 2) as u32;
    let _bImmediate = WinApiAbi::arg(emu, 3) as u32;

    log_red!(emu, "kernel32!WriteTapemark");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
