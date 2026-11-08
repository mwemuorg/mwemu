use crate::emu;
use crate::windows::constants;

pub fn SetProcessAffinityMask(emu: &mut emu::Emu) {
    let _hProcess = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _dwProcessAffinityMask = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!SetProcessAffinityMask");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
