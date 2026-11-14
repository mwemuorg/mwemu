use crate::emu;
use crate::winapi::helper;
use crate::winapi::helper::WinApiAbi;

pub(super) fn CryptCreateHash(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let algid = emu.regs().rdx as u32;
    let h_key = emu.regs().r8;
    let _flags = emu.regs().r9;
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

    log_red!(
        emu,
        "crypt32!CryptCreateHash prov: 0x{:x} alg: 0x{:x} ({}) key: 0x{:x}",
        h_prov,
        algid,
        alg_name,
        h_key
    );

    let h_hash = helper::handler_create(&format!("hash:{}", alg_name));
    if h_hash_ptr != 0 {
        emu.maps.write_qword(h_hash_ptr, h_hash);
    }
    emu.regs_mut().rax = 1; // TRUE
}
