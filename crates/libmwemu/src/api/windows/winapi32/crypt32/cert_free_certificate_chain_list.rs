use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CertFreeCertificateChainList(emu: &mut emu::Emu) {
    let cert_ctx = WinApiAbi::arg(emu, 0);
    log_red!(
        emu,
        "crypt32!CertFreeCertificateChainList ctx:0x{:x}",
        cert_ctx
    );
    let h = WinApiAbi::arg(emu, 0);
    if helper::handler_exist(h) {
        helper::handler_close(h);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..1 {
        emu.stack_pop32(false);
    }
}
