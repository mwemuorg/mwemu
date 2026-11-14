use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn GetFileBandwidthReservation(emu: &mut emu::Emu) {
    let _hFile = WinApiAbi::arg(emu, 0) as u32;
    let _lpPeriodMilliseconds = WinApiAbi::arg(emu, 1) as u32;
    let _lpBytesPerPeriod = WinApiAbi::arg(emu, 2) as u32;
    let _pDiscardable = WinApiAbi::arg(emu, 3) as u32;
    let _lpTransferSize = WinApiAbi::arg(emu, 4) as u32;
    let _lpNumOutstandingRequests = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!GetFileBandwidthReservation");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
