use crate::emu;
use crate::windows::constants;

pub fn FindActCtxSectionGuidWorker(emu: &mut emu::Emu) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _extguid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _section_kind = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _guid = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _a5 = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);

    log_red!(emu, "kernel32!FindActCtxSectionGuidWorker");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
