use crate::emu;
use crate::windows::constants;

pub fn NtVdm64CreateProcessInternalW(emu: &mut emu::Emu) {
    let _a1 = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _a2 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _cchDest = emu
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
    let _a7 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");
    let _a8 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 28)
        .expect("cannot read the api parameter");
    let _a9 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 32)
        .expect("cannot read the api parameter");
    let _a10 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 36)
        .expect("cannot read the api parameter");
    let _a11 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 40)
        .expect("cannot read the api parameter");
    let _a12 = emu
        .maps
        .read_dword(emu.regs().get_esp() + 44)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!NtVdm64CreateProcessInternalW");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..12 {
        emu.stack_pop32(false);
    }
}
