use crate::emu;
use crate::windows::constants;

pub fn FoldStringA(emu: &mut emu::Emu) {
    let _dwMapFlags = emu.regs().rcx as u32;
    let lpSrcStr = emu.regs().rdx as u32;
    let _cchSrc = emu.regs().r8 as u32;
    let lpDestStr = emu.regs().r9 as u32;
    let _cchDest = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let lpSrcStr = emu.maps.read_string(lpSrcStr as u64);
    let lpDestStr = emu.maps.read_string(lpDestStr as u64);

    log_red!(emu, "kernel32!FoldStringA {} {}", lpSrcStr, lpDestStr);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
