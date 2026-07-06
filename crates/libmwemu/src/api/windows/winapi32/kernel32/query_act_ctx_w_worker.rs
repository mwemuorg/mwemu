use crate::emu;
use crate::windows::constants;

pub fn QueryActCtxWWorker(emu: &mut emu::Emu) {
    let _int = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _Context = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _pvSubInstance = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _ulInfoClass = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _pvBuffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _cbBuffer = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");
    let _pcbWrittenOrRequired = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!QueryActCtxWWorker");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
