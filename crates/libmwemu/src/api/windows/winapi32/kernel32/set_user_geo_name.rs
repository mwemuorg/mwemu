use crate::emu;
use crate::windows::constants;

pub fn SetUserGeoName(emu: &mut emu::Emu) {
    let _a1 = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!SetUserGeoName");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
