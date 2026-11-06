use crate::emu;
use crate::winapi::helper;

pub fn ClearCommBreak(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    log_red!(emu, "kernel32!ClearCommBreak hFile:0x{:x}", h_file);
    if helper::handler_exist(emu.regs().rcx) { }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
