use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptDeriveKey(emu: &mut emu::Emu) {
    let h_prov = WinApiAbi::arg(emu, 0);
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let h_hash = WinApiAbi::arg(emu, 2);
    let _flags = WinApiAbi::arg(emu, 3) as u32;
    let h_key_ptr = WinApiAbi::arg(emu, 4);
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
