use crate::emu;
use crate::winapi::helper;

pub fn PFXImportCertStore(emu: &mut emu::Emu) {
    let _p_pfx = emu.regs().rcx;
    let _sz_password = emu.regs().rdx;
    let _dw_flags = emu.regs().r8;
    let handle = helper::handler_create("pfx_cert_store");
    log_red!(emu, "crypt32!PFXImportCertStore => 0x{:x}", handle);
    emu.set_last_error(0);
    emu.regs_mut().rax = handle;
}
