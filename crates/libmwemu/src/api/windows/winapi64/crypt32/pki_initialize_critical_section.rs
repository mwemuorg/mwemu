use crate::emu;

pub(super) fn PkiInitializeCriticalSection(emu: &mut emu::Emu) {
    let addr = emu.regs().rcx;
    let flags = emu.regs().rdx;

    log_red!(
        emu,
        "crypt32!PkiInitializeCriticalSection flags: 0x{:x} addr: 0x{:x}",
        flags,
        addr
    );

    emu.regs_mut().rax = 1;
}
