use crate::emu;
use crate::windows::constants;

pub fn GetDurationFormat(emu: &mut emu::Emu) {
    let _Locale = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpDuration = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _ullDuration = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _lpFormat = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _lpDurationStr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _cchDuration = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    log_red!(emu, "kernel32!GetDurationFormat");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
