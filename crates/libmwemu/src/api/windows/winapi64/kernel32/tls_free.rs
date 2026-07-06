use crate::emu;

pub fn TlsFree(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx as usize; // First parameter passed in RCX in x64

    log_red!(emu, "kernel32!TlsFree idx: {}", idx);

    if idx < emu.tls64().len() {
        emu.tls64_mut()[idx] = 0; // Clear the slot
        emu.regs_mut().rax = 1; // Return TRUE
    } else {
        emu.regs_mut().rax = 0; // Return FALSE if invalid index
    }
}
