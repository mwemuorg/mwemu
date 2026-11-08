use crate::emu;
use crate::windows::constants;

pub fn GetFriendlyMatchComm(emu: &mut emu::Emu) {
    let _int = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _SourceString = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _int = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);

    log_red!(emu, "kernel32!GetFriendlyMatchComm");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
