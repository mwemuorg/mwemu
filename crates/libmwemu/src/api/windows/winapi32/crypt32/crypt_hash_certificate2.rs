use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptHashCertificate2(emu: &mut emu::Emu) {
    let pwsz_hash_alg = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let dw_flags = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let pv_reserved = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let pb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let cb_encoded = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    let pb_computed_hash = emu.maps.read_dword(emu.regs().get_esp() + 20).unwrap_or(0) as u64;
    log_red!(emu, "crypt32!CryptHashCertificate2 0x{:x}", pwsz_hash_alg);
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 7);
}
