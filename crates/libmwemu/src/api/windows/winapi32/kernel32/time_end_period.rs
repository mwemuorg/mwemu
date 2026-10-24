use crate::emu;
use crate::windows::constants;

pub fn timeEndPeriod(emu: &mut emu::Emu) {
    let _uPeriod = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);

    log_red!(emu, "kernel32!timeEndPeriod");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
