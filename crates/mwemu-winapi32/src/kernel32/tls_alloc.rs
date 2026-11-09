use crate::emu;

pub fn TlsAlloc(emu: &mut emu::Emu) {
    emu.tls32_mut().push(0);
    let idx = (emu.tls32().len() - 1) as u64;
    emu.regs_mut().set_eax(idx);

    log_red!(emu, "kernel32!TlsAlloc = {}", idx);
}
