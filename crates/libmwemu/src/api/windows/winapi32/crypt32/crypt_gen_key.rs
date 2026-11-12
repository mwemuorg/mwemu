use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let h_prov = WinApiAbi::arg(emu, 0);
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let flags = WinApiAbi::arg(emu, 2) as u32;
    let h_key_ptr = WinApiAbi::arg(emu, 3);
    for _ in 0..4u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptGenKey prov: 0x{:x} alg: 0x{:x} flags: 0x{:x}",
        h_prov,
        algid,
        flags
    );
    let h_key = helper::handler_create(&format!("cryptkey:0x{:x}", algid));
    let key_bits = flags >> 16;
    crypto_state::create_key(h_key, algid, key_bits);
    if h_key_ptr != 0 {
        emu.maps.write_dword(h_key_ptr, h_key as u32);
    }
    emu.regs_mut().rax = 1; // TRUE
}
