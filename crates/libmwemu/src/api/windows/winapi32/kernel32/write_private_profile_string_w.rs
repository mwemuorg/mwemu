use crate::emu;
use crate::windows::constants;

pub fn WritePrivateProfileStringW(emu: &mut emu::Emu) {
    let lpAppName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpKeyName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let lpString = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpAppName = emu.maps.read_wide_string(lpAppName as u64);
    let lpKeyName = emu.maps.read_wide_string(lpKeyName as u64);
    let lpString = emu.maps.read_wide_string(lpString as u64);
    let lpFileName = emu.maps.read_wide_string(lpFileName as u64);

    log_red!(
        emu,
        "kernel32!WritePrivateProfileStringW {} {} {} {}",
        lpAppName,
        lpKeyName,
        lpString,
        lpFileName
    );

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
