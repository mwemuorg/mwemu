use crate::emu;
use crate::windows::constants;

pub fn ULongMult(emu: &mut emu::Emu) {
    let _ulMultiplicand = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _ulMultiplier = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _pulResult = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);

    log_red!(emu, "kernel32!ULongMult");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..3 {
        emu.stack_pop32(false);
    }
}
