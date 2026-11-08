use crate::emu;
use crate::windows::constants;

pub fn QuirkIsEnabledForPackage4Worker(emu: &mut emu::Emu) {
    let _userSid = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _packageFullName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!QuirkIsEnabledForPackage4Worker");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
