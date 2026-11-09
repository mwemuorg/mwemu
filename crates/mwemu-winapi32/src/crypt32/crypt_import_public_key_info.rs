use crate::emu;
use crate::winapi::helper;

pub fn CryptImportPublicKeyInfo(emu: &mut emu::Emu) {
    let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let p_info = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let ph_key = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptImportPublicKeyInfo 0x{:x}", h_crypt_prov);
    let hndl = helper::handler_create("CryptImportPublicKeyInfo");
    if ph_key != 0 {
        emu.maps.write_dword(ph_key, hndl as u32);
    }
    emu.regs_mut().rax = 1;
    for _ in 0..4 {
        emu.stack_pop32(false);
    }
}
