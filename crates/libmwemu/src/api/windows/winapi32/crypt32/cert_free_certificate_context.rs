use crate::emu;
use crate::winapi::winapi32::arg;
use crate::winapi::winapi32::stdcall_ret;

pub fn CertFreeCertificateContext(emu: &mut emu::Emu) {
    let cert_ctx = arg(emu, 0);

    log_red!(
        emu,
        "crypt32!CertFreeCertificateContext ctx: 0x{:x}",
        cert_ctx
    );

    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 1);
}
