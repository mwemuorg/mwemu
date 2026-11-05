use crate::emu;
use crate::windows::constants;

pub fn _tlgWriteTransfer_EventWriteTransfer(emu: &mut emu::Emu) {
    let _hProvider = emu.regs().rcx as u32;
    let _pEventMetadata = emu.regs().rdx as u32;
    let _pActivityId = emu.regs().r8 as u32;
    let _pRelatedActivityId = emu.regs().r9 as u32;
    let _cData = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _pData = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "kernel32!_tlgWriteTransfer_EventWriteTransfer");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
