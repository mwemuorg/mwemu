use crate::emu;
use crate::windows::constants;

pub fn StringCopyWorkerW_2_0(emu: &mut emu::Emu) {
    let _pszDest = emu.regs().rcx as u32;
    let _cchDest = emu.regs().rdx as u32;
    let _pcchNewDestLength = emu.regs().r8 as u32;
    let _pszSrc = emu.regs().r9 as u32;
    let _cchToCopy = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!StringCopyWorkerW_2_0");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
