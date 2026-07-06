use crate::emu;
use crate::windows::constants;

pub fn GetMailslotInfo(emu: &mut emu::Emu) {
    let _hMailslot = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _lpMaxMessageSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _lpNextSize = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _lpMessageCount = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _lpReadTimeout = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!GetMailslotInfo");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
