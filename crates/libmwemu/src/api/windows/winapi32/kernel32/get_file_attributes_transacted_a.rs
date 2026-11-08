use crate::emu;
use crate::windows::constants;

pub fn GetFileAttributesTransactedA(emu: &mut emu::Emu) {
    let lpFileName = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _fInfoLevelId = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpFileInformation = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(emu, "kernel32!GetFileAttributesTransactedA {}", lpFileName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
