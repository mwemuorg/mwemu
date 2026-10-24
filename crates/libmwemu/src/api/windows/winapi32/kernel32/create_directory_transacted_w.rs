use crate::emu;
use crate::windows::constants;

pub fn CreateDirectoryTransactedW(emu: &mut emu::Emu) {
    let _lpTemplateDirectory = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _lpNewDirectory = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _lpSecurityAttributes = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _hTransaction = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);

    log_red!(emu, "kernel32!CreateDirectoryTransactedW");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
