use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CertEnumCertificatesInStore(emu: &mut emu::Emu) {
    let store = WinApiAbi::arg(emu, 0);
    let prev_ctx = WinApiAbi::arg(emu, 1);
    log_red!(
        emu,
        "crypt32!CertEnumCertificatesInStore store:0x{:x} prev:0x{:x}",
        store,
        prev_ctx
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 0;
    for _ in 0..2u32 {
        emu.stack_pop32(false);
    }
}
