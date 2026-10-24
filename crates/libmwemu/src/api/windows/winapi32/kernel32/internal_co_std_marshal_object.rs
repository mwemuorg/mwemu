use crate::emu;
use crate::windows::constants;

pub fn InternalCoStdMarshalObject(emu: &mut emu::Emu) {
    let _pStm = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0);
    let _riid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let _pUnk = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0);
    let _pServerCtx = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let _dwDestCtx = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0);
    let _pvDestCtx = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0);
    let _mshlflags = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0);

    log_red!(emu, "kernel32!InternalCoStdMarshalObject");

    emu.regs_mut().rax = constants::ERROR_SUCCESS;

    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
