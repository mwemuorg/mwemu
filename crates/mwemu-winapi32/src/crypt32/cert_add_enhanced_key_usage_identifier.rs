use crate::emu;
use crate::winapi::helper;

pub fn CertAddEnhancedKeyUsageIdentifier(emu: &mut emu::Emu) {
    let cert_ctx = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CertAddEnhancedKeyUsageIdentifier ctx:0x{:x}",
        cert_ctx
    );
    let handle = helper::handler_create("CertAddEnhancedKeyUsageIdentifier");
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
