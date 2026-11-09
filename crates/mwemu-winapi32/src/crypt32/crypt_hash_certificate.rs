use crate::emu;

pub fn CryptHashCertificate(emu: &mut emu::Emu) {
    let alg_id = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let cert_ptr = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cert_len = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let hash_len_ptr = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
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
