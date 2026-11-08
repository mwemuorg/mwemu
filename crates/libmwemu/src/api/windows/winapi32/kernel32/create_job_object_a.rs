use crate::emu;
use crate::windows::constants;

pub fn CreateJobObjectA(emu: &mut emu::Emu) {
    let _lpJobAttributes = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let lpName = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!CreateJobObjectA {}", lpName);

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..2 {
        emu.stack_pop32(false);
    }
}
