use crate::emu;
use crate::winapi::helper;
use crate::windows::constants;
pub fn StringCchCopyExW(emu: &mut emu::Emu) {
    let _pszDest = emu.regs().rcx as u32;
    let _cchDest = emu.regs().rdx as u32;
    let _pszSrc = emu.regs().r8 as u32;
    let _ppszDestEnd = emu.regs().r9 as u32;
    let _pcchRemaining = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _dwFlags = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(emu, "kernel32!StringCchCopyExW");
    let handle = helper::handler_create("StringCchCopyExW");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
