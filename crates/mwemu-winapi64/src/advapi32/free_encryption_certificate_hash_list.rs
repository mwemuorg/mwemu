use crate::emu;
use crate::winapi::helper;

pub fn FreeEncryptionCertificateHashList(emu: &mut emu::Emu) {
    let handle = emu.regs().rcx;
    helper::handler_close(handle);
    log_red!(
        emu,
        "advapi32!FreeEncryptionCertificateHashList handle:0x{:x}",
        handle
    );
    emu.regs_mut().rax = 1;
}
