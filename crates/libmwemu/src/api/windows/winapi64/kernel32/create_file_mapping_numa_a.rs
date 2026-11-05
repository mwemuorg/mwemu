use crate::emu;
use crate::winapi::helper;

pub fn CreateFileMappingNumaA(emu: &mut emu::Emu) {
    let _hFile = emu.regs().rcx as u32;
    let _lpFileMappingAttributes = emu.regs().rdx as u32;
    let _flProtect = emu.regs().r8 as u32;
    let _dwMaximumSizeHigh = emu.regs().r9 as u32;
    let _dwMaximumSizeLow = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let lpName = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _nndPreferred = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);

    let lpName = emu.maps.read_string(lpName);

    log_red!(emu, "kernel32!CreateFileMappingNumaA {}", lpName);

    emu.regs_mut().rax = helper::handler_create("CreateFileMappingNumaA");
}
