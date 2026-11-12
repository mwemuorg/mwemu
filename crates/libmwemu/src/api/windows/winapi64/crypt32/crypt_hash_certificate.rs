use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptHashCertificate(emu: &mut emu::Emu) {
    let alg_id = emu.regs().rdx;
    let cert_ptr = emu.regs().r8;
    let cert_len = emu.regs().r9;
    let hash_ptr = WinApiAbi::arg(emu, 4);
    let hash_len_ptr = WinApiAbi::arg(emu, 5);
    helper::advance_tick(emu, 1);
    emu.set_last_error(0);
    emu.regs_mut().rax = crate::api::windows::common::crypt32::crypt_hash_certificate(
        emu,
        alg_id as u32,
        cert_ptr,
        cert_len,
        hash_ptr,
        hash_len_ptr,
    );
}
