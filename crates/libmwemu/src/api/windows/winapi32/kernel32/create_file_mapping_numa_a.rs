use crate::emu;
use crate::windows::constants;

pub fn CreateFileMappingNumaA(emu: &mut emu::Emu) {
    let _hFile = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpFileMappingAttributes = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _flProtect = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _dwMaximumSizeHigh = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _dwMaximumSizeLow = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _nndPreferred = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateFileMappingNumaA {}", lpName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
