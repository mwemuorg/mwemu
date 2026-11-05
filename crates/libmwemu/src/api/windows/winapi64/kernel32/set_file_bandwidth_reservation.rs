use crate::emu;

pub fn SetFileBandwidthReservation(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let _nPeriodMilliseconds = emu.regs().rdx as u32;
    let _nBytesPerPeriod = emu.regs().r8 as u32;
    let _bDiscardable = emu.regs().r9 as u32;
    let _lpTransferSize = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _lpNumOutstandingRequests = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "kernel32!SetFileBandwidthReservation");

    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
