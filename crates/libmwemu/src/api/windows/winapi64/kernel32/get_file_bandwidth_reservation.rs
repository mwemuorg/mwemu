use crate::emu;
use crate::windows::constants;

pub fn GetFileBandwidthReservation(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let _lpPeriodMilliseconds = emu.regs().rdx as u32;
    let _lpBytesPerPeriod = emu.regs().r8 as u32;
    let _pDiscardable = emu.regs().r9 as u32;
    let _lpTransferSize = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _lpNumOutstandingRequests = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "kernel32!GetFileBandwidthReservation");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
