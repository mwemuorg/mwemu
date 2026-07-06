use crate::emu;
use crate::windows::constants;

pub fn InternalCoStdMarshalObject(emu: &mut emu::Emu) {
    let _pStm = emu
        .maps
        .read_dword(emu.regs().get_esp())
        .expect("cannot read the api parameter");
    let _riid = emu
        .maps
        .read_dword(emu.regs().get_esp() + 4)
        .expect("cannot read the api parameter");
    let _pUnk = emu
        .maps
        .read_dword(emu.regs().get_esp() + 8)
        .expect("cannot read the api parameter");
    let _pServerCtx = emu
        .maps
        .read_dword(emu.regs().get_esp() + 12)
        .expect("cannot read the api parameter");
    let _dwDestCtx = emu
        .maps
        .read_dword(emu.regs().get_esp() + 16)
        .expect("cannot read the api parameter");
    let _pvDestCtx = emu
        .maps
        .read_dword(emu.regs().get_esp() + 20)
        .expect("cannot read the api parameter");
    let _mshlflags = emu
        .maps
        .read_dword(emu.regs().get_esp() + 24)
        .expect("cannot read the api parameter");

    log_red!(emu, "kernel32!InternalCoStdMarshalObject");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
