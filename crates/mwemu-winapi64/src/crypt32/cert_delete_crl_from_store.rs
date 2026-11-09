use crate::emu;
use crate::winapi::helper;

pub fn CertDeleteCRLFromStore(emu: &mut emu::Emu) {
    let cert_ctx = emu.regs().rcx;
    log_red!(emu, "crypt32!CertDeleteCRLFromStore ctx:0x{:x}", cert_ctx);
    let h = emu.regs().rcx;
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
}
