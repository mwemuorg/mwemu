use crate::emu;
use crate::winapi::helper::WinApiAbi;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptHashCertificate2(emu: &mut emu::Emu) {
    let pwsz_hash_alg = WinApiAbi::arg(emu, 0);
    let dw_flags = WinApiAbi::arg(emu, 1);
    let pv_reserved = WinApiAbi::arg(emu, 2);
    let pb_encoded = WinApiAbi::arg(emu, 3);
    let cb_encoded = WinApiAbi::arg(emu, 4);
    let pb_computed_hash = WinApiAbi::arg(emu, 5);
    log_red!(emu, "crypt32!CryptHashCertificate2 0x{:x}", pwsz_hash_alg);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
