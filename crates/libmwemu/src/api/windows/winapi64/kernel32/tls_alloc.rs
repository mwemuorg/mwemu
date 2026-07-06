use crate::emu;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    log_red!(emu, "kernel32!TlsAlloc");

    emu.tls64_mut().push(0);
    emu.regs_mut().rax = (emu.tls64().len() - 1) as u64; // Return index of newly allocated slot
}
