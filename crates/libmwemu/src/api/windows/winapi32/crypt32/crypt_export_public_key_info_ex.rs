use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptExportPublicKeyInfoEx(emu: &mut emu::Emu) {
    let h_crypt_prov = WinApiAbi::arg(emu, 0);
    let dw_key_spec = WinApiAbi::arg(emu, 1);
    let dw_cert_encoding = WinApiAbi::arg(emu, 2);
    let psz_public_key_obj_id = WinApiAbi::arg(emu, 3);
    let dw_flags = WinApiAbi::arg(emu, 4);
    let pv_aux_info = WinApiAbi::arg(emu, 5);
    let p_info = WinApiAbi::arg(emu, 6);
    log_red!(
        emu,
        "crypt32!CryptExportPublicKeyInfoEx 0x{:x}",
        h_crypt_prov
    );
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
