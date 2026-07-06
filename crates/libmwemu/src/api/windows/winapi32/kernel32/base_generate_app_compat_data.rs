use crate::emu;
use crate::windows::constants;

pub fn BaseGenerateAppCompatData(emu: &mut emu::Emu) {
    let _a1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _a2 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _a3 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _a4 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _a5 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _a6 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!BaseGenerateAppCompatData");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
