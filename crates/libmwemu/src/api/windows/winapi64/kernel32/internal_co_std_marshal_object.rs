
use crate::windows::constants::errors::*;
use crate::emu;

pub fn InternalCoStdMarshalObject(emu: &mut emu::Emu) {
    let _pStm = emu.regs().rcx as u32;
    let _riid = emu.regs().rdx as u32;
    let _pUnk = emu.regs().r8 as u32;
    let _pServerCtx = emu.regs().r9 as u32;
    let _dwDestCtx = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _pvDestCtx = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _mshlflags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "kernel32!InternalCoStdMarshalObject");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
