use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptDeriveKey(emu: &mut emu::Emu) {
    let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let h_hash = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let h_key_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptDeriveKey prov: 0x{:x} alg: 0x{:x} hash: 0x{:x}",
        h_prov,
        algid,
        h_hash
    );
    let h_key = helper::handler_create(&format!("derivedkey:0x{:x}", algid));
    crypto_state::derive_key_from_hash(h_key, h_hash, algid);
    if h_key_ptr != 0 {
        emu.maps.write_dword(h_key_ptr, h_key as u32);
    }
    emu.regs_mut().rax = 1; // TRUE
}
