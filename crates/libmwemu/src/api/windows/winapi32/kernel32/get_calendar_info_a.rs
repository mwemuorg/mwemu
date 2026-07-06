use crate::emu;
use crate::windows::constants;

pub fn GetCalendarInfoA(emu: &mut emu::Emu) {
    let _Locale = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _Calendar = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _CalType = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _lpCalData = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _cchData = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _lpValue = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!GetCalendarInfoA");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..6 {
        emu.stack_pop32(false);
    }
}
