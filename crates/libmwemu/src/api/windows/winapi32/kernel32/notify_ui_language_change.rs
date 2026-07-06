use crate::emu;
use crate::windows::constants;

pub fn NotifyUILanguageChange(emu: &mut emu::Emu) {
    let _dwFlags = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _pcwstrNewLanguage = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _pcwstrPreviousLanguage = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _dwReserved = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _pdwStatusRtrn = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!NotifyUILanguageChange");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
