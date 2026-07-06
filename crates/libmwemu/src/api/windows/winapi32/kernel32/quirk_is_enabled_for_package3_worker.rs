use crate::emu;
use crate::windows::constants;

pub fn QuirkIsEnabledForPackage3Worker(emu: &mut emu::Emu) {
    let _userSid = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _packageFullName = emu
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

    log_red!(emu, "kernel32!QuirkIsEnabledForPackage3Worker");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
