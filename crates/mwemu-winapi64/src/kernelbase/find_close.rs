use crate::emu;
use crate::winapi::helper;

pub fn FindClose(emu: &mut emu::Emu) {
    let hfind = emu.regs().rcx;
    log_red!(emu, "kernelbase!FindClose handle:0x{:x}", hfind);
    if helper::handler_exist(hfind) {
        helper::handler_close(hfind);
    }
    emu.regs_mut().rax = 1;
}
