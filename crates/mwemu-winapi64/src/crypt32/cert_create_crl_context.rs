use crate::emu;
use crate::winapi::helper;

pub fn CertCreateCRLContext(emu: &mut emu::Emu) {
    let arg1 = emu.regs().rcx;
    let arg2 = emu.regs().rdx;
    let arg3 = emu.regs().r8;
    log_red!(
        emu,
        "crypt32!CertCreateCRLContext arg1:0x{:x} arg2:0x{:x} arg3:0x{:x}",
        arg1,
        arg2,
        arg3
    );
    emu.regs_mut().rax = helper::handler_create("CertCreateCRLContext");
}
