
use crate::windows::constants::errors::*;
use crate::emu;
use crate::winapi::helper;

pub fn StringCopyWorkerW(emu: &mut emu::Emu) {
    let _pszDest = emu.regs().rcx as u32;
    let _cchDest = emu.regs().rdx as u32;
    let _pcchNewDestLength = emu.regs().r8 as u32;
    let _pszSrc = emu.regs().r9 as u32;
    let _cchToCopy = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    log_red!(emu, "kernel32!StringCopyWorkerW");
    let handle = helper::handler_create("StringCopyWorkerW");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, handle);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
