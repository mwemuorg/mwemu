use crate::emu;
use crate::winapi::helper;

pub fn CryptHashCertificate(emu: &mut emu::Emu) {
    let alg_id = emu.regs().rdx;
    let cert_ptr = emu.regs().r8;
    let cert_len = emu.regs().r9;
    let hash_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    let hash_len_ptr = emu.maps.read_qword(emu.regs().rsp + 0x28).unwrap_or(0);
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
