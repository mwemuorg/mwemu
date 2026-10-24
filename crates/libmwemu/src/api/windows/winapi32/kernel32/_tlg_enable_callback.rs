use crate::emu;
use crate::windows::constants;

pub fn _tlgEnableCallback(emu: &mut emu::Emu) {
    let _pSourceId = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _callbackType = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _level = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _keywordAny = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _keywordAll = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _pFilterData = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _pCallbackContext = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    log_red!(emu, "kernel32!_tlgEnableCallback");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
