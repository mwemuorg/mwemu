
use crate::windows::constants::errors::*;
use crate::emu;

pub fn FindActCtxSectionStringA(emu: &mut emu::Emu) {
    let _dwFlags = emu.regs().rcx as u32;
    let _lpExtensionGuid = emu.regs().rdx as u32;
    let _ulSectionId = emu.regs().r8 as u32;
    let lpStringToFind = emu.regs().r9 as u32;
    let _ReturnedData = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let lpStringToFind = emu.maps.read_string(lpStringToFind as u64);

    log_red!(emu, "kernel32!FindActCtxSectionStringA {}", lpStringToFind);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
