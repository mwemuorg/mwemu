use crate::emu;

pub fn ClearCommBreak(emu: &mut emu::Emu) {
    let h_file = emu.regs().rcx;
    log_red!(emu, "kernel32!ClearCommBreak hFile:0x{:x}", h_file);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
