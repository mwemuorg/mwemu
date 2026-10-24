use crate::emu;
use crate::windows::constants;

pub fn RegisterApplicationRecoveryCallback(emu: &mut emu::Emu) {
    let _pRecoveyCallback = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _pvParameter = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwPingInterval = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!RegisterApplicationRecoveryCallback");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
