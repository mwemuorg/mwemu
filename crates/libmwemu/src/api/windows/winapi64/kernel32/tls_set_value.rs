use crate::emu;

pub fn TlsSetValue(emu: &mut emu::Emu) {
    let idx = emu.regs().rcx as usize; // First parameter in RCX
    let val = emu.regs().rdx; // Second parameter in RDX

    log_red!(emu, "kernel32!TlsSetValue idx: {} val: 0x{:x}", idx, val);

    if idx < emu.tls64().len() {
        emu.tls64_mut()[idx] = val;
    } else {
        // Expand TLS array if needed
        while emu.tls64().len() <= idx {
            emu.tls64_mut().push(0);
        }
        emu.tls64_mut()[idx] = val;
    }

    emu.regs_mut().rax = 1; // Return TRUE
}
