use crate::emu;
use crate::windows::constants;

pub fn FindNextVolumeMountPointW(emu: &mut emu::Emu) {
    let _hFindVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpszVolumeMountPoint = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _cchBufferLength = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!FindNextVolumeMountPointW");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
