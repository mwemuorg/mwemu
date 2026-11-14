use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptVerifyCertificateSignatureEx(emu: &mut emu::Emu) {
    let h_crypt_prov = WinApiAbi::arg(emu, 0);
    let dw_cert_encoding = WinApiAbi::arg(emu, 1);
    let dw_subject_type = WinApiAbi::arg(emu, 2);
    let pv_subject = WinApiAbi::arg(emu, 3);
    let dw_issuer_type = WinApiAbi::arg(emu, 4);
    let pv_issuer = WinApiAbi::arg(emu, 5);
    let dw_flags = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "crypt32!CryptVerifyCertificateSignatureEx 0x{:x}",
        h_crypt_prov
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
