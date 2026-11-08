use crate::emu;

pub fn CryptVerifyCertificateSignatureEx(emu: &mut emu::Emu) {
    let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_subject_type = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pv_subject = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let dw_issuer_type = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pv_issuer = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptVerifyCertificateSignatureEx 0x{:x}",
        h_crypt_prov
    );
    let out_ptr = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    if out_ptr != 0 {
        emu.maps.write_dword(out_ptr, 0);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..7 {
        emu.stack_pop32(false);
    }
}
