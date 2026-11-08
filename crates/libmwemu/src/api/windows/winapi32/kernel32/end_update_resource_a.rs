use crate::emu;
use crate::windows::constants;

pub fn EndUpdateResourceA(emu: &mut emu::Emu) {
    let _hUpdate = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _fDiscard = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    log_red!(emu, "kernel32!EndUpdateResourceA");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
