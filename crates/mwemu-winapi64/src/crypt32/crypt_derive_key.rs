use crate::emu;
use crate::winapi::helper;

pub fn CryptDeriveKey(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let algid = emu.regs().rdx as u32;
    let h_hash = emu.regs().r8;
    let flags = emu.regs().r9;
    let h_key_ptr = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);
    log_red!(
        emu,
        "crypt32!CryptDeriveKey prov: 0x{:x} alg: 0x{:x} hash: 0x{:x}",
        h_prov,
        algid,
        h_hash
    );
    let h_key = helper::handler_create(&format!("derivedkey:0x{:x}", algid));
    if h_key_ptr != 0 {
        emu.maps.write_qword(h_key_ptr, h_key);
    }
    emu.regs_mut().rax = 1; // TRUE
}
