use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptHashToBeSigned(emu: &mut emu::Emu) {
    let h_crypt_prov = WinApiAbi::arg(emu, 0);
    let dw_cert_encoding = WinApiAbi::arg(emu, 1);
    let pb_encoded = WinApiAbi::arg(emu, 2);
    let cb_encoded = WinApiAbi::arg(emu, 3);
    let pb_computed_hash = WinApiAbi::arg(emu, 4);
    log_red!(emu, "crypt32!CryptHashToBeSigned 0x{:x}", h_crypt_prov);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 6);
}
