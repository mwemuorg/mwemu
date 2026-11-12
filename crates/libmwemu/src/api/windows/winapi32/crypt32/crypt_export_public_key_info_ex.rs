use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptExportPublicKeyInfoEx(emu: &mut emu::Emu) {
    let h_crypt_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_key_spec = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let dw_cert_encoding = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let psz_public_key_obj_id = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pv_aux_info = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    let p_info = emu.maps.read_dword(emu.regs().get_esp() + 24).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptExportPublicKeyInfoEx 0x{:x}",
        h_crypt_prov
    );
    let val_ptr = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
