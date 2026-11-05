use crate::emu;
use crate::windows::constants;

pub fn GetFullPathNameTransactedA(emu: &mut emu::Emu) {
    let lpFileName = emu.regs().rcx as u32;
    let _nBufferLength = emu.regs().rdx as u32;
    let _lpBuffer = emu.regs().r8 as u32;
    let _lpFilePart = emu.regs().r9 as u32;
    let _hTransaction = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let lpFileName = emu.maps.read_string(lpFileName as u64);

    log_red!(emu, "kernel32!GetFullPathNameTransactedA {}", lpFileName);

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
