use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptSignCertificate(emu: &mut emu::Emu) {
    let h_crypt_prov = WinApiAbi::arg(emu, 0);
    let dw_key_spec = WinApiAbi::arg(emu, 1);
    let dw_cert_encoding = WinApiAbi::arg(emu, 2);
    let pb_encoded_to_be_signed = WinApiAbi::arg(emu, 3);
    let cb_encoded_to_be_signed = WinApiAbi::arg(emu, 4);
    let p_sign_algorithm = WinApiAbi::arg(emu, 5);
    let pv_hash_aux_info = WinApiAbi::arg(emu, 6);
    let pb_signature = WinApiAbi::arg(emu, 7);
    log_red!(emu, "crypt32!CryptSignCertificate 0x{:x}", h_crypt_prov);
    let val_ptr = WinApiAbi::arg(emu, 1);
    if val_ptr != 0 && emu.maps.is_mapped(val_ptr) {
        let _val = emu.maps.read_string(val_ptr);
    }
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 9);
}
