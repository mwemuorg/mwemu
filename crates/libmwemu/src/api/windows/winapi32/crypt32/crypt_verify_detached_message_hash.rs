use crate::emu;
use crate::winapi::winapi32::stdcall_ret;

pub fn CryptVerifyDetachedMessageHash(emu: &mut emu::Emu) {
    let p_hash_para = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let pb_detached_hash_blob = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0) as u64;
    let cb_detached_hash_blob = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let c_to_be_hashed = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0) as u64;
    let rgb_pb_to_be_hashed = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    log_red!(
        emu,
        "crypt32!CryptVerifyDetachedMessageHash 0x{:x}",
        p_hash_para
    );
    emu.set_last_error(0);
    emu.regs_mut().rax = 1;
    stdcall_ret(emu, 8);
}
