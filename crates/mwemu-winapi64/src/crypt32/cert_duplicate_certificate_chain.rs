use crate::emu;
use crate::winapi::helper;

pub fn CertDuplicateCertificateChain(emu: &mut emu::Emu) {
    let context = emu.regs().rcx;
    log_red!(
        emu,
        "crypt32!CertDuplicateCertificateChain ctx:0x{:x}",
        context
    );
    emu.set_last_error(0);
    let handle = helper::handler_create("certduplicatecertificatechain");
    emu.regs_mut().rax = handle;
}
