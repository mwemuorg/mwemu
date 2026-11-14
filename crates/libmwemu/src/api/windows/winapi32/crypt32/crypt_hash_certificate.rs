use crate::emu;
use crate::winapi::helper::WinApiAbi;

pub fn CryptHashCertificate(emu: &mut emu::Emu) {
    let alg_id = WinApiAbi::arg(emu, 1) as u32;
    let cert_ptr = WinApiAbi::arg(emu, 2);
    let cert_len = WinApiAbi::arg(emu, 3);
    let hash_ptr = WinApiAbi::arg(emu, 4);
    let hash_len_ptr = WinApiAbi::arg(emu, 5);
    for _ in 0..7u32 {
        emu.stack_pop32(false);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
        emu,
        alg_id,
        cert_ptr,
        cert_len,
        hash_ptr,
        hash_len_ptr,
    );
}
