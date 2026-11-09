use crate::api::windows::common::crypto_state;
use crate::emu;
use crate::winapi::helper;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let h_prov = emu.maps.read_dword(emu.regs().get_esp()).unwrap_or(0) as u64;
    let algid = emu.maps.read_dword(emu.regs().get_esp() + 4).unwrap_or(0);
    let h_key = emu.maps.read_dword(emu.regs().get_esp() + 8).unwrap_or(0) as u64;
    let _flags = emu.maps.read_dword(emu.regs().get_esp() + 12).unwrap_or(0);
    let h_hash_ptr = emu.maps.read_dword(emu.regs().get_esp() + 16).unwrap_or(0) as u64;
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
