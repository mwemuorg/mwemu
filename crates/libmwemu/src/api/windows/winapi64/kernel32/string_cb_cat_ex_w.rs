use crate::emu;
use crate::windows::constants;

pub fn StringCbCatExW(emu: &mut emu::Emu) {
    let _pszDest = emu.regs().rcx as u32;
    let _cbDest = emu.regs().rdx as u32;
    let _pszSrc = emu.regs().r8 as u32;
    let _ppszDestEnd = emu.regs().r9 as u32;
    let _pcbRemaining = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _dwFlags = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "kernel32!StringCbCatExW");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
