
use crate::windows::constants::crypto::*;
use crate::emu;
use crate::winapi::helper;

pub fn CryptCreateHash(emu: &mut emu::Emu) {
    let hprov = emu.regs().rcx;
    let algid = emu.regs().rdx as u32;
    let hkey = emu.regs().r8;
    let flags = emu.regs().r9 as u32;
    let ptr_hash = emu.maps.read_qword(emu.regs().rsp + 0x20).unwrap_or(0);

    let alg_name = get_cryptoalgorithm_name(algid);

    log_red!(
        emu,
        "kernel32!CryptCreateHash prov: 0x{:x} alg:{} key: 0x{:x} flags: 0x{:x}",
        hprov,
        alg_name,
        hkey,
        flags
    );

    if ptr_hash != 0 {
        emu.maps.write_qword(
            ptr_hash,
            helper::handler_create(&format!("alg://{}", alg_name)),
        );
    }
    emu.regs_mut().rax = 1;
}
