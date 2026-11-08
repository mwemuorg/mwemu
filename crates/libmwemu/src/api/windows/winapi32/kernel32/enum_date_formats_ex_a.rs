use crate::emu;
use crate::windows::constants;

pub fn EnumDateFormatsExA(emu: &mut emu::Emu) {
    let _lpDateFmtEnumProcEx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _Locale = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!EnumDateFormatsExA");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
