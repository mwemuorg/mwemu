use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let h_prov = WinApiAbi::arg(emu, 0);
    let algid = WinApiAbi::arg(emu, 1) as u32;
    let h_key = WinApiAbi::arg(emu, 2);
    let _flags = WinApiAbi::arg(emu, 3) as u32;
    let h_hash_ptr = WinApiAbi::arg(emu, 4);
    let alg_name = match algid {
        0x8001 => "MD2",
        0x8002 => "MD4",
        0x8003 => "MD5",
        0x8004 => "SHA1",
        0x800C => "SHA256",
        0x800D => "SHA384",
        0x800E => "SHA512",
        _ => "UNKNOWN",
    };
    for _ in 0..5u32 {
        emu.stack_pop32(false);
    }
    log_red!(
        emu,
        "crypt32!CryptCreateHash prov: 0x{:x} alg: 0x{:x} ({}) key: 0x{:x}",
        h_prov,
        algid,
        alg_name,
        h_key
    );
    let h_hash = helper::handler_create(&format!("hash:{}", alg_name));
    crypto_state::create_hash(h_hash, algid);
    if h_hash_ptr != 0 {
        emu.maps.write_dword(h_hash_ptr, h_hash as u32);
    }
    emu.regs_mut().rax = 1; // TRUE
}
