use crate::emu;
use crate::windows::constants;

pub fn FindActCtxSectionStringA(emu: &mut emu::Emu) {
    let _dwFlags = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpExtensionGuid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _ulSectionId = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let lpStringToFind = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _ReturnedData = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    let lpStringToFind = emu.maps.read_string(lpStringToFind as u64);

    log_red!(emu, "kernel32!FindActCtxSectionStringA {}", lpStringToFind);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
