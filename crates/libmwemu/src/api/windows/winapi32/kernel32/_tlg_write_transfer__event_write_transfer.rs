use crate::emu;
use crate::windows::constants;

pub fn _tlgWriteTransfer_EventWriteTransfer(emu: &mut emu::Emu) {
    let _hProvider = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _pEventMetadata = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _pActivityId = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _pRelatedActivityId = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _cData = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _pData = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    log_red!(emu, "kernel32!_tlgWriteTransfer_EventWriteTransfer");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
