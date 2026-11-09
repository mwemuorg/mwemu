
use crate::windows::constants::errors::*;
use crate::emu;

pub fn QuirkIsEnabledForPackage2Worker(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;
    let _a3 = emu.regs().r8 as u32;
    let _a4 = emu.regs().r9 as u32;
    let _a5 = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    log_red!(emu, "kernel32!QuirkIsEnabledForPackage2Worker");
    let h = emu.regs().rcx;
    emu.regs_mut().rax = 1;
    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
