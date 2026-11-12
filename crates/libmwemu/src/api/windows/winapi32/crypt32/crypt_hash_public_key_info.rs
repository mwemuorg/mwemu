use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptHashPublicKeyInfo(emu: &mut emu::Emu) {
    let h_crypt_prov = WinApiAbi::arg(emu, 0);
    let algid = WinApiAbi::arg(emu, 1);
    let dw_flags = WinApiAbi::arg(emu, 2);
    let dw_cert_encoding = WinApiAbi::arg(emu, 3);
    let p_info = WinApiAbi::arg(emu, 4);
    let pb_computed_hash = WinApiAbi::arg(emu, 5);
    log_red!(emu, "crypt32!CryptHashPublicKeyInfo 0x{:x}", h_crypt_prov);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
