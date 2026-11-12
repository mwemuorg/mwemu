use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetTapePosition(emu: &mut emu::Emu) {
    let _hDevice = WinApiAbi::arg(emu, 0) as u32;
    let _dwPositionType = WinApiAbi::arg(emu, 1) as u32;
    let _lpdwPartition = WinApiAbi::arg(emu, 2) as u32;
    let _lpdwOffsetLow = WinApiAbi::arg(emu, 3) as u32;
    let _lpdwOffsetHigh = WinApiAbi::arg(emu, 4) as u32;

    log_red!(emu, "kernel32!GetTapePosition");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
