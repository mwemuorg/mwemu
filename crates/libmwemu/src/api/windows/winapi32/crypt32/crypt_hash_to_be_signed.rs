use crate::emu;

pub fn CryptHashToBeSigned(emu: &mut emu::Emu) {
    let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let pb_computed_hash = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptHashToBeSigned 0x{:x}", h_crypt_prov);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    for _ in 0..5 {
        emu.stack_pop32(false);
    }
}
