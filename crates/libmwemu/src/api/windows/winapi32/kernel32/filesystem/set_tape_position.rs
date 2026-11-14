use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn SetTapePosition(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwPositionMethod = WinApiAbi::arg(emu, 1) as u32;
    let _dwPartition = WinApiAbi::arg(emu, 2) as u32;
    let _dwOffsetLow = WinApiAbi::arg(emu, 3) as u32;
    let _dwOffsetHigh = WinApiAbi::arg(emu, 4) as u32;
    let _bImmediate = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!SetTapePosition");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
