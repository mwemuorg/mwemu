use crate::emu;
use crate::winapi::helper;

pub fn CertEnumCRLsInStore(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    log_red!(
        emu,
        "crypt32!CertEnumCRLsInStore arg1:0x{:x} arg2:0x{:x}",
        arg1,
        arg2
    );
    emu.regs_mut().rax = helper::handler_create("CertEnumCRLsInStore");
}
