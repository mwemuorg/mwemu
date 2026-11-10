
use crate::windows::constants::errors::*;
use crate::emu;

pub fn GetDateFormatWWorker(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;
    let _a3 = emu.regs().r8 as u32;
    let _a4 = emu.regs().r9 as u32;
    let _a5 = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _a6 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _a7 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);

    log_red!(emu, "kernel32!GetDateFormatWWorker");
    let out_ptr = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    if out_ptr != 0 {
        emu.maps.write_qword(out_ptr, 0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
