use crate::emu;
use crate::winapi::helper;

pub fn CryptGenKey(emu: &mut emu::Emu) {
    let h_prov = emu.regs().rcx;
    let algid = emu.regs().rdx as u32;
    let flags = emu.regs().r8;
    let h_key_ptr = emu.regs().r9;
    log_red!(
        emu,
        "crypt32!CryptGenKey prov: 0x{:x} alg: 0x{:x} flags: 0x{:x}",
        h_prov,
        algid,
        flags
    );
    let h_key = helper::handler_create(&format!("cryptkey:0x{:x}", algid));
    if h_key_ptr != 0 {
        emu.maps.write_qword(h_key_ptr, h_key);
    }
    emu.regs_mut().rax = 1; // TRUE
}
