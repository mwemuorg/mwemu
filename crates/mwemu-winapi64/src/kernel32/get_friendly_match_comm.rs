
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetFriendlyMatchComm(emu: &mut emu::Emu) {
    let _int = emu.regs().rcx as u32;
    let _int = emu.regs().rdx as u32;
    let _SourceString = emu.regs().r8 as u32;
    let _int = emu.regs().r9 as u32;
    let _int = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _int = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    log_red!(emu, "kernel32!GetFriendlyMatchComm");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
