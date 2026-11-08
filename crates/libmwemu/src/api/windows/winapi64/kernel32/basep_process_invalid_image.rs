use crate::emu;
use crate::windows::constants;

pub fn BasepProcessInvalidImage(emu: &mut emu::Emu) {
    let _a1 = emu.regs().rcx as u32;
    let _a2 = emu.regs().rdx as u32;
    let _lpFileName = emu.regs().r8 as u32;
    let _a4 = emu.regs().r9 as u32;
    let _a5 = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let _a6 = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    let _a7 = emu.maps.read_qword(emu.regs().rsp + 0x30).unwrap_or(0);
    let _a8 = emu.maps.read_qword(emu.regs().rsp + 0x38).unwrap_or(0);
    let _a9 = emu.maps.read_qword(emu.regs().rsp + 0x40).unwrap_or(0);
    let _a10 = emu.maps.read_qword(emu.regs().rsp + 0x48).unwrap_or(0);
    let _a11 = emu.maps.read_qword(emu.regs().rsp + 0x50).unwrap_or(0);
    let _a12 = emu.maps.read_qword(emu.regs().rsp + 0x58).unwrap_or(0);
    let _a13 = emu.maps.read_qword(emu.regs().rsp + 0x60).unwrap_or(0);
    let _a14 = emu.maps.read_qword(emu.regs().rsp + 0x68).unwrap_or(0);
    let _a15 = emu.maps.read_qword(emu.regs().rsp + 0x70).unwrap_or(0);
    let _a16 = emu.maps.read_qword(emu.regs().rsp + 0x78).unwrap_or(0);
    let _a17 = emu.maps.read_qword(emu.regs().rsp + 0x80).unwrap_or(0);
    let _a18 = emu.maps.read_qword(emu.regs().rsp + 0x88).unwrap_or(0);
    let _a19 = emu.maps.read_qword(emu.regs().rsp + 0x90).unwrap_or(0);
    let _a20 = emu.maps.read_qword(emu.regs().rsp + 0x98).unwrap_or(0);
    let _a21 = emu.maps.read_qword(emu.regs().rsp + 0xa0).unwrap_or(0);

    log_red!(emu, "kernel32!BasepProcessInvalidImage");
    let first_param = emu.regs().rcx;
    if first_param != 0 && emu.maps.is_mapped(first_param) {
        let _ = emu.maps.read_dword(first_param).unwrap_or(0);
    }

    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
