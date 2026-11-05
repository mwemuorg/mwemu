use crate::emu;
use crate::windows::constants;

pub fn SortFindString(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;
    let _a3 = emu.regs().r8 as u32;
    let _a4 = emu.regs().r9 as u32;
    let _a5 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _a6 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _a7 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _a8 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let _a9 = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);

    log_red!(emu, "kernel32!SortFindString");

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
