
use crate::emu;
use crate::windows::constants;
use crate::winapi::helper;
pub fn QuirkIsEnabledForPackage4Worker(emu: &mut emu::Emu) {
    let _userSid = emu.regs().rcx as u32;
    let _packageFullName = emu.regs().rdx as u32;
    let _int = emu.regs().r8 as u32;
    let _int = emu.regs().r9 as u32;
    let _int = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
    log_red!(emu, "kernel32!QuirkIsEnabledForPackage4Worker");
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        emu.regs_mut().rax = 1;
    } else {
        emu.regs_mut().rax = 0;
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = constants::ERROR_SUCCESS;
}
