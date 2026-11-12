use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::windows::constants::errors::*;

pub fn _tlgWriteTransfer_EventWriteTransfer(emu: &mut emu::Emu) {
    let _hProvider = WinApiAbi::arg(emu, 0) as u32;
    let _pEventMetadata = WinApiAbi::arg(emu, 1) as u32;
    let _pActivityId = WinApiAbi::arg(emu, 2) as u32;
    let _pRelatedActivityId = WinApiAbi::arg(emu, 3) as u32;
    let _cData = WinApiAbi::arg(emu, 4) as u32;
    let _pData = WinApiAbi::arg(emu, 5) as u32;

    log_red!(emu, "kernel32!_tlgWriteTransfer_EventWriteTransfer");

    emu.regs_mut().rax = ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
