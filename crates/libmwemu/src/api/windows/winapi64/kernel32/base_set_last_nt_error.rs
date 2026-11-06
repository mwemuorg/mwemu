use crate::emu;

pub fn BaseSetLastNTError(emu: &mut emu::Emu) {
    let param1 = emu.regs().rcx;
    log_red!(emu, "kernel32!BaseSetLastNTError param1:0x{:x}", param1);
    let status = emu.regs().rcx as u32;
    emu.set_last_error(status);
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
}
