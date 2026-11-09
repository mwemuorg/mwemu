
use crate::windows::constants::errors::*;
use crate::emu;

pub fn UpdateResourceA(emu: &mut emu::Emu) {
    let _hUpdate = emu.regs().rcx as u32;
    let _lpType = emu.regs().rdx as u32;
    let lpName = emu.regs().r8 as u32;
    let _wLanguage = emu.regs().r9 as u32;
    let _lpData = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _cb = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName as u64);

    log_red!(emu, "kernel32!UpdateResourceA {}", lpName);

    emu.set_last_error(0);
    emu.regs_mut().rax = ERROR_SUCCESS;
}
