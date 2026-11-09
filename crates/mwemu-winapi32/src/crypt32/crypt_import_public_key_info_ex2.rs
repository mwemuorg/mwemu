use crate::emu;

pub fn CryptImportPublicKeyInfoEx2(emu: &mut emu::Emu) {
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let p_info = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pv_aux_info = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let ph_key = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptImportPublicKeyInfoEx2 0x{:x}",
        dw_cert_encoding
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
