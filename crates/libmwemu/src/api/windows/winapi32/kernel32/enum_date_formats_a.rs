use crate::emu;
use crate::windows::constants;

pub fn EnumDateFormatsA(emu: &mut emu::Emu) {
    let _lpDateFmtEnumProc = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Locale = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!EnumDateFormatsA");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
