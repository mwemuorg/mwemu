use crate::emu;
use crate::windows::constants;

pub fn RegisterConsoleOS2(emu: &mut emu::Emu) {
    let _a1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!RegisterConsoleOS2");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
