use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptImportPublicKeyInfo(emu: &mut emu::Emu) {
    let h_crypt_prov = WinApiAbi::arg(emu, 0);
    let dw_cert_encoding = WinApiAbi::arg(emu, 1);
    let p_info = WinApiAbi::arg(emu, 2);
    let ph_key = WinApiAbi::arg(emu, 3);
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
