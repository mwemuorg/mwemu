
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetFileBandwidthReservation(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let _lpPeriodMilliseconds = emu.regs().rdx as u32;
    let _lpBytesPerPeriod = emu.regs().r8 as u32;
    let _pDiscardable = emu.regs().r9 as u32;
    let _lpTransferSize = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _lpNumOutstandingRequests = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!GetFileBandwidthReservation");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
